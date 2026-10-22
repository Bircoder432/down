mod cli;
mod config;
mod downloader;
mod parser;

use std::path::PathBuf;
use std::process::exit;

use clap::{CommandFactory, Parser};

use cli::Args;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let config_path = dirs::config_dir()
        .map(|p| p.join("down").join("config.toml"))
        .unwrap_or_else(|| PathBuf::from("config.toml"));

    let config = config::Config::load(&config_path).await.unwrap_or_default();

    let dl_config = config.downloader.unwrap_or_default();
    let msg_config = config.messages;

    let mut links = args.links;
    let mut filenames = args.file_names;

    let concurrents = args
        .concurrents
        .or(dl_config.concurrent_downloads)
        .unwrap_or(1);

    let output = match args.output_dir {
        Some(dir) => PathBuf::from(dir),
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

    downloader::run_downloads(links, filenames, output, concurrents, dl_config, msg_config).await;
}
