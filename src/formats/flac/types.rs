// This file is part of termtag by Davis Sherman <davissherman2007@gmail.com> licensed under the GPL-3.0-or-later license.
// See the included LICENSE file or go to <https://www.gnu.org/licenses/> for more information.

use std::path::{Path, PathBuf};

pub struct FlacFile {
    pub path: PathBuf,
}


#[derive(Default)]
pub struct MetadataBlock {
    pub metadata_type: MetadataBlockType,
    pub length: usize,
    pub data: Vec<u8>,
}



#[derive(Debug, Default, PartialEq)]
pub enum MetadataBlockType {
    #[default]
    StreamInfo,
    Padding,
    Application,
    SeekTable,
    VorbisComment,
    Cuesheet,
    Picture,
    Reserved,
    Forbidden = 127,
}


#[derive(Debug, Default)]
pub struct VorbisCommentBlock {
    pub vendor_length: usize,
    pub vendor: String,
    pub num_fields: usize,
    pub fields: Option<Vec<VorbisComment>>,
}


#[derive(Debug, Default)]
pub struct VorbisComment {
    pub length: usize,
    pub key: String,
    pub value: String,
}

impl VorbisComment {
    pub fn new(length: usize, key: String, value: String) -> Self {
        VorbisComment {
            length,
            key,
            value ,
        }
    }
}
