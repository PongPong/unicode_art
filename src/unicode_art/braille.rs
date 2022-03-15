use std::io::Write;

use super::color::{ANSI_BG_COLOUR_ESCAPES, ANSI_RESET_ATTRIBUTES};
use super::{color::AnsiColor, error::UnicodeArtError, UnicodeArt};
use image::io::Reader as ImageReader;
use image::DynamicImage;
use image::{imageops::FilterType, GenericImageView};

// Braille symbol is 2x4 dots
const X_DOTS: u8 = 2;
const Y_DOTS: u8 = 4;

pub const DEFAULT_THRESHOLD: u8 = 127;

pub struct BrailleAsciiArt<'a> {
    image_path: &'a str,
    threshold: u8, // range 0 - 255
    num_cols: u32,
    is_color: bool,
    is_invert: bool,
}

impl<'a> BrailleAsciiArt<'a> {
    pub fn new(
        num_cols: u32,
        image_path: &'a str,
        threshold: u8,
        is_color: bool,
        is_invert: bool,
    ) -> Self {
        BrailleAsciiArt {
            image_path,
            threshold,
            num_cols,
            is_color,
            is_invert,
        }
    }

    pub fn generate_without_color(
        &self,
        img: &DynamicImage,
        writer: &mut dyn Write,
    ) -> Result<(), UnicodeArtError> {
        let height = img.height();
        let width = img.width();
        // w: 645, h: 938
        // println!("w: {}, h: {}", width, height);
        let padding = &image::Rgba([0u8; 4]);
        for y in (0..height).step_by(Y_DOTS as usize) {
            for x in (0..width).step_by(X_DOTS as usize) {
                let sub_image = img.view(
                    x,
                    y,
                    (width - x).min(X_DOTS as u32),
                    (height - y).min(Y_DOTS as u32),
                );
                let sub_image = sub_image.to_image();
                let dots = [
                    sub_image.get_pixel_checked(0, 0).unwrap_or(padding), // 0
                    sub_image.get_pixel_checked(0, 1).unwrap_or(padding), // 2
                    sub_image.get_pixel_checked(0, 2).unwrap_or(padding), // 4
                    sub_image.get_pixel_checked(1, 0).unwrap_or(padding), // 1
                    sub_image.get_pixel_checked(1, 1).unwrap_or(padding), // 3
                    sub_image.get_pixel_checked(1, 2).unwrap_or(padding), // 5
                    sub_image.get_pixel_checked(0, 3).unwrap_or(padding), // 6
                    sub_image.get_pixel_checked(1, 3).unwrap_or(padding), // 7
                ];
                let bits = dots
                    // .map(|dot| (dot[0] < self.threshold) as u8);
                    .map(|dot| ((dot[0] as u32 + dot[1] as u32 + dot[2] as u32) / 3) as u8)
                    .map(|grey| {
                        (match grey < self.threshold {
                            true => !self.is_invert,
                            false => self.is_invert,
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
        // w: 645, h: 938
        // println!("w: {}, h: {}", width, height);
        let padding = &image::Rgba([0u8; 4]);
        let background = match self.is_invert {
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
                let dots = [
                    sub_image.get_pixel_checked(0, 0).unwrap_or(padding), // 0
                    sub_image.get_pixel_checked(0, 1).unwrap_or(padding), // 2
                    sub_image.get_pixel_checked(0, 2).unwrap_or(padding), // 4
                    sub_image.get_pixel_checked(1, 0).unwrap_or(padding), // 1
                    sub_image.get_pixel_checked(1, 1).unwrap_or(padding), // 3
                    sub_image.get_pixel_checked(1, 2).unwrap_or(padding), // 5
                    sub_image.get_pixel_checked(0, 3).unwrap_or(padding), // 6
                    sub_image.get_pixel_checked(1, 3).unwrap_or(padding), // 7
                ];
                let bits = dots
                    // .map(|dot| (dot[0] < self.threshold) as u8);
                    .map(|dot| ((dot[0] as u32 + dot[1] as u32 + dot[2] as u32) / 3) as u8)
                    .map(|grey| {
                        (match grey < self.threshold {
                            true => !self.is_invert,
                            false => self.is_invert,
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
    fn generate(&self, writer: &mut dyn Write) -> Result<(), UnicodeArtError> {
        let img = ImageReader::open(self.image_path)
            .map_err(|err| UnicodeArtError::from(err))?
            .decode()
            .map_err(|err| UnicodeArtError::from(err))?;

        let img = img.thumbnail(self.num_cols * X_DOTS as u32, ::std::u32::MAX);
        match self.is_color {
            true => self.generate_with_color(&img, writer),
            false => self.generate_without_color(&img, writer),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufWriter;

    #[test]
    fn test_generate_braille() {
        let art = BrailleAsciiArt::new(40, "tests/support/test_gundam.png", 12, false, false);
        let mut buf = BufWriter::new(Vec::new());
        let _ = art.generate(&mut buf);
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
