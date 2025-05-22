// This file is part of termtag licensed under the GPL-3.0-or-later license.
// See the included LICENSE file or go to <https://www.gnu.org/licenses/> for more information.

use std::path::{Path, PathBuf};

use crate::metadata::Metadatum;


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

pub type VorbisComment = Metadatum;
