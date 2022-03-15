use super::color::AnsiColor;
use super::error::UnicodeArtError;
use super::mean::Mean;
use super::UnicodeArt;
use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImageView};
use std::io::Write;

const CHAR_LIST_STANDARD: &'static str =
    "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/|()1{}[]?-_+~<>i!lI;:,\"^`'. ";
const CHAR_LIST_LEVELS_10: &'static str = "@%#*+=-:. ";
const CHAR_LIST_LEVELS_19: &'static str = "BBQROHETI)7ri=+;:,.";
const CHAR_LIST_LEVELS_16: &'static str = "#8XOHLTI)i=+;:,.";
const CHAR_LIST_LEVELS_23: &'static str = "MWNXK0Okxdolc:;,'...   ";
/// ANSI background colour escapes.
const ANSI_BG_COLOUR_ESCAPES: [&str; 8] = [
    "\x1B[40m", "\x1B[41m", "\x1B[42m", "\x1B[43m", "\x1B[44m", "\x1B[45m", "\x1B[46m", "\x1B[47m",
];

#[derive(Default, Clone)]
pub struct SimpleAsciiUnicodeArt<'a> {
    is_color: bool,
    image_path: &'a str,
    char_list: &'static str,
    num_cols: u32,
}

impl<'a> SimpleAsciiUnicodeArt<'a> {
    pub fn new_standard(num_cols: u32, image_path: &'a str, is_color: bool) -> Self {
        Self {
            image_path,
            char_list: CHAR_LIST_STANDARD,
            num_cols,
            is_color,
        }
    }

    pub fn new_level_10(num_cols: u32, image_path: &'a str, is_color: bool) -> Self {
        Self {
            image_path,
            char_list: CHAR_LIST_LEVELS_10,
            num_cols,
            is_color,
        }
    }

    pub fn new_level_19(num_cols: u32, image_path: &'a str, is_color: bool) -> Self {
        Self {
            image_path,
            char_list: CHAR_LIST_LEVELS_19,
            num_cols,
            is_color,
        }
    }

    pub fn new_level_16(num_cols: u32, image_path: &'a str, is_color: bool) -> Self {
        Self {
            image_path,
            char_list: CHAR_LIST_LEVELS_16,
            num_cols,
            is_color,
        }
    }

    pub fn new_level_23(num_cols: u32, image_path: &'a str, is_color: bool) -> Self {
        Self {
            image_path,
            char_list: CHAR_LIST_LEVELS_23,
            num_cols,
            is_color,
        }
    }

    fn generate_with_color(
        &self,
        writer: &mut dyn Write,
        img: &DynamicImage,
    ) -> Result<(), UnicodeArtError> {
        let num_chars = self.char_list.len();
        let (width, height) = (img.width(), img.height());
        let img = img.thumbnail(
            self.num_cols,
            ((self.num_cols as f64 / width as f64) * height as f64) as u32,
        );
        let (num_rows, num_cols) = (img.height() / 2, img.width());

        let backround = &image::Rgba([0u8; 4]);
        for y in 0..num_rows {
            let upper_y = y * 2;
            for x in 0..num_cols {
                let upper_pixel = img.get_pixel(x, upper_y);
                let lower_pixel = img.get_pixel(x, upper_y + 1);
                let mean =
                    (lower_pixel[0] as u32 + lower_pixel[1] as u32 + lower_pixel[2] as u32) / 3;
                let char_idx = (num_chars - 1).min(mean as usize * num_chars / 255);
                let char = self.char_list.chars().nth(char_idx).unwrap();
                write!(
                    writer,
                    "{}{}{}",
                    upper_pixel.foreground(),
                    char,
                    backround.background()
                )?;
            }
            writeln!(writer, "{}", ANSI_BG_COLOUR_ESCAPES[0])?;
        }
        Ok(())
    }

    fn generate_with_grayscale(
        &self,
        writer: &mut dyn Write,
        img: &DynamicImage,
    ) -> Result<(), UnicodeArtError> {
        let num_chars = self.char_list.len();
        // let img = img.grayscale();
        let (width, height) = (img.width(), img.height());
        let mut cell_width = width / self.num_cols as u32;
        let mut cell_height = 2 * cell_width;
        let (mut num_rows, mut num_cols) = ((height / cell_height), self.num_cols);
        if num_cols > width || num_rows > height {
            eprintln!("too many colums or rows. Use default string");
            cell_width = 6;
            cell_height = 12;
            num_cols = width / cell_width;
            num_rows = height / cell_height;
        }

        for i in 0..num_rows {
            for j in 0..num_cols {
                let sy = i * cell_height;
                let ey = height.min((i + 1) * cell_height);
                let sx = j * cell_width;
                let ex = width.min((j + 1) * cell_width);
                let mean = img.mean(sx, ex, sy, ey);
                let char_idx = (num_chars - 1).min(mean as usize * num_chars / 255);
                let char = self.char_list.chars().nth(char_idx).unwrap();
                write!(writer, "{}", char)?
            }
            writeln!(writer)?
        }
        Ok(())
    }
}

