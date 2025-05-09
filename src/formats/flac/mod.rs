use std::{
    fs,
    io::{Error, ErrorKind},
    path::Path,
};

use crate::metadata::{AudioMetadata, Metadata};

mod types;
pub use types::*;


// Every flac file starts with these 4 bytes
const MAGIC: &str = "fLaC";
const LAST_BLOCK_FLAG: u8 = 0b10000000;
const METADATA_TYPE_FLAG: u8 = 0b01111111;


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
                metadata.artist = value;
            } else if key == "artist" {
                metadata.artist = value
            } else if key == "comment" {
                metadata.comment = value;
            } else if key == "genre" {
                metadata.genre = value;
            } else if key == "year" {
                metadata.year = value;
            } else if key == "month" {
                metadata.month = value;
            } else if key == "day" {
                metadata.day = value;
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

fn is_flac(data: &Vec<u8>) -> bool {
    let marker = match String::from_utf8(data[0..4].to_vec()) {
        Ok(m) => m,
        Err(_) => return false,
    };
    marker == MAGIC
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
