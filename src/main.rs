use clap::{ColorChoice, Parser};
use mp4ameta::{AdvisoryRating, Tag};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, after_help = env!("CARGO_PKG_REPOSITORY"), color = ColorChoice::Never)]
struct Cli {
    /// Files or directories to process
    #[arg(required = true)]
    paths: Vec<PathBuf>,
}

async fn process_file(path: &Path, explicit_words: &[&str]) -> eyre::Result<()> {
    let mut tag = Tag::read_from_path(path)?;

    let lyrics = match tag.lyrics() {
        Some(lyrics) => lyrics.to_lowercase(),
        None => {
            println!("File: {} - No lyrics found", path.display());
            return Ok(());
        }
    };

    let rating = match explicit_words
        .iter()
        .any(|word| lyrics.contains(&word.trim().to_lowercase()))
    {
        true => AdvisoryRating::Explicit,
        false => AdvisoryRating::Inoffensive,
    };

    tag.set_advisory_rating(rating);

    println!("File: {} - {}", path.display(), rating);

    Ok(())
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let args = Cli::parse();

    let explicit_words: Vec<&str> = include_str!("words.txt").split("\n").collect();

    let mut tasks = vec![];

    for path in args.paths {
        for entry in WalkDir::new(path)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path().to_path_buf();
            let explicit_words = explicit_words.clone();
            let task = tokio::spawn(async move { process_file(&path, &explicit_words).await });
            tasks.push(task);
        }
    }

    for task in tasks {
        task.await??;
    }

    Ok(())
}
