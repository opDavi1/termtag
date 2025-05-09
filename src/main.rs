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
