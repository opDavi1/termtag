// This file is part of termtag licensed under the GPL-3.0-or-later license.
// See the included LICENSE file or go to <https://www.gnu.org/licenses/> for more information.

use std::io::Error;

#[derive(Debug, Default)]
/* TODO: add fields from Streaminfo such as sample rate, number of channels,
 * and bits per sample, as well as the vender field from the Vorbis comment.
 */

/* TODO: add support for multiple artist tags; make artist an Option<Vec<String>>
 * or something
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
    pub length: usize,
    pub key: String,
    pub value: String,
}

impl Metadatum {
    pub fn new(length: usize, key: String, value: String) -> Self {
        Metadatum {
            length,
            key,
            value ,
        }
    }
}


pub trait AudioMetadata {
    fn read_metadata(&self) -> Result<Metadata, Error>;
    fn write_metadata(&self, metadata: &Metadata) -> Result<(), Error>;
}
