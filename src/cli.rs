// This file is part of termtag licensed under the GPL-3.0-or-later license.
// See the included LICENSE file or go to <https://www.gnu.org/licenses/> for more information.

use clap::Parser;

#[derive(Parser)]
#[command(name = "termtag")]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Path of the audio file to edit
    // #[arg(short, long)]
    pub path: std::path::PathBuf,
}
