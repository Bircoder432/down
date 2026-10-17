use clap::Parser;

#[derive(Parser, Clone, Debug)]
#[command(about = "A simple cli downloader")]
pub struct Args {
    /// List of URLs to download
    pub links: Vec<String>,
    /// Number of concurrent downloads
    #[arg(short = 'c', long, default_value = "1")]
    pub concurrents: usize,

    /// Path to a file containing URLs and optional filenames
    #[arg(short = 'F', long)]
    pub downloads_file: Option<String>,

    /// Output directory for downloaded files
    #[arg(short = 'o', long)]
    pub output_dir: Option<String>,

    /// Custom filenames for the downloaded files, comma-separated
    #[arg(short = 'f', long, value_delimiter = ',')]
    pub file_names: Vec<String>,
}
