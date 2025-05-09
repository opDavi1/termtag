use std::io::Error;

#[derive(Debug, Default)]
pub struct Metadata {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub track_number: Option<u32>,
    pub disk_number: Option<u32>,
    pub year: Option<u32>,
    pub genre: Option<String>,
}

pub trait AudioMetadata {
    fn read_metadata(&self) -> Result<Metadata, Error>;
    fn write_metadata(&self, metadata: &Metadata) -> Result<(), Error>;
}
