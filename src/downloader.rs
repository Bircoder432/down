use std::path::PathBuf;
use trauma::{download::Download, downloader::Downloader};

pub async fn run_downloads(
    links: Vec<String>,
    filenames: Vec<String>,
    output: PathBuf,
    concurrents: usize,
) {
    let mut downloads: Vec<Download> = Vec::new();
    let has_filenames = !filenames.is_empty();
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
        .concurrent_downloads(concurrents)
        .build();

    downloader.download(&downloads).await;
}
