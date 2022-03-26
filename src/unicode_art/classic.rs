use super::aspect_ratio::{AspectRatio, SimpleAspectRatio, TermFit};
use super::color::{AnsiColor, ANSI_RESET_ATTRIBUTES};
use super::error::UnicodeArtError;
use super::mean::Mean;
use super::{UnicodeArt, UnicodeArtOption};
use image::{DynamicImage, GenericImageView};
use std::io::Write;

pub const CHAR_LIST_STANDARD: &'static str =
    "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/|()1{}[]?-_+~<>i!lI;:,\"^`'. ";
pub const CHAR_LIST_LEVELS_10: &'static str = "@%#*+=-:. ";
pub const CHAR_LIST_LEVELS_19: &'static str = "BBQROHETI)7ri=+;:,.";
pub const CHAR_LIST_LEVELS_16: &'static str = "#8XOHLTI)i=+;:,.";
pub const CHAR_LIST_LEVELS_23: &'static str = "MWNXK0Okxdolc:;,'...   ";
pub const CHAR_LIST_LEVELS_4: &'static str = "3210";

/// ANSI background colour escapes.
const ANSI_BG_COLOUR_ESCAPES: [&str; 8] = [
    "\x1B[40m", "\x1B[41m", "\x1B[42m", "\x1B[43m", "\x1B[44m", "\x1B[45m", "\x1B[46m", "\x1B[47m",
];

#[derive(Default, Clone)]
pub struct ClassicAsciiArtOption<'a> {
    pub(crate) is_color: bool,
    pub(crate) is_invert: bool,
    pub(crate) char_list: &'a str,
    pub(crate) num_cols: Option<u32>,
    pub(crate) num_rows: Option<u32>,
}

pub struct ClassicAsciiArt<'a> {
    pub options: &'a ClassicAsciiArtOption<'a>,
    pub image: &'a DynamicImage,
}

impl<'a> ClassicAsciiArt<'a> {
    fn generate_with_color(&self, writer: &mut dyn Write) -> Result<(), UnicodeArtError> {
        let num_chars = self.options.char_list.len();
        let (num_cols, num_rows) = match (self.options.num_cols, self.options.num_rows) {
            (Some(cols), Some(rows)) => (cols, rows),
            (Some(cols), None) => SimpleAspectRatio::new_auto_height(cols, TermFit::Auto, false)
                .calculate(self.image.width(), self.image.height()),
            (None, Some(rows)) => SimpleAspectRatio::new_auto_width(rows, TermFit::Auto, false)
                .calculate(self.image.width(), self.image.height()),
            _ => (self.image.width(), self.image.height()),
        };
        let background = &image::Rgba([0u8; 4]);

        let x_ratio = (self.image.width() - 1) as f64 / num_cols as f64;
        let y_ratio = (self.image.height() - 1) as f64 / num_rows as f64;

        for i in 0..num_rows {
            for j in 0..num_cols {
                let sy = (i as f64 * y_ratio).round() as u32;
                let ey = (((i + 1) as f64) * y_ratio).round() as u32;
                let sx = (j as f64 * x_ratio).round() as u32;
                let ex = (((j + 1) as f64) * x_ratio).round() as u32;
                let mean = self.image.mean(sx, ex, sy, ey);
                let upper_pixel = self.image.get_pixel(sx, sy);
                let char_idx = (num_chars - 1).min(mean as usize * num_chars / 255);
                let char = self.options.char_list.chars().nth(char_idx).unwrap();
                write!(
                    writer,
                    "{}{}{}",
                    upper_pixel.foreground(),
                    background.background(),
                    char
                )?;
            }
            writeln!(writer, "{}", ANSI_BG_COLOUR_ESCAPES[0])?;
        }
        write!(writer, "{}", ANSI_RESET_ATTRIBUTES)?;

        Ok(())
    }

