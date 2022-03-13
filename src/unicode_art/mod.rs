pub mod error;
pub mod simple;
pub mod mandel;
mod mean;

use crate::unicode_art::error::UnicodeArtError;
use std::io::Write;

pub trait UnicodeArt {
    fn generate(&self, image_path: &str, writer: &mut dyn Write) -> Result<(), UnicodeArtError>;
}
