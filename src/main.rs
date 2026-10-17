mod cli;
mod downloader;
mod parser;

use std::process::exit;

use clap::{CommandFactory, Parser};

use cli::Args;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let mut links = args.links;
    let mut filenames = args.file_names;
    let concurrents = args.concurrents;
    let output = match args.output_dir {
        Some(dir) => std::path::PathBuf::from(dir),
        None => std::env::current_dir().unwrap_or_default(),
    };

    if let Some(file) = args.downloads_file {
        parser::parse_file(&file, &mut links, &mut filenames).await;
    }

    let has_filenames = !filenames.is_empty();

    if links.is_empty() {
        Args::command().print_help().unwrap();
        eprintln!("there must be more than 0 links");
        exit(1);
    }

    if has_filenames && filenames.len() != links.len() {
        Args::command().print_help().unwrap();
        eprintln!("There should be as many filenames as there are links, or 0");
        exit(2);
    }

    downloader::run_downloads(links, filenames, output, concurrents).await;
}
