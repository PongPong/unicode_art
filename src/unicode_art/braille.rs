use std::io::Write;

use super::color::{ANSI_BG_COLOUR_ESCAPES, ANSI_RESET_ATTRIBUTES};
use super::UnicodeArtOption;
use super::{color::AnsiColor, error::UnicodeArtError, UnicodeArt};
use clap::lazy_static::lazy_static;
use image::{imageops::FilterType, GenericImageView};
use image::{DynamicImage, ImageBuffer, Rgba};

// Braille symbol is 2x4 dots
const X_DOTS: u8 = 2;
const Y_DOTS: u8 = 4;

pub const DEFAULT_THRESHOLD: u8 = 127;

pub struct BrailleAsciiArtOption {
    threshold: u8, // range 0 - 255
    num_cols: u32,
    is_color: bool,
    is_invert: bool,
}

pub struct BrailleAsciiArt<'a> {
    options: &'a BrailleAsciiArtOption,
    image: &'a DynamicImage,
}

trait BrailleDot {
    fn to_dots(&self) -> [&Rgba<u8>; 8];
}

lazy_static! {
    static ref PADDING: image::Rgba<u8> = image::Rgba([0u8; 4]);
}

impl BrailleDot for ImageBuffer<Rgba<u8>, Vec<u8>> {
    fn to_dots(&self) -> [&Rgba<u8>; 8] {
        [
            self.get_pixel_checked(0, 0), // 0
            self.get_pixel_checked(0, 1), // 2
            self.get_pixel_checked(0, 2), // 4
            self.get_pixel_checked(1, 0), // 1
            self.get_pixel_checked(1, 1), // 3
            self.get_pixel_checked(1, 2), // 5
            self.get_pixel_checked(0, 3), // 6
            self.get_pixel_checked(1, 3), // 7
        ]
        .map(|dot| dot.unwrap_or(&PADDING))
    }
}

impl BrailleAsciiArtOption {
    pub fn new(num_cols: u32, threshold: u8, is_color: bool, is_invert: bool) -> Self {
        Self {
            threshold,
            num_cols,
            is_color,
            is_invert,
        }
    }
}

impl UnicodeArtOption for BrailleAsciiArtOption {
    fn new_unicode_art<'a>(
        &'a self,
        image: &'a DynamicImage,
    ) -> Result<Box<dyn UnicodeArt + 'a>, UnicodeArtError> {
        Ok(Box::new(BrailleAsciiArt {
            options: self,
            image,
        }))
    }
}

impl<'a> BrailleAsciiArt<'a> {
    pub fn generate_without_color(
        &self,
        img: &DynamicImage,
        writer: &mut dyn Write,
    ) -> Result<(), UnicodeArtError> {
        let height = img.height();
        let width = img.width();
        for y in (0..height).step_by(Y_DOTS as usize) {
            for x in (0..width).step_by(X_DOTS as usize) {
                let sub_image = img.view(
                    x,
                    y,
                    (width - x).min(X_DOTS as u32),
                    (height - y).min(Y_DOTS as u32),
                );
                let sub_image = sub_image.to_image();
                let dots = sub_image.to_dots();
                let bits = dots
                    .map(|dot| ((dot[0] as u32 + dot[1] as u32 + dot[2] as u32) / 3) as u8)
                    .map(|grey| {
                        (match grey < self.options.threshold {
                            true => !self.options.is_invert,
                            false => self.options.is_invert,
                        }) as u8
                    });
                let dec = bits.iter().rev().fold(0, |acc, &b| acc * 2 + b as u32);
                // Braille Unicode range starts at U2800 (= 10240 decimal)
                let char = char::from_u32(dec + 10240).unwrap();
                write!(writer, "{}", char)?;
            }
            writeln!(writer)?;
        }
        Ok(())
    }

