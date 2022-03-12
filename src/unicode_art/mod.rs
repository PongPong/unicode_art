pub mod simple;
pub mod error;

use crate::unicode_art::error::UnicodeArtError;

pub trait UnicodeArt {
    fn generate(&self, image_path: &str) -> Result<(),UnicodeArtError>;
}
