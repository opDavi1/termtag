use clap::Parser;

#[derive(Parser)]
#[command(name = "termtag")]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Path of the audio file to edit
    // #[arg(short, long)]
    pub path: std::path::PathBuf,
}