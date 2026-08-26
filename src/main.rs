use std::{env, process::exit};

use clap::Parser;
use tokio::fs;
use trauma::{download::Download, downloader::Downloader};

#[derive(Parser, Clone, Debug)]
struct Args {
    links: Vec<String>,
    #[arg(short = 'F', long)]
    downloads_file: Option<String>,
    #[arg(short = 'o', long)]
    output_dir: Option<String>,
    #[arg(short = 'f', long, value_delimiter = ',')]
    file_names: Vec<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let mut links = args.links;
    let output = args.output_dir.unwrap_or(
        env::current_dir()
            .unwrap_or_default()
            .into_string()
            .unwrap(),
    );
    let mut filenames = args.file_names;

    if let Some(file) = args.downloads_file {
        let content = fs::read_to_string(file).await.unwrap();

        let lines: Vec<&str> = content.split("\n").collect();
        for line in lines {
            let parts: Vec<&str> = line.split(" ").collect();
            if parts.len() == 0 {
                continue;
            }
            if parts.len() == 1 {
                if parts[0] == "" {
                    continue;
                }
                links.push(parts[0].trim().to_string());
            }
            if parts.len() == 2 {
                links.push(parts[0].trim().to_string());
                filenames.push(parts[1].trim().to_string());
            }
        }
    }
    let has_filenames = filenames.len() != 0;
    if links.is_empty() {
        eprintln!("there must be more than 0 links");
        exit(1);
    }
    if has_filenames && filenames.len() != links.len() {
        println!("There should be as many filenames as there are links, or 0");
        exit(2);
    }
    let mut downloads: Vec<Download> = Vec::new();
    if !has_filenames {
        for l in links {
            downloads.push(Download::builder().url(l).unwrap().build());
        }
    } else {
        for (l, n) in links.iter().zip(filenames) {
            downloads.push(
                Download::builder()
                    .url(l)
                    .unwrap()
                    .filename_override(n)
                    .build(),
            );
        }
    }
    let downloader = Downloader::builder().directory(output).build();
    downloader.download(&downloads).await;
}