    pub fn generate_with_color(
        &self,
        img: &DynamicImage,
        writer: &mut dyn Write,
    ) -> Result<(), UnicodeArtError> {
        let height = img.height();
        let width = img.width();
        let background = match self.options.is_invert {
            true => &image::Rgba([0u8; 4]),
            false => &image::Rgba([255u8; 4]),
        };
        for y in (0..height).step_by(Y_DOTS as usize) {
            for x in (0..width).step_by(X_DOTS as usize) {
                let sub_image = img.view(
                    x,
                    y,
                    (width - x).min(X_DOTS as u32),
                    (height - y).min(Y_DOTS as u32),
                );
                let pixel =
                    image::imageops::resize(&sub_image.to_image(), 1, 1, FilterType::CatmullRom);
                // let pixel = sub_image.to_image().resize_exact(1, 1, FilterType::Triangle);
                let sub_image = sub_image.to_image();
                let dots = sub_image.to_dots();
                let bits = dots
                    // .map(|dot| (dot[0] < self.threshold) as u8);
                    .map(|dot| ((dot[0] as u32 + dot[1] as u32 + dot[2] as u32) / 3) as u8)
                    .map(|grey| {
                        (match grey < self.options.threshold {
                            true => !self.options.is_invert,
                            false => self.options.is_invert,
                        }) as u8
                    });
                let dec = bits.iter().rev().fold(0, |acc, &b| acc * 2 + b as u32);
                let char = char::from_u32(dec + 10240).unwrap();

                write!(
                    writer,
                    "{}{}{}",
                    pixel.get_pixel(0, 0).foreground(),
                    char,
                    background.background()
                )?;
            }
            writeln!(writer, "{}", ANSI_BG_COLOUR_ESCAPES[0])?;
        }
        write!(writer, "{}", ANSI_RESET_ATTRIBUTES)?;
        Ok(())
    }
}

impl<'a> UnicodeArt for BrailleAsciiArt<'a> {
    fn write_all(&self, writer: &mut dyn Write) -> Result<(), UnicodeArtError> {
        let img = self
            .image
            .thumbnail(self.options.num_cols * X_DOTS as u32, ::std::u32::MAX);
        match self.options.is_color {
            true => self.generate_with_color(&img, writer),
            false => self.generate_without_color(&img, writer),
        }
    }
}

#[cfg(test)]
mod tests {
    use image::io::Reader;

    use super::*;
    use std::io::BufWriter;

