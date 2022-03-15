pub mod block;
pub mod braille;
pub mod error;
pub mod mandel;
mod mean;
pub mod simple;

use crate::unicode_art::error::UnicodeArtError;
use std::io::Write;

pub trait UnicodeArt {
    fn generate(&self, writer: &mut dyn Write) -> Result<(), UnicodeArtError>;
}
