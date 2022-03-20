mod aspect_ratio;
pub mod block;
pub mod braille;
pub mod classic;
mod color;
pub mod error;
pub mod mandel;
mod mean;
pub mod subpixel;

use crate::unicode_art::error::UnicodeArtError;
use std::io::Write;

pub trait UnicodeArt {
    fn write_all(&self, writer: &mut dyn Write) -> Result<(), UnicodeArtError>;
}

pub trait UnicodeArtOption<'a> {
    fn new_unicode_art(self) -> Result<Box<dyn UnicodeArt + 'a>, UnicodeArtError>;
}
