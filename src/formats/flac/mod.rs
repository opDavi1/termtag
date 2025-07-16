// This file is part of termtag licensed under the GPL-3.0-or-later license.
// See the included LICENSE file or go to <https://www.gnu.org/licenses/> for more information.

mod types;

use crate::metadata::{AudioMetadata, Metadata};
use types::*;
pub use types::FlacFile;

use std::{
    fs,
    io::{Error, ErrorKind},
};


// Every flac file starts with these 4 bytes as defined in the flac standard
const FLAC_MAGIC: [u8; 4] = [0x66, 0x4C, 0x61, 0x43]; // "fLaC"
const LAST_BLOCK_FLAG: u8 = 0x80; // 10000000
const METADATA_TYPE_FLAG: u8 = 0x7F; // 01111111



impl AudioMetadata for FlacFile {
    fn read_metadata(&self) -> Result<Metadata, Error> {
        let data: Vec<u8> = fs::read(&self.path)?;
        if !is_flac(&data) {
            return Err(Error::new(ErrorKind::InvalidData, "Given path is not a FLAC file."));
        }

        let metadata_blocks = get_metadata_blocks(&data)?;
        let vorbis_comment_block: VorbisCommentBlock = match metadata_blocks
            .iter()
            .filter(|block| block.metadata_type == MetadataBlockType::VorbisComment)
            .next() {
            Some(b) => VorbisCommentBlock::from(&b.data),
            None => VorbisCommentBlock::default(),
        };

        let picture_blocks: Vec<&MetadataBlock> = metadata_blocks
            .iter()
            .filter(|block| block.metadata_type == MetadataBlockType::Picture)
            .collect();

        println!("{:?}", picture_blocks);

        let metadata = Metadata::from(vorbis_comment_block);
        eprintln!("Metadata: {:?}", metadata);
        Ok(metadata)
    }

    fn write_metadata(&self, metadata: &Metadata) -> Result<(), Error> {
        todo!()
    }
}

fn get_metadata_blocks(data: &Vec<u8>) -> Result<Vec<MetadataBlock>, Error> {

    let data_len = data.len();

    if data.len() < 4 {
        return Err(
            Error::new(
                ErrorKind::InvalidData, 
                "Invalid or corrupt FLAC file detected"
            )
        );
    }

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

        if index + length > data_len {
            return Err(
                Error::new(
                    ErrorKind::InvalidData,
                    "Invalid or corrupt FLAC file given"
                )
            );
        }

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
    data[0..4] == FLAC_MAGIC
}
