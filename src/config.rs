use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;
use tokio::fs;
use trauma::downloader::{ProgressBarOpts, StyleOptions};

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Config {
    pub downloader: Option<DownloaderConfig>,
    pub messages: Option<MessagesConfig>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct DownloaderConfig {
    pub concurrent_downloads: Option<usize>,
    pub retries: Option<u32>,
    pub resumable: Option<bool>,
    pub progress_bar: Option<ProgressBarConfig>,
    pub headers: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct ProgressBarConfig {
    pub hidden: Option<bool>,
    pub template: Option<String>,
    pub progress_chars: Option<String>,
    pub clear_on_finish: Option<bool>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct MessagesConfig {
    pub success: Option<String>,
    pub error: Option<String>,
    pub skipped: Option<String>,
    pub not_started: Option<String>,
}

impl Config {
    pub async fn load(path: &Path) -> Option<Self> {
        if let Ok(content) = fs::read_to_string(path).await {
            toml::from_str(&content).ok()
        } else {
            None
        }
    }
}

fn build_progress_bar_opts(config: &ProgressBarConfig) -> ProgressBarOpts {
    if config.hidden == Some(true) {
        return ProgressBarOpts::hidden();
    }

    let clear = config.clear_on_finish.filter(|&b| b).map(|_| ());

    ProgressBarOpts::builder()
        .maybe_template(config.template.clone())
        .maybe_progress_chars(config.progress_chars.clone())
        .maybe_clear(clear)
        .build()
}

pub fn build_style_options(config: &Option<ProgressBarConfig>) -> StyleOptions {
    let mut style_opts = StyleOptions::default();

    if let Some(pb_config) = config {
        if pb_config.hidden == Some(true) {
            return StyleOptions::hidden();
        }

        let opts = build_progress_bar_opts(pb_config);
        style_opts.set_main(opts.clone());
        style_opts.set_child(opts);
    }

    style_opts
}
