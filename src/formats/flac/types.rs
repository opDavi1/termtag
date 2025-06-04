// This file is part of termtag licensed under the GPL-3.0-or-later license.
// See the included LICENSE file or go to <https://www.gnu.org/licenses/> for more information.

use std::path::{Path, PathBuf};

use crate::metadata::Metadatum;


pub struct FlacFile {
    pub path: PathBuf,
}


#[derive(Debug, Default)]
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

pub type VorbisComment = Metadatum;
