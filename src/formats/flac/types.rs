use crate::metadata::{Metadata, Metadatum};
use std::path::{Path, PathBuf};


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


impl From<VorbisCommentBlock> for Metadata {
    fn from(value: VorbisCommentBlock) -> Self {
        Metadata { fields: value.fields }
    }
}


#[derive(Debug, Default)]
pub struct MetadataBlock {
    pub metadata_type: MetadataBlockType,
    pub data: Vec<u8>,
}

impl MetadataBlock {
    pub fn new(metadata_type: MetadataBlockType, data: Vec<u8>) -> Self {
        MetadataBlock {
            metadata_type,
            data,
        }
    }

    pub fn header_length(&self) -> u64 {
        self.data.len().try_into().unwrap()
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

pub type VorbisComment = Metadatum;

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
                    key.to_string(),
                    value[1..].to_string(),
                ))
            }
        }
        VorbisCommentBlock::new(vendor_length, vendor, num_fields, Some(fields))
    }
}
