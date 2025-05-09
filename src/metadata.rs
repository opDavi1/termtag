use std::io::Error;

#[derive(Debug, Default)]
pub struct Metadata {
    pub album: Option<String>,
    pub album_artist: Option<String>,
    pub artist: Option<String>,
    pub comment: Option<String>,
    pub genre: Option<String>,
    pub year: Option<String>,
    pub month: Option<String>,
    pub day: Option<String>,
    pub disk_number: Option<String>,
    pub label: Option<String>,
    pub title: Option<String>,
    pub track_number: Option<String>,
}

pub trait AudioMetadata {
    fn read_metadata(&self) -> Result<Metadata, Error>;
    fn write_metadata(&self, metadata: &Metadata) -> Result<(), Error>;
}
