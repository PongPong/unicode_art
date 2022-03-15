use super::color::{AnsiColor, ANSI_BG_COLOUR_ESCAPES, ANSI_RESET_ATTRIBUTES};
use super::error::UnicodeArtError;
use super::UnicodeArt;
use image::io::Reader as ImageReader;
use image::GenericImageView;
use std::io::Write;

#[derive(Default)]
pub struct BlockUnicodeArt<'a> {
    is_color: bool,
    image_path: &'a str,
    num_cols: u32,
}

impl<'a> BlockUnicodeArt<'a> {
    pub fn new(num_cols: u32, image_path: &'a str, is_color: bool) -> Self {
        Self {
            image_path,
            num_cols,
            is_color,
        }
    }
}

impl<'a> UnicodeArt for BlockUnicodeArt<'a> {
    fn generate(&self, writer: &mut dyn Write) -> Result<(), UnicodeArtError> {
        let img = ImageReader::open(self.image_path)
            .map_err(|err| UnicodeArtError::from(err))?
            .decode()
            .map_err(|err| UnicodeArtError::from(err))?;
        let (width, height) = (img.width(), img.height());
        let mut img = img.thumbnail(
            self.num_cols,
            ((self.num_cols as f64 / width as f64) * height as f64) as u32,
        );
        if !self.is_color {
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
    use std::io::BufWriter;

    use super::*;

    #[test]
    fn test_generate_level_19() {
        let art = BlockUnicodeArt::new(20, "tests/support/test_gundam.png", false);
        let mut buf = BufWriter::new(Vec::new());
        let _ = art.generate(&mut buf);
        let bytes = buf.into_inner().unwrap();
        let actual = String::from_utf8(bytes).unwrap();
        println!("{}", actual);
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