    fn generate_with_grayscale(&self, writer: &mut dyn Write) -> Result<(), UnicodeArtError> {
        let num_chars = self.options.char_list.len();
        let (num_cols, num_rows) = match (self.options.num_cols, self.options.num_rows) {
            (Some(cols), Some(rows)) => (cols, rows),
            (Some(cols), None) => SimpleAspectRatio::new_auto_height(cols, TermFit::Auto, false)
                .calculate(self.image.width(), self.image.height()),
            (None, Some(rows)) => SimpleAspectRatio::new_auto_width(rows, TermFit::Auto, false)
                .calculate(self.image.width(), self.image.height()),
            _ => (self.image.width(), self.image.height()),
        };

        let x_ratio = (self.image.width() - 1) as f64 / num_cols as f64;
        let y_ratio = (self.image.height() - 1) as f64 / num_rows as f64;

        for i in 0..num_rows {
            for j in 0..num_cols {
                let sy = (i as f64 * y_ratio).round() as u32;
                let ey = (((i + 1) as f64) * y_ratio).round() as u32;
                let sx = (j as f64 * x_ratio).round() as u32;
                let ex = (((j + 1) as f64) * x_ratio).round() as u32;
                // println!("sx = {}, sy = {}, ex = {}, ey = {}", sx, sy, ex, ey);
                let mean = match self.options.is_invert {
                    true => 255 - self.image.mean(sx, ex, sy, ey),
                    false => self.image.mean(sx, ex, sy, ey),
                };
                let char_idx = (num_chars - 1).min(mean as usize * num_chars / 255);
                let char = self.options.char_list.chars().nth(char_idx).unwrap();
                write!(writer, "{}", char)?
            }
            writeln!(writer)?
        }
        Ok(())
    }
}

impl UnicodeArtOption for ClassicAsciiArtOption<'static> {
    fn new_unicode_art<'a>(
        &'a self,
        image: &'a DynamicImage,
    ) -> Result<Box<(dyn UnicodeArt + 'a)>, UnicodeArtError> {
        Ok(Box::new(ClassicAsciiArt {
            options: self,
            image,
        }))
    }
}

impl<'a> ClassicAsciiArtOption<'a> {
    pub fn new_standard(num_cols: u32, is_color: bool, is_invert: bool) -> Self {
        Self {
            char_list: CHAR_LIST_STANDARD,
            num_cols: Some(num_cols),
            num_rows: None,
            is_color,
            is_invert,
        }
    }

    pub fn new_level_4(num_cols: u32, is_color: bool, is_invert: bool) -> Self {
        Self {
            char_list: CHAR_LIST_LEVELS_4,
            num_cols: Some(num_cols),
            num_rows: None,
            is_color,
            is_invert,
        }
    }

    pub fn new_level_10(num_cols: u32, is_color: bool, is_invert: bool) -> Self {
        Self {
            char_list: CHAR_LIST_LEVELS_10,
            num_cols: Some(num_cols),
            num_rows: None,
            is_color,
            is_invert,
        }
    }

    pub fn new_level_19(num_cols: u32, is_color: bool, is_invert: bool) -> Self {
        Self {
            char_list: CHAR_LIST_LEVELS_19,
            num_cols: Some(num_cols),
            num_rows: None,
            is_color,
            is_invert,
        }
    }

    pub fn new_level_16(num_cols: u32, is_color: bool, is_invert: bool) -> Self {
        Self {
            char_list: CHAR_LIST_LEVELS_16,
            num_cols: Some(num_cols),
            num_rows: None,
            is_color,
            is_invert,
        }
    }

    pub fn new_level_23(num_cols: u32, is_color: bool, is_invert: bool) -> Self {
        Self {
            char_list: CHAR_LIST_LEVELS_23,
            num_cols: Some(num_cols),
            num_rows: None,
            is_color,
            is_invert,
        }
    }
}

impl<'a> UnicodeArt for ClassicAsciiArt<'a> {
    fn write_all(&self, writer: &mut dyn Write) -> Result<(), UnicodeArtError> {
        match self.options.is_color {
            true => self.generate_with_color(writer),
            false => self.generate_with_grayscale(writer),
        }
    }
}

