use std::{env, process::exit};

use clap::Parser;
use trauma::{download::Download, downloader::Downloader};

#[derive(Parser, Clone, Debug)]
struct Args {
    links: Vec<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let links = args.links;
    if links.is_empty() {
        eprintln!("Need >0 links");
        exit(1);
    }
    let mut downloads: Vec<Download> = Vec::new();
    for l in links {
        downloads.push(Download::builder().url(l).unwrap().build());
    }
    let downloader = Downloader::builder()
        .directory(env::current_dir().unwrap())
        .build();
    downloader.download(&downloads).await;
}
