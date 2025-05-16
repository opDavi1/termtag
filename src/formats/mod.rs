// This file is part of termtag by Davis Sherman <davissherman2007@gmail.com> licensed under the GPL-3.0-or-later license.
// See the included LICENSE file or go to <https://www.gnu.org/licenses/> for more information.

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
