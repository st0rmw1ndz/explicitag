use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use clap::{ColorChoice, Parser};
use env_logger::Env;
use eyre::eyre;
use log::LevelFilter;
use mp4ameta::{AdvisoryRating, Tag};
use reqwest::header::{self, HeaderMap, HeaderValue};
use reqwest::Client;
use serde::Deserialize;
use walkdir::WalkDir;

const LYRICS_API_URL: &str = "https://lyrist.vercel.app/api";
const CLIENT_USER_AGENT: &str = "explicitag, https://github.com/st0rm1wndz/explicitag";

static MARK_CLEAN: AtomicBool = AtomicBool::new(false);
static USE_API: AtomicBool = AtomicBool::new(false);
static WRITE_LYRICS: AtomicBool = AtomicBool::new(false);
static NO_WRITE: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Deserialize)]
struct LyristResponse {
    lyrics: Option<String>,
}

enum LyricsSource {
    Api,
    Local,
}

#[derive(Debug, Parser)]
#[command(
    about,
    long_about = None,
    after_help = env!("CARGO_PKG_REPOSITORY"),
    version,
    color = ColorChoice::Never,
)]
struct Cli {
    /// Files or directories to process
    #[arg(required = true)]
    paths: Vec<PathBuf>,

    /// Mark safe tracks as specifically clean
    #[arg(short = 'c', long, default_value_t = false)]
    mark_clean: bool,

    /// Search API for lyrics if local ones aren't present
    #[arg(short = 'a', long, default_value_t = false)]
    use_api: bool,

    /// Write lyrics from the API to files
    #[arg(short = 'w', long, default_value_t = true, requires = "use_api")]
    write_lyrics: bool,

    /// Disable writing lyrics and ratings to files
    #[arg(
        short = 'n',
        long,
        default_value_t = false,
        conflicts_with = "write_lyrics"
    )]
    no_write: bool,

    /// Suppress all output from the program
    #[arg(short = 'q', long, default_value_t = false)]
    quiet: bool,
}

async fn search_lyrist(artist: &str, title: &str, client: &Client) -> eyre::Result<String> {
    let url = format!("{}/{}/{}/", LYRICS_API_URL, artist, title);

    let response = client
        .get(url)
        .send()
        .await
        .map_err(|_| eyre!("Unable to access lyrics API"))?;

    let text = response.text().await?;
    let lyrist_response: LyristResponse = serde_json::from_str(&text)?;

    lyrist_response
        .lyrics
        .map(|lyrics| lyrics.to_lowercase())
        .ok_or_else(|| eyre!("No track was found via API"))
}

async fn get_track_lyrics(tag: &Tag, client: &Client) -> eyre::Result<(String, LyricsSource)> {
    if let Some(lyrics) = tag.lyrics() {
        return Ok((lyrics.to_lowercase(), LyricsSource::Local));
    }

    if !USE_API.load(Ordering::Relaxed) {
        return Err(eyre!("No lyrics found"));
    }

    let artist = tag
        .artist()
        .ok_or_else(|| eyre!("Artist tag is required"))?;
    let title = tag.title().ok_or_else(|| eyre!("Title tag is required"))?;

    search_lyrist(artist, title, client)
        .await
        .map(|lyrics| (lyrics, LyricsSource::Api))
}

fn get_lyrics_rating(lyrics: String, explicit_words: &[&str]) -> AdvisoryRating {
    match explicit_words
        .iter()
        .any(|word| lyrics.contains(&word.trim().to_lowercase()))
    {
        true => AdvisoryRating::Explicit,
        false => match MARK_CLEAN.load(Ordering::Relaxed) {
            true => AdvisoryRating::Clean,
            false => AdvisoryRating::Inoffensive,
        },
    }
}

async fn process_file(
    path: &Path,
    explicit_words: &[&str],
    client: &Client,
) -> eyre::Result<String> {
    let mut tag = Tag::read_from_path(path).map_err(|_| eyre!("Invalid MP4 file"))?;

    let (lyrics, lyrics_source) = get_track_lyrics(&tag, client).await?;
    let source_string = match lyrics_source {
        LyricsSource::Api => "API",
        LyricsSource::Local => "Local",
    };

    if let LyricsSource::Api = lyrics_source {
        if WRITE_LYRICS.load(Ordering::Relaxed) && !NO_WRITE.load(Ordering::Relaxed) {
            tag.set_lyrics(&lyrics);
        }
    }

    let new_rating = get_lyrics_rating(lyrics, explicit_words);

    if !NO_WRITE.load(Ordering::Relaxed) {
        tag.set_advisory_rating(new_rating);
        tag.write_to_path(path)?;
    }

    Ok(format!("({}) {}", source_string, new_rating))
}

async fn create_task(path: PathBuf, explicit_words: Arc<Vec<&str>>, client: Arc<Client>) -> bool {
    let result = process_file(&path, &explicit_words, &client).await;
    match &result {
        Ok(message) => log::info!("{} - {}", path.display(), message),
        Err(error) => log::warn!("{} - {}", path.display(), error),
    }

    result.is_ok()
}

async fn process_path_entries(
    entries: impl Iterator<Item = walkdir::DirEntry>,
    explicit_words: Arc<Vec<&'static str>>,
    client: Arc<Client>,
) -> (i32, i32) {
    let mut processed_files = 0;
    let mut skipped_files = 0;

    let tasks: Vec<_> = entries
        .map(|entry| {
            let path = entry.path().to_path_buf();
            let explicit_words = Arc::clone(&explicit_words);
            let client = Arc::clone(&client);

            processed_files += 1;
            tokio::spawn(async move { create_task(path, explicit_words, client).await })
        })
        .collect();

    for task in futures::future::join_all(tasks).await {
        if !(task.unwrap_or(false)) {
            skipped_files += 1;
        }
    }

    (processed_files, skipped_files)
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let args = Cli::parse();

    MARK_CLEAN.store(args.mark_clean, Ordering::Relaxed);
    USE_API.store(args.use_api, Ordering::Relaxed);
    WRITE_LYRICS.store(args.write_lyrics, Ordering::Relaxed);
    NO_WRITE.store(args.write_lyrics, Ordering::Relaxed);

    if args.quiet {
        log::set_max_level(LevelFilter::Off);
    } else {
        log::set_max_level(LevelFilter::Info);
    }

    let mut headers = HeaderMap::new();
    headers.insert(
        header::USER_AGENT,
        HeaderValue::from_str(CLIENT_USER_AGENT)?,
    );

    let explicit_words: Arc<Vec<&str>> = Arc::new(include_str!("words.txt").split('\n').collect());
    let client = Arc::new(Client::builder().default_headers(headers).build()?);

    let entries = args.paths.into_iter().flat_map(|path| {
        WalkDir::new(path)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
    });

    log::info!(
        "Running with {} words marked as explicit.",
        explicit_words.len()
    );

    let (processed_files, skipped_files) =
        process_path_entries(entries, explicit_words, client).await;

    log::info!(
        "Processed {} files. Skipped {} files.",
        processed_files,
        skipped_files
    );

    Ok(())
}
