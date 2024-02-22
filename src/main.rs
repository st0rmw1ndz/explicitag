use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;

use clap::{ColorChoice, Parser};
use eyre::eyre;
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
static QUIET: AtomicBool = AtomicBool::new(false);

static PROCESSED_FILES: AtomicUsize = AtomicUsize::new(0);
static SKIPPED_FILES: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Parser)]
#[command(about, long_about = None, after_help = env!("CARGO_PKG_REPOSITORY"), version, color = ColorChoice::Never)]
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

    /// Write the lyrics from the API to the file
    #[arg(short = 'w', long, default_value_t = true, requires = "use_api")]
    write_lyrics: bool,

    /// Suppress all output from the program
    #[arg(short = 'q', long, default_value_t = false)]
    quiet: bool,
}

#[derive(Debug, Deserialize)]
struct LyristResponse {
    lyrics: Option<String>,
}

enum LyricsSource {
    Local,
    API,
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

    match lyrist_response.lyrics {
        Some(lyrics) => Ok(lyrics.to_lowercase()),
        None => Err(eyre!("No track was found via API")),
    }
}

async fn get_track_lyrics(tag: &Tag, client: &Client) -> eyre::Result<(String, LyricsSource)> {
    if let Some(lyrics) = tag.lyrics() {
        return Ok((lyrics.to_lowercase(), LyricsSource::Local));
    }

    if USE_API.load(Ordering::Relaxed) {
        if let (Some(artist), Some(title)) = (tag.artist(), tag.title()) {
            match search_lyrist(artist, title, client).await {
                Ok(lyrics) => return Ok((lyrics, LyricsSource::API)),
                Err(error) => return Err(error),
            }
        } else {
            return Err(eyre!("Artist and title are required"));
        }
    }

    Err(eyre!("No lyrics found"))
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

async fn process_file(path: &Path, explicit_words: &[&str], client: &Client) -> eyre::Result<()> {
    let mut tag = Tag::read_from_path(path)?;

    match get_track_lyrics(&tag, client).await {
        Ok((lyrics, lyrics_source)) => {
            if let LyricsSource::API = lyrics_source {
                if WRITE_LYRICS.load(Ordering::Relaxed) {
                    tag.set_lyrics(&lyrics);
                }
            }

            let new_rating = get_lyrics_rating(lyrics, explicit_words);

            if tag.advisory_rating() != Some(new_rating) {
                tag.set_advisory_rating(new_rating);
                tag.write_to_path(path)?;

                if !QUIET.load(Ordering::Relaxed) {
                    let source = match lyrics_source {
                        LyricsSource::API => "API",
                        LyricsSource::Local => "Local",
                    };
                    println!("{} - ({}) {}", path.display(), source, new_rating);
                }

                return Ok(());
            }

            if !QUIET.load(Ordering::Relaxed) {
                println!("{} - Rating is the same as current", path.display());
            }
            SKIPPED_FILES.fetch_add(1, Ordering::Relaxed);
        }
        Err(error) => {
            if !QUIET.load(Ordering::Relaxed) {
                println!("{} - {}", path.display(), error);
            }
            SKIPPED_FILES.fetch_add(1, Ordering::Relaxed);
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let args = Cli::parse();

    MARK_CLEAN.store(args.mark_clean, Ordering::Relaxed);
    USE_API.store(args.use_api, Ordering::Relaxed);
    WRITE_LYRICS.store(args.write_lyrics, Ordering::Relaxed);
    QUIET.store(args.quiet, Ordering::Relaxed);

    let explicit_words: Arc<Vec<&str>> = Arc::new(include_str!("words.txt").split("\n").collect());

    let mut headers = HeaderMap::new();
    headers.insert(
        header::USER_AGENT,
        HeaderValue::from_str(CLIENT_USER_AGENT)?,
    );
    let client: Arc<Client> = Arc::new(Client::builder().default_headers(headers).build()?);

    if !QUIET.load(Ordering::Relaxed) {
        println!(
            "Running with {} words marked as explicit.",
            explicit_words.len()
        );
    }

    let mut tasks = vec![];

    for path in args.paths {
        for entry in WalkDir::new(path)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path().to_path_buf();
            let explicit_words = Arc::clone(&explicit_words);
            let client = Arc::clone(&client);
            let task = tokio::spawn(async move {
                PROCESSED_FILES.fetch_add(1, Ordering::Relaxed);
                process_file(&path, &explicit_words, &client).await
            });
            tasks.push(task);
        }
    }

    let results = futures::future::try_join_all(tasks).await;
    results?;

    if !QUIET.load(Ordering::Relaxed) {
        println!(
            "Processed {} files. Skipped {} files.",
            PROCESSED_FILES.load(Ordering::Relaxed),
            SKIPPED_FILES.load(Ordering::Relaxed)
        );
    }

    Ok(())
}
