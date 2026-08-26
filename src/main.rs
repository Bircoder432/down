use std::{env, process::exit};

use clap::{CommandFactory, Parser};
use tokio::fs;
use trauma::{download::Download, downloader::Downloader};

#[derive(Parser, Clone, Debug)]
#[command(about = "A simple cli downloader")]
struct Args {
    links: Vec<String>,
    #[arg(short = 'c', long)]
    concurrents: Option<usize>,
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
    let concurents = args.concurrents.unwrap_or(1);
    if let Some(file) = args.downloads_file {
        parse_file(&file, &mut links, &mut filenames).await;
    }
    let has_filenames = filenames.len() != 0;
    if links.is_empty() {
        Args::command().print_help().unwrap();
        eprintln!("there must be more than 0 links");
        exit(1);
    }
    if has_filenames && filenames.len() != links.len() {
        Args::command().print_help().unwrap();
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
    let downloader = Downloader::builder()
        .directory(output)
        .concurrent_downloads(concurents)
        .build();
    downloader.download(&downloads).await;
}

async fn parse_file(path: &str, links: &mut Vec<String>, filenames: &mut Vec<String>) {
    let content = fs::read_to_string(path).await.unwrap();

    let lines: Vec<&str> = content.split("\n").collect();
    for line in lines {
        let mut parts: Vec<&str> = line.split(" ").collect();
        parts = parts
            .iter()
            .filter_map(|p| if *p == "" { None } else { Some(*p) })
            .collect();
        if parts.len() == 0 {
            continue;
        }
        if parts.len() == 1 {
            links.push(parts[0].trim().to_string());
        }
        if parts.len() == 2 {
            links.push(parts[0].trim().to_string());
            filenames.push(parts[1].trim().to_string());
        }
    }
}
