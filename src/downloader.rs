use crate::config::{DownloaderConfig, MessagesConfig, build_style_options};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::path::PathBuf;
use std::process::exit;
use trauma::download::{Download, Status};
use trauma::downloader::Downloader;

pub async fn run_downloads(
    links: Vec<String>,
    filenames: Vec<String>,
    output: PathBuf,
    concurrents: usize,
    dl_config: DownloaderConfig,
    msg_config: Option<MessagesConfig>,
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

    let retries = dl_config.retries.unwrap_or(3);
    let resumable = dl_config.resumable.unwrap_or(true);
    let style_opts = build_style_options(&dl_config.progress_bar);

    let mut header_map = HeaderMap::new();
    if let Some(headers) = &dl_config.headers {
        for (key, value) in headers {
            if let (Ok(h_name), Ok(h_val)) = (
                HeaderName::from_bytes(key.as_bytes()),
                HeaderValue::from_str(value),
            ) {
                header_map.insert(h_name, h_val);
            }
        }
    }

    let downloader = Downloader::builder()
        .directory(output)
        .concurrent_downloads(concurrents)
        .retries(retries)
        .resumable(resumable)
        .style_options(style_opts)
        .headers(header_map)
        .build();

    let summaries = downloader.download(&downloads).await;

    let msgs = msg_config.unwrap_or_default();
    let tpl_success = msgs.success.as_deref().unwrap_or("✓ Успешно: {url}");
    let tpl_error = msgs
        .error
        .as_deref()
        .unwrap_or("✗ Ошибка загрузки {url}: {error}");
    let tpl_skipped = msgs
        .skipped
        .as_deref()
        .unwrap_or("⊘ Пропущено {url}: {reason}");
    let tpl_not_started = msgs
        .not_started
        .as_deref()
        .unwrap_or("✗ Не запущено: {url}");

    let mut has_error = false;
    for summary in summaries {
        let summary_url = summary.download().url();
        let url = summary_url.as_str();
        match summary.status() {
            Status::Success => {
                println!("{}", tpl_success.replace("{url}", url));
            }
            Status::Fail(err) => {
                eprintln!(
                    "{}",
                    tpl_error.replace("{url}", url).replace("{error}", err)
                );
                has_error = true;
            }
            Status::Skipped(reason) => {
                eprintln!(
                    "{}",
                    tpl_skipped
                        .replace("{url}", url)
                        .replace("{reason}", reason)
                );
            }
            Status::NotStarted => {
                eprintln!("{}", tpl_not_started.replace("{url}", url));
                has_error = true;
            }
        }
    }

    if has_error {
        exit(3);
    }
}
