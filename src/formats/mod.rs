mod flac;

use crate::metadata::AudioMetadata;
use std::path::Path;

pub fn load_file(path: &Path) -> Option<Box<dyn AudioMetadata>> {
    match path.extension()?.to_str()? {
        "flac" => Some(Box::new(flac::FlacFile::new(path))),
        "mp3" => todo!("Implement mp3"),
        _ => None,
    }
}