impl<'a> UnicodeArt for SimpleAsciiUnicodeArt<'a> {
    fn generate(&self, writer: &mut dyn Write) -> Result<(), UnicodeArtError> {
        let img = ImageReader::open(self.image_path)
            .map_err(|err| UnicodeArtError::from(err))?
            .decode()
            .map_err(|err| UnicodeArtError::from(err))?;
        match self.is_color {
            true => self.generate_with_color(writer, &img),
            false => self.generate_with_grayscale(writer, &img),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::BufWriter;

    use super::*;

    #[test]
    fn test_generate_level_19() {
        let art = SimpleAsciiUnicodeArt::new_level_19(20, "tests/support/test_gundam.png", false);
        let mut buf = BufWriter::new(Vec::new());
        let _ = art.generate(&mut buf);
        let bytes = buf.into_inner().unwrap();
        let actual = String::from_utf8(bytes).unwrap();
        assert_eq!(
            r#"BBBBBBBBBBBBBBBBBBBB
BBBBBBBBBQQQBBBBBBBB
BBBBBQBBBQQQBBBBBBBB
BBBBQQRQBBBBBRQBQBBB
BBBRBBQBBBBBBBRQRBBB
BBBRQBBBBBBBBQBBRQBB
BBRQRBBBBBQBBQQRBQBB
BBBQRRRQQRRQBQRQRBBB
BBBBORQBBQBQBBQQBBBB
BBBBQRQBBBRQBBBBBBBB
BBBBRQRBBBOBBBBBBBBB
BBBBBBBQBQBBBBBBBBBB
BBBBBBBBQBBBBBBBBBBB
BBBBBBBBBBBBBBBBBBBB
"#,
            actual
        );
    }

    #[test]
    fn test_generate_standard() {
        let art = SimpleAsciiUnicodeArt::new_standard(20, "tests/support/test_gundam.png", false);
        let mut buf = BufWriter::new(Vec::new());
        let _ = art.generate(&mut buf);
        let bytes = buf.into_inner().unwrap();
        let actual = String::from_utf8(bytes).unwrap();
        assert_eq!(
            r#"&%$$$$$@88@$%$$$@B%%
$W%$$B&M%MoM88B%B@$$
$@88W#&W@M##WW&B%B$$
$$&W*ob#&B@@@a*8o$$$
$$%a$8*8B@$$%$h#h8$$
$$Wk#%@@$%B8@MW&h*$$
$$aoa&%&WB*&8M*hW*$$
$$$*hha##aa#$*a#k@$$
$$$$qb*$$MMM@B#M$$$$
$$$@#k#W%WhM$$B$$$$$
$$$8a*hM$$d%%@$$$$$$
$$@WW@&M&MB@$$$$$$$$
$$8%&&WW#B@$$$$$$$$$
$$$$$$B$8$$$$$$$$$$$
"#,
            actual
        );
    }

    #[test]
    fn test_generate_level_10() {
        let art = SimpleAsciiUnicodeArt::new_level_10(20, "tests/support/test_gundam.png", false);
        let mut buf = BufWriter::new(Vec::new());
        let _ = art.generate(&mut buf);
        let bytes = buf.into_inner().unwrap();
        let actual = String::from_utf8(bytes).unwrap();
        assert_eq!(
            r#"@@@@@@@@@@@@@@@@@@@@
@@@@@@@%@%%%@@@@@@@@
@@@@@%@@@%%%@@@@@@@@
@@@@%%#%@@@@@%%@%@@@
@@@%@@%@@@@@@@%%%@@@
@@@#%@@@@@@@@%@@%%@@
@@%%%@@@@@%@@%%%@%@@
@@@%%%%%%%%%@%%%#@@@
@@@@##%@@%%%@@%%@@@@
@@@@%#%@@@%%@@@@@@@@
@@@@%%%%@@#@@@@@@@@@
@@@@@@@%@%@@@@@@@@@@
@@@@@@@@%@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@
"#,
            actual
        );
    }
}
