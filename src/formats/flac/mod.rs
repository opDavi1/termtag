use std::{
    fs,
    io::{Error, ErrorKind},
    path::Path,
};

use crate::metadata::{AudioMetadata, Metadata};

mod types;
use types::*;
pub use types::FlacFile;


// Every flac file starts with these 4 bytes
const MAGIC: &str = "fLaC";
const LAST_BLOCK_FLAG: u8 = 0b10000000;
const METADATA_TYPE_FLAG: u8 = 0b01111111;


impl FlacFile {
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_path_buf(),
        }
    }
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


impl VorbisCommentBlock {
    pub fn new(vendor_length: usize, vendor: String, num_fields: usize, fields: Option<Vec<VorbisComment>>) -> Self {
        VorbisCommentBlock {
            vendor_length,
            vendor,
            num_fields,
            fields
        }
    }
}

impl From<&Vec<u8>> for VorbisCommentBlock {
    fn from(data: &Vec<u8>) -> Self {
        // The length fields in the Vorbis Comment metadata block are little endian instead of big endian like the rest of FLAC.
        let vendor_length = usize::from_le_bytes([data[0], data[1], data[2], data[3], 0, 0, 0, 0]);
        let mut index = 4;

        let vendor = String::from_utf8(data[index..index+vendor_length]
            .to_vec())
            .unwrap_or(String::from("Unknown Vendor")); 
        index += vendor_length;
        
        let num_fields = usize::from_le_bytes([data[index], data[index+1], data[index+2], data[index+3], 0, 0, 0, 0]);
        index += 4;
        
        if num_fields == 0 {
            return VorbisCommentBlock::new(vendor_length, vendor, num_fields, None );
        }

        let mut fields: Vec<VorbisComment> = Vec::new();
        for _ in 0..num_fields {
            let length = usize::from_le_bytes([data[index], data[index+1], data[index+2], data[index+3], 0, 0, 0, 0]);
            index += 4;

            let string = String::from_utf8(data[index..index+length].to_vec())
            .unwrap_or(String::new());
            index += length;

            if let Some(pos) = string.find("=") {
                let (key, value) = string.split_at(pos);
                fields.push(VorbisComment::new(
                    length,
                    key.to_string(),
                    value[1..].to_string(),
                ))
            }
        }
        VorbisCommentBlock::new(vendor_length, vendor, num_fields, Some(fields))
    }
}

impl AudioMetadata for FlacFile {
    fn read_metadata(&self) -> Result<Metadata, Error> {
        let data: Vec<u8> = fs::read(&self.path)?;
        if !is_flac(&data) {
            return Err(Error::new(ErrorKind::InvalidData, "Given path is not a FLAC file."));
        }

        let metadata_blocks = get_metadata_blocks(&data)?;
        let vorbis_comment_block= match metadata_blocks
        .iter()
        .filter(|block| block.metadata_type == MetadataBlockType::VorbisComment)
        .nth(0) {
            Some(b) => VorbisCommentBlock::from(&b.data),
            None => VorbisCommentBlock::default(),
        };

        let fields = match vorbis_comment_block.fields {
            Some(f) => f,
            None => return Ok(Metadata::default()),
        };

        let mut metadata = Metadata::default();
        for field in fields.iter() {
            let key = field.key.to_lowercase();
            let value = Some(field.value.clone());
            if key == "album" {
                metadata.album = value;
            } else if key == "albumartist" {
                metadata.album_artist = value;
            } else if key == "artist" {
                metadata.artist = value
            } else if key == "comment" {
                metadata.comment = value;
            } else if key == "genre" {
                metadata.genre = value;
            } else if key == "date" {
                metadata.date = value;
            } else if key == "disknumber" {
                metadata.disk_number = value;
            } else if key == "label" {
                metadata.label = value;
            } else if key == "title" {
                metadata.title = value;
            } else if key == "tracknumber" {
                metadata.track_number = value;
            }
        };
        eprintln!("Metadata: {:?}", metadata);
        Ok(metadata)
    }
    
    fn write_metadata(&self, metadata: &Metadata) -> Result<(), Error> {
        todo!()
    }
}


fn get_metadata_blocks(data: &Vec<u8>) -> Result<Vec<MetadataBlock>, Error> {

    let mut index: usize = 4;
    let mut metadata_list: Vec<MetadataBlock> = Vec::new();
    let mut is_last_block = false;
    while !is_last_block {
        let header = data[index..=index+3].to_vec();
        index += 4;

        is_last_block = match header[0] & LAST_BLOCK_FLAG {
            0 => false,
            _ => true,
        };

        let metadata_type = MetadataBlockType::from(header[0] & METADATA_TYPE_FLAG);
        let length = usize::from_be_bytes([0, 0, 0, 0, 0, header[1], header[2], header[3]]);
        let data: Vec<u8> = data[index..index+length].to_vec();
        index += length;

        metadata_list.push(MetadataBlock::new(
            metadata_type,
            length,
            data,
        ));
    }

    Ok(metadata_list)
}


fn is_flac(data: &Vec<u8>) -> bool {
    let marker = match String::from_utf8(data[0..4].to_vec()) {
        Ok(m) => m,
        Err(_) => return false,
    };
    marker == MAGIC
}
