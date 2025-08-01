// This file is part of termtag licensed under the GPL-3.0-or-later license.
// See the included LICENSE file or go to <https://www.gnu.org/licenses/> for more information.

use std::io::Error;

#[derive(Debug, Default)]
/* TODO: add fields from Streaminfo such as sample rate, number of channels,
 * and bits per sample, as well as the vender field from the Vorbis comment.
 */

/*
pub struct Metadata {
    pub album: Option<String>,
    pub album_artist: Option<String>,
    pub artist: Option<String>,
    pub comment: Option<String>,
    pub genre: Option<String>,
    pub date: Option<String>,
    pub disk_number: Option<String>,
    pub label: Option<String>,
    pub title: Option<String>,
    pub track_number: Option<String>,
}
*/

pub struct Metadata {
    pub fields: Option<Vec<Metadatum>>,
}

#[derive(Debug, Default)]
pub struct Metadatum {
    pub key: String,
    pub value: String,
}

impl Metadatum {
    pub fn new(key: String, value: String) -> Self {
        Metadatum {
            key,
            value ,
        }
    }

    pub fn header_length(&self) -> u64 {
        // we add 1 here because of the '=' that is required between the key and value in the
        // metadata.
        (self.key.len() + self.value.len() + 1).try_into().unwrap()
    }
}


pub trait AudioMetadata {
    fn read_metadata(&self) -> Result<Metadata, Error>;
    fn write_metadata(&self, metadata: &Metadata) -> Result<(), Error>;
}
