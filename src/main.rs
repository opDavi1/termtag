use std::path::Path;

mod formats;
mod metadata;

fn main() {
    let file = formats::load_file(Path::new("./test.flac")).unwrap();
    let _ = file.read_metadata().unwrap();
}
