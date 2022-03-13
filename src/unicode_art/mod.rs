pub mod error;
pub mod simple;
mod mean;

use crate::unicode_art::error::UnicodeArtError;
use std::io::Write;

pub trait UnicodeArt {
    fn generate<W: ?Sized>(&self, image_path: &str, writer: &mut W) -> Result<(), UnicodeArtError>
    where
        W: Write;
}
