use std::{fmt, path::Path};

pub struct FlacFile {
    pub path: String,
}

impl FlacFile {
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_string_lossy().to_string(),
        }
    }
}

#[derive(Default)]
pub struct MetadataBlock {
    pub metadata_type: MetadataBlockType,
    pub length: usize,
    pub data: Vec<u8>,
}

impl MetadataBlock {
    pub fn new(metadata_type: MetadataBlockType, length: usize, data: Vec<u8>) -> Self {
        MetadataBlock {
            metadata_type,
            length,
            data,
        }
    }
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

impl From<u8> for MetadataBlockType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::StreamInfo,
            1 => Self::Padding,
            2 => Self::Application,
            3 => Self::SeekTable,
            4 => Self::VorbisComment,
            5 => Self::Cuesheet,
            6 => Self::Picture,
            7..127 => Self::Reserved,
            _ => Self::Forbidden,
        }
    }
}