#[cfg(test)]
mod tests {
    use image::io::Reader as ImageReader;
    use std::io::BufWriter;

    use super::*;

    #[test]
    fn test_generate_level_19() {
        let image_path = "tests/support/test_gundam.png";
        let image = ImageReader::open(image_path);
        let art = ClassicAsciiArtOption::new_level_19(20, false, false)
            .new_unicode_art(&image.unwrap().decode().unwrap())
            .unwrap();
        let mut buf = BufWriter::new(Vec::new());
        let _ = art.write_all(&mut buf);
        let bytes = buf.into_inner().unwrap();
        let actual = String::from_utf8(bytes).unwrap();

        assert_eq!(
            r#"BBBBBBBBBBBBBBBBBBBB
BBBBBBBBBQQQBBBBBBBB
BBBBBQBQBQQQBBBBBBBB
BBBBQQRQBBBBBRQBQBBB
BBBRBBQBBBBBBBOBRBBB
BBQRBBBBBBBBBBBBRQBB
BBRQQBBBBBQBBQRQQQBB
BBBQRRQRQQRQBQQQRBBB
BBBBHRRBBQBBBBRQBBBB
BBBBQRQBBQQQBBBBBBBB
BBBBRORQBBRBBBBBBBBB
BBBBBBQQBBBBBBBBBBBB
BBBBQBBBBBBBBBBBBBBB
BBBBBBBBBBBBBBBBBBBB
BBBBBBBBBBBBBBBBBBBB
"#,
            actual
        );
    }

    #[test]
    fn test_generate_standard() {
        let image_path = "tests/support/test_gundam.png";
        let image = ImageReader::open(image_path);
        let art = ClassicAsciiArtOption::new_standard(20, false, false)
            .new_unicode_art(&image.unwrap().decode().unwrap())
            .unwrap();
        let mut buf = BufWriter::new(Vec::new());
        let _ = art.write_all(&mut buf);
        let bytes = buf.into_inner().unwrap();
        let actual = String::from_utf8(bytes).unwrap();

        assert_eq!(
            r#"&%$$$$$@8%@$B$$$@B%B
$W%$$BWW%o*M8%B%B$$$
$@8%W#8M@M*MW&8B%B$$
$$&&*ok#&B@@BhM&#$$$
$$Bh@8#8B@$$%$d&bB$$
$$MbW%8@$$B8$WBMa#$$
$$aoo%@8M&#&8Mko*M$$
$$@okk*a#oaMBoM#b$$$
$$$@wkh$$M&&@&hMB$$$
$$$$*hM8%oo*$$@@$$$$
$$$8aba#$@k&@@$$$$$$
$$$W&$##&&&@B$$$$$$$
$$&&*M88W%@$$$$$$$$$
$$$$$$8BW@$$$$$$$$$$
$$$$$@@$$$$$$$$$$$$$
"#,
            actual
        );
    }

    #[test]
    fn test_generate_level_10() {
        let image_path = "tests/support/test_gundam.png";
        let image = ImageReader::open(image_path);
        let art = ClassicAsciiArtOption::new_level_10(20, false, false)
            .new_unicode_art(&image.unwrap().decode().unwrap())
            .unwrap();
        let mut buf = BufWriter::new(Vec::new());
        let _ = art.write_all(&mut buf);
        let bytes = buf.into_inner().unwrap();
        let actual = String::from_utf8(bytes).unwrap();

        assert_eq!(
            r#"@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@%%%@@@@@@@@
@@@@@%@%@%%%@@@@@@@@
@@@@%%%%@@@@@%%@%@@@
@@@%@@%@@@@@@@#@#@@@
@@%#@@@@@@@@@@@%%%@@
@@%%%@@@%@%@@%%%%%@@
@@@%%%%%%%%%@%%%#@@@
@@@@#%%@@%@@@@%%@@@@
@@@@%%%@@%%%@@@@@@@@
@@@@%#%%@@#@@@@@@@@@
@@@@@@%%@@@@@@@@@@@@
@@@@%%@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@
"#,
            actual
        );
    }
}
