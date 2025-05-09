use std::path::Path;

mod flac;

fn main() {
    let _ = flac::get_metadata(Path::new("./test.flac"));
}
