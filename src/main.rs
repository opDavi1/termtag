// This file is part of termtag by Davis Sherman <davissherman2007@gmail.com> licensed under the GPL-3.0-or-later license.
// See the included LICENSE file or go to <https://www.gnu.org/licenses/> for more information.

use clap::Parser;
use std::{env, path::Path};

mod cli;
mod formats;
mod metadata;

fn main() {
    let cli = cli::Cli::parse();
    let path = cli.path;

    let file = match formats::load_file(&path) {
        Some(f) => f,
        None => {
            eprintln!("Error: File type not supported.");
            return;
        }
    };
    let metadata = match file.read_metadata() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };
}
