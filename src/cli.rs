// This file is part of termtag licensed under the GPL-3.0-or-later license.
// See the included LICENSE file or go to <https://www.gnu.org/licenses/> for more information.

use clap::Parser;

#[derive(Parser)]
#[command(name = "termtag")]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Path of the audio file to edit
    pub path: std::path::PathBuf,

    /// Show the current metadata of the audio file
    #[arg(short, long)]
    pub list: bool,

    /// Edit the metadata of a given audio file. Use with -a, -A, -s, etc.
    #[arg(short, long)]
    pub edit: bool,

    /// Set the album
    #[arg(short='A', long)]
    pub album: Option<String>,

    /// Set the primary artist(s)
    #[arg(short, long, num_args=1.., value_delimiter = ' ')]
    pub artist: Option<Vec<String>>,

    /// Set the comment
    #[arg(short, long)]
    pub comment: Option<String>,

    /// Set the disk number
    #[arg(short, long)]
    pub disk_number: Option<u32>,

    /// Set the title of the song
    #[arg(short, long)]
    pub title: Option<String>,

    /// Set the track number.
    #[arg(short='n', long="number")]
    pub track_number: Option<u32>,

}
