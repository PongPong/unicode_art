mod aspect_ratio;
pub mod block;
pub mod braille;
pub mod classic;
mod color;
pub mod error;
pub mod input;
pub mod mandel;
mod mean;
pub mod subpixel;

use image::DynamicImage;

use crate::unicode_art::error::UnicodeArtError;
use std::io::{BufRead, Seek, Write};

pub trait SeekBufRead: Seek + BufRead + Sync {}

pub trait UnicodeArt {
    fn write_all(&self, writer: &mut dyn Write) -> Result<(), UnicodeArtError>;
}

pub trait UnicodeArtOption {
    fn new_unicode_art<'a>(
        &'a self,
        image: &'a DynamicImage,
    ) -> Result<Box<dyn UnicodeArt + 'a>, UnicodeArtError>;
}
