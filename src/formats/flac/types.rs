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

#[derive(Default)]
pub struct VorbisCommentBlock {
    vendor_length: usize,
    vendor: String,
    num_fields: usize,
    fields: Vec<VorbisComment>,
}

impl From<Vec<u8>> for VorbisCommentBlock {
    fn from(data: Vec<u8>) -> Self {
        let vendor_length = usize::from_le_bytes([data[0], data[1], data[2], data[3], 0, 0, 0, 0]);
        let mut index = 4;

        let vendor = String::from_utf8(data[index..index+vendor_length]
            .to_vec())
            .unwrap_or(String::from("Unknown Vendor")); 
        index += vendor_length;
        println!("Vendor: {:?}", vendor);
        
        let num_fields = usize::from_le_bytes([data[index], data[index+1], data[index+2], data[index+3], 0, 0, 0, 0]);
        println!("Number of fields: {:?}", num_fields);
        

        VorbisCommentBlock { vendor_length, vendor, num_fields, fields: vec![VorbisComment::default()] }
    }
}

#[derive(Default)]
pub struct VorbisComment {
    length: usize,
    key: String,
    value: String,
}