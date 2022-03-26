use super::color::{AnsiColor, ANSI_BG_COLOUR_ESCAPES, ANSI_RESET_ATTRIBUTES};
use super::error::UnicodeArtError;
use super::{UnicodeArt, UnicodeArtOption};
use image::{DynamicImage, GenericImageView};
use std::io::Write;

#[derive(Default)]
pub struct BlockUnicodeArtOption {
    is_color: bool,
    num_cols: u32,
}

impl BlockUnicodeArtOption {
    pub fn new(num_cols: u32, is_color: bool) -> Self {
        Self { num_cols, is_color }
    }
}

impl UnicodeArtOption for BlockUnicodeArtOption {
    fn new_unicode_art<'a>(
        &'a self,
        image: &'a DynamicImage,
    ) -> Result<Box<dyn UnicodeArt + 'a>, UnicodeArtError> {
        Ok(Box::new(BlockUnicodeArt {
            options: self,
            image,
        }))
    }
}

pub struct BlockUnicodeArt<'a> {
    options: &'a BlockUnicodeArtOption,
    image: &'a DynamicImage,
}

impl<'a> UnicodeArt for BlockUnicodeArt<'a> {
    fn write_all(&self, writer: &mut dyn Write) -> Result<(), UnicodeArtError> {
        let (width, height) = (self.image.width(), self.image.height());
        let mut img = self.image.thumbnail(
            self.options.num_cols,
            ((self.options.num_cols as f64 / width as f64) * height as f64) as u32,
        );
        if !self.options.is_color {
            img = img.grayscale();
        }
        let (num_rows, num_cols) = (img.height() / 2, img.width());

        for y in 0..num_rows {
            let upper_y = y * 2;
            let lower_y = upper_y + 1;
            for x in 0..num_cols {
                let upper_pixel = img.get_pixel(x, upper_y);
                let lower_pixel = img.get_pixel(x, lower_y);
                // 24 bits color
                write!(
                    writer,
                    "{}{}\u{2580}", // ▀
                    upper_pixel.foreground(),
                    lower_pixel.background(),
                )?;
            }
            writeln!(writer, "{}", ANSI_BG_COLOUR_ESCAPES[0])?;
        }
        write!(writer, "{}", ANSI_RESET_ATTRIBUTES)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use image::io::Reader;
    use std::io::BufWriter;

    use super::*;

    #[test]
    fn test_generate_level_19() {
        let image_path = "tests/support/test_gundam.png";
        let image = Reader::open(image_path).unwrap();
        let art = BlockUnicodeArtOption::new(20, false)
            .new_unicode_art(&image.decode().unwrap())
            .unwrap();
        let mut buf = BufWriter::new(Vec::new());
        let _ = art.write_all(&mut buf);
        let bytes = buf.into_inner().unwrap();
        let _ = String::from_utf8(bytes).unwrap();
        // println!("{}", actual);
        //         assert_eq!(
        //             r#"BBBBBBBBBBBBBBBBBBBB
        // BBBBBBBBBQQQBBBBBBBB
        // BBBBBQBBBQQQBBBBBBBB
        // BBBBQQRQBBBBBRQBQBBB
        // BBBRBBQBBBBBBBRQRBBB
        // BBBRQBBBBBBBBQBBRQBB
        // BBRQRBBBBBQBBQQRBQBB
        // BBBQRRRQQRRQBQRQRBBB
        // BBBBORQBBQBQBBQQBBBB
        // BBBBQRQBBBRQBBBBBBBB
        // BBBBRQRBBBOBBBBBBBBB
        // BBBBBBBQBQBBBBBBBBBB
        // BBBBBBBBQBBBBBBBBBBB
        // BBBBBBBBBBBBBBBBBBBB
        // "#,
        //     actual
        // );
    }
}
