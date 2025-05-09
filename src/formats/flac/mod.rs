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
        let metadata_blocks = get_metadata_blocks(Path::new(&self.path)).unwrap();
        let vorbis_comment = metadata_blocks
        .iter()
        .filter(|block| block.metadata_type == MetadataBlockType::VorbisComment);

        Ok(Metadata::default())
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

fn get_metadata_blocks(path: &Path) -> Result<Vec<MetadataBlock>, Error> {
    let data: Vec<u8> = fs::read(path)?;
    if !is_flac(&data) {
        return Err(Error::new(ErrorKind::InvalidData, "Given file is not FLAC."));
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
        println!("Is last block: {:?}", is_last_block);

        let metadata_type = MetadataBlockType::from(header[0] & METADATA_TYPE_FLAG);
        println!("Metadata type: {:?}", metadata_type);

        let length = usize::from_be_bytes([0, 0, 0, 0, 0, header[1], header[2], header[3]]);
        println!("Metadata length: {:?}", length);

        let data: Vec<u8> = data[index..index+length].to_vec();
        println!("Data: {:?}\n", data);

        index += length;
        metadata_list.push(MetadataBlock::new(
            metadata_type,
            length,
            data,
        ));
    }

    Ok(metadata_list)
}
