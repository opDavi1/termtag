// This file is part of termtag licensed under the GPL-3.0-or-later license.
// See the included LICENSE file or go to <https://www.gnu.org/licenses/> for more information.

use crate::metadata::{AudioMetadata, Metadata, Metadatum};

use std::{
    fs,
    io::{Error, ErrorKind},
    path::{Path, PathBuf},
};


// Every flac file starts with these 4 bytes as defined in the flac standard
const FLAC_MAGIC: &str = "fLaC";
const LAST_BLOCK_FLAG: u8 = 0x80; //10000000
const METADATA_TYPE_FLAG: u8 = 0x7F; //01111111


pub struct FlacFile {
    pub path: PathBuf,
}

impl FlacFile {
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_path_buf(),
        }
    }
}

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

impl From<VorbisCommentBlock> for Metadata {
    fn from(value: VorbisCommentBlock) -> Self {
        Metadata { fields: value.fields }
    }
}

#[derive(Debug, Default)]
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


pub struct PictureBlock {
    pub picture_type: PictureType,
    pub media_type_length: usize,
}


pub enum PictureType {
    Other = 0,
    PNG,
    GeneralIcon,
    FrontCover,
    BackCover,
    LinerNotes,
    MediaLabel,
    LeadArtist,
    ArtistOrPerformer,
    Conductor,
    BandOrOrchestra,
    Composer,
    Lyricist,
    RecordingLocation,
    DuringRecording,
    DuringPerformance,
    MovieOrScreenCapture,
    ABrightColoredFish,
    Illustration,
    BandOrArtistLogotype,
    PublisherOrStudioLogotype,
}


#[derive(Debug, Default)]
pub struct VorbisCommentBlock {
    pub vendor_length: usize,
    pub vendor: String,
    pub num_fields: usize,
    pub fields: Option<Vec<VorbisComment>>,
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

pub type VorbisComment = Metadatum;


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
    marker == FLAC_MAGIC
}