use std::io;

#[derive(Debug)]
pub enum UnicodeArtError {
    UnsupportError,
    ImageError(image::ImageError),
    IoError(io::Error),
}

impl From<image::ImageError> for UnicodeArtError {
    fn from(err: image::ImageError) -> UnicodeArtError {
        UnicodeArtError::ImageError(err)
    }
}
impl From<io::Error> for UnicodeArtError {
    fn from(err: io::Error) -> UnicodeArtError {
        UnicodeArtError::IoError(err)
    }
}
