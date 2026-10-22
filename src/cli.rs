use clap::Parser;

#[derive(Parser, Clone, Debug)]
#[command(about = "A simple cli downloader")]
pub struct Args {
    #[arg(required = false)]
    pub links: Vec<String>,

    #[arg(short = 'c', long)]
    pub concurrents: Option<usize>,

    #[arg(short = 'F', long)]
    pub downloads_file: Option<String>,

    #[arg(short = 'o', long)]
    pub output_dir: Option<String>,

    #[arg(short = 'f', long, value_delimiter = ',')]
    pub file_names: Vec<String>,
}
