use std::{
    fs,
    io::{Error, ErrorKind},
    path::Path,
};

const MAGIC: &str = "fLaC";

#[derive(Debug, Default)]
pub enum MetadataType {
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

impl From<u8> for MetadataType {
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
pub struct Metadata {
    metadata_type: MetadataType,
    length: usize,
    data: Vec<u8>,
}

impl Metadata {
    fn new(metadata_type: MetadataType, length: usize, data: Vec<u8>) -> Self {
        Metadata {
            metadata_type,
            length,
            data,
        }
    }
}

fn is_flac(data: &Vec<u8>) -> bool {
    let marker = match String::from_utf8(data[0..4].to_vec()) {
        Ok(m) => m,
        Err(_) => return false,
    };
    marker == MAGIC
}

pub fn get_metadata(path: &Path) -> Result<Vec<Metadata>, Error> {
    let data: Vec<u8> = fs::read(path)?;
    if !is_flac(&data) {
        return Err(Error::new(ErrorKind::InvalidData, "Given file is not FLAC."));
    }

    let mut index: usize = 4;
    let mut metadata_list: Vec<Metadata> = Vec::new();
    let mut is_last_block = false;
    while !is_last_block {
        let header = data[index..=index+3].to_vec();
        index += 4;

        is_last_block = match header[0] & 0b10000000 {
            0 => false,
            1 => true,
            _ => true,
        };
        println!("Is last block: {:?}", is_last_block);

        let metadata_type = MetadataType::from(header[0] & 0b01111111);
        println!("Metadata type: {:?}", metadata_type);

        let length = usize::from_be_bytes([0, 0, 0, 0, 0, header[1], header[2], header[3]]);
        println!("Metadata length: {:?}", length);

        let data: Vec<u8> = data[index..index+length].to_vec();
        println!("Data: {:?}\n", data);

        index += length;
        metadata_list.push(Metadata::new(
            metadata_type,
            length,
            data,
        ));
    }

    Ok(metadata_list)
}