    #[test]
    fn test_generate_braille() {
        let image_path = "tests/support/test_gundam.png";
        let image = Reader::open(image_path);
        let art = BrailleAsciiArtOption::new(40, 12, false, false)
            .new_unicode_art(&image.unwrap().decode().unwrap())
            .unwrap();
        let mut buf = BufWriter::new(Vec::new());
        let _ = art.write_all(&mut buf);
        let bytes = buf.into_inner().unwrap();
        let actual = String::from_utf8(bytes).unwrap();
        assert_eq!(
            r#"⡏⢎⢻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠟⠋⣡⣼
⣿⡌⢦⠻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠟⣛⠛⠛⠛⠿⢿⣿⡟⢁⣾⣿⣿⣿⣿⣿⣿⡿⢛⡩⢐⣵⣾⣿⣿
⣿⣿⡘⣧⠹⣿⣿⣿⣿⣿⣿⠿⣋⣵⠞⠋⣠⣴⣶⠀⢹⡦⠈⠀⠾⣿⣿⣿⡿⠟⣋⡥⢞⣡⣾⣿⣿⣿⣿⣿
⣿⣿⣷⠸⣧⡹⣿⣿⣿⢟⣵⣶⢖⢄⠐⣿⣿⣿⣿⣇⠀⡡⣱⣿⣷⠦⢙⣩⣶⠟⣩⣶⣿⣿⡿⢿⣿⣿⣿⣿
⣿⣿⣿⣧⢹⣷⡘⣿⡁⣄⢠⡔⢱⣿⣦⠸⣿⠿⣋⢅⡜⡠⠐⣀⣵⣾⠟⠫⣴⣿⣿⣿⢟⠩⣢⣾⣿⣿⣿⣿
⣿⣿⣿⣿⣆⢿⣷⡘⣧⢹⡜⣿⡌⢹⢿⠇⣡⣾⠄⣊⣤⣶⣿⠿⣋⡑⠿⣦⡙⠟⡫⣀⣴⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⡌⡿⢷⡘⢂⣀⣒⠒⠀⠨⢚⡩⢶⣿⣿⡿⢛⣥⣾⣿⡷⠄⡨⢐⡅⣦⣬⠙⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⡇⢇⠈⢡⣦⣤⢠⡵⠀⡊⢅⣠⣾⡿⢩⣾⣿⣿⡿⠏⠴⣧⡄⢸⡇⣿⣿⠀⢻⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⡿⡈⠄⣾⣿⣿⢸⡇⣴⣾⡿⠟⣋⣵⣿⣿⣿⣿⣷⣤⣾⣿⣿⡈⠕⠿⠟⡀⡌⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⠡⢿⢸⣿⣿⠟⢘⡃⠛⢡⣶⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⢣⣌⣙⠣⢰⣧⠘⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⢘⠠⡍⡉⣠⣶⣿⡿⢃⣾⣿⣿⣿⣿⣿⣿⣿⡿⠁⣿⣿⡟⣴⡿⠿⢏⠔⠸⣿⡇⢹⣿⣿⣿⣿
⣿⣿⣿⣿⡿⠂⠧⠄⠑⠼⢛⣥⣾⣿⢿⠿⠿⠿⠛⣛⣩⣵⣴⠿⢻⣟⣠⢐⣒⣚⢫⣼⣄⠲⠁⢸⣿⣿⣿⣿
⣿⣿⣿⣿⡇⡀⠟⢁⣇⢘⣿⣿⣿⠷⢃⣤⠴⠿⠟⣛⠛⣿⣷⠾⢸⣿⣧⠍⠉⠉⠹⠿⣿⡆⠇⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣧⡑⢬⠘⠃⢰⠈⠋⣁⠝⠻⠷⠶⠿⠿⣫⠆⠄⣵⡄⢸⣿⢋⢻⡶⢃⡄⠘⡿⠁⣸⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣆⠸⣇⠸⣤⠀⣤⣉⡑⠀⠲⠒⠃⠑⠊⠘⠟⢠⣿⡏⣼⣄⠰⠟⠃⣰⠃⢀⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⡄⠿⠀⢡⡈⡉⢹⣿⣿⣷⣶⣿⡆⠀⣷⡆⣿⣿⣿⡄⡉⣁⣂⢸⣿⢀⣼⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠈⡁⠋⠛⢿⣿⣿⣿⡿⠀⡧⢹⠇⣿⢿⣿⣿⠆⠉⣀⣬⡀⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⡇⠄⢀⠿⠘⠳⢾⣿⣿⣿⠃⡸⠁⢹⠀⢋⣼⣿⢛⣵⣾⣿⢏⣼⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⠫⣤⡤⠓⣦⢇⣎⢻⡿⡫⢖⡃⠁⢸⡇⣿⣛⣶⣿⣿⡿⢣⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⡇⣶⠈⡆⠈⢩⠘⡿⠀⣴⣾⣿⡇⠸⣶⢢⢶⣿⣿⣿⠟⣵⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⡿⢸⡇⠀⠉⡀⠀⣥⣴⠃⣿⣿⣿⣇⠀⢃⣾⡆⣿⡿⢫⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⢃⣿⢠⣠⣾⣇⢣⠹⡟⣸⣿⣿⣿⠿⠀⣾⣿⣿⠘⣵⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⡟⣸⡇⣼⣿⣿⣿⣌⠓⢠⣭⣭⠥⠶⠊⣼⣿⡿⢫⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⢃⣿⢰⠩⢀⡚⠿⢿⣿⣶⣶⣶⣾⠏⣼⣿⢟⣴⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⡏⢜⣡⣴⣤⣍⣛⠻⠶⣦⣭⠡⠀⠝⣼⡿⣣⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣭⣶⣿⣿⣿⣿⣿⣿⣿⣷⡆⣭⣑⠚⣼⢋⣼⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢏⣴⣿⣿⠐⣵⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⣣⣾⡿⣡⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣯⣾⣿⢏⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
"#,
            actual
        );
    }
}
