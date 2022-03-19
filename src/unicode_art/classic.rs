use super::aspect_ratio::{AspectRatio, SimpleAspectRatio, TermFit};
use super::color::{AnsiColor, ANSI_RESET_ATTRIBUTES};
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
const CHAR_LIST_LEVELS_4: &'static str = "3210";
/// ANSI background colour escapes.
const ANSI_BG_COLOUR_ESCAPES: [&str; 8] = [
    "\x1B[40m", "\x1B[41m", "\x1B[42m", "\x1B[43m", "\x1B[44m", "\x1B[45m", "\x1B[46m", "\x1B[47m",
];

#[derive(Default, Clone)]
pub struct ClassicAsciiArt<'a> {
    is_color: bool,
    image_path: &'a str,
    char_list: &'static str,
    num_cols: Option<u32>,
    num_rows: Option<u32>,
}

impl<'a> ClassicAsciiArt<'a> {
    pub fn new_standard(num_cols: u32, image_path: &'a str, is_color: bool) -> Self {
        Self {
            image_path,
            char_list: CHAR_LIST_STANDARD,
            num_cols: Some(num_cols),
            num_rows: None,
            is_color,
        }
    }

    pub fn new_level_4_with_num_cols(num_cols: u32, image_path: &'a str, is_color: bool) -> Self {
        Self {
            image_path,
            char_list: CHAR_LIST_LEVELS_4,
            num_cols: Some(num_cols),
            num_rows: None,
            is_color,
        }
    }

    pub fn new_level_10(num_cols: u32, image_path: &'a str, is_color: bool) -> Self {
        Self {
            image_path,
            char_list: CHAR_LIST_LEVELS_10,
            num_cols: Some(num_cols),
            num_rows: None,
            is_color,
        }
    }

    pub fn new_level_19(num_cols: u32, image_path: &'a str, is_color: bool) -> Self {
        Self {
            image_path,
            char_list: CHAR_LIST_LEVELS_19,
            num_cols: Some(num_cols),
            num_rows: None,
            is_color,
        }
    }

    pub fn new_level_16(num_cols: u32, image_path: &'a str, is_color: bool) -> Self {
        Self {
            image_path,
            char_list: CHAR_LIST_LEVELS_16,
            num_cols: Some(num_cols),
            num_rows: None,
            is_color,
        }
    }

    pub fn new_level_23(num_cols: u32, image_path: &'a str, is_color: bool) -> Self {
        Self {
            image_path,
            char_list: CHAR_LIST_LEVELS_23,
            num_cols: Some(num_cols),
            num_rows: None,
            is_color,
        }
    }

    fn generate_with_color(
        &self,
        writer: &mut dyn Write,
        img: &DynamicImage,
    ) -> Result<(), UnicodeArtError> {
        let num_chars = self.char_list.len();
        let num_cols = self.num_cols.unwrap_or(img.width());
        let (num_cols, num_rows) = SimpleAspectRatio::new_auto_height(num_cols, TermFit::Auto, false)
            .calculate(img.width(), img.height());
        let img = img.thumbnail_exact(num_cols, num_rows);
        let (cell_width, cell_height) = (img.width() / num_cols, img.height() / num_rows);
        let background = &image::Rgba([0u8; 4]);

        for i in 0..num_rows {
            for j in 0..num_cols {
                let sy = i * cell_height;
                let ey = (i + 1) * cell_height;
                let sx = j * cell_width;
                let ex = (j + 1) * cell_width;
                let mean = img.mean(sx, ex, sy, ey);
                let upper_pixel = img.get_pixel(j, i);
                let char_idx = (num_chars - 1).min(mean as usize * num_chars / 255);
                let char = self.char_list.chars().nth(char_idx).unwrap();
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

    fn generate_with_grayscale(
        &self,
        writer: &mut dyn Write,
        img: &DynamicImage,
    ) -> Result<(), UnicodeArtError> {
        let num_chars = self.char_list.len();
        let num_cols = self.num_cols.unwrap_or(img.width());
        let (num_cols, num_rows) = SimpleAspectRatio::new_auto_height(num_cols, TermFit::Auto, false)
            .calculate(img.width(), img.height());
        let cell_width = 1.max(img.width() / num_cols);
        let cell_height = 1.max(img.height() / num_rows); 
        let num_cols = num_cols.min(img.width());
        let num_rows = num_rows.min(img.height());
        // println!("cell_width = {}, cell_width = {}, num_cols = {}, num_rows = {}, img.width = {}, img.height= {}", 
        //     cell_width, cell_height, num_cols, num_rows, img.width(), img.height());

        for i in 0..num_rows {
            for j in 0..num_cols {
                let sy = i * cell_height;
                let ey = (i + 1) * cell_height;
                let sx = j * cell_width;
                let ex = (j + 1) * cell_width;
                // println!("sx = {}, sy = {}, ex = {}, ey = {}", sx, sy, ex, ey);
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

impl<'a> UnicodeArt for ClassicAsciiArt<'a> {
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
        let art = ClassicAsciiArt::new_level_19(20, "tests/support/test_gundam.png", false);
        let mut buf = BufWriter::new(Vec::new());
        let _ = art.generate(&mut buf);
        let bytes = buf.into_inner().unwrap();
        let actual = String::from_utf8(bytes).unwrap();

        assert_eq!(
            r#"BBBBBBBBBBBBBBBBBBBB
BBBBBBBBBQQQBBBBBBBB
BBBBBQBBBBQQBBBBBBBB
BBBBQQRQBBBBBRQBQBBB
BBBRBBQBBBBBBBOBRBBB
BBQOBBBBBBBBBBBQRQBB
BBRQRBBBBBQBBQRRQQBB
BBBQRRQRQQRQBQQQRBBB
BBBBOORBBBBBBBRQBBBB
BBBBQRBBBQRQBBBBBBBB
BBBBQRRQBBOBBBBBBBBB
BBBBBBQQBBBBBBBBBBBB
BBBBQQBBQBBBBBBBBBBB
BBBBBBBBQBBBBBBBBBBB
BBBBBBBBBBBBBBBBBBBB
"#,
            actual
        );
    }

    #[test]
    fn test_generate_standard() {
        let art = ClassicAsciiArt::new_standard(20, "tests/support/test_gundam.png", false);
        let mut buf = BufWriter::new(Vec::new());
        let _ = art.generate(&mut buf);
        let bytes = buf.into_inner().unwrap();
        let actual = String::from_utf8(bytes).unwrap();

        assert_eq!(
            r#"&%$$$$$@8%@$%$$$$B%%
$W%$$BWW%#oM8%B%B$$$
$@8%W#8M@M*#W&8B%%$$
$$&&*okMWB@@@h#&#$$$
$$Bh@8M8B@$$%$d8k%$$
$$MbW%&@$$@8$&BMa#$$
$$a*aB@8M&#88Mka#o$$
$$@okhMa#ohMBo#ohB$$
$$$@qbb$$WW&$8b#8$$$
$$$$*hW8B*a*@$@@$$$$
$$$8obh#@$bW@@$$$$$$
$$$&W$*MB&M$%$$$$$$$
$$&&oMB&M%@$$$$$$$$$
$$$$$$WBM@$$$$$$$$$$
$$$$$@@$$$$$$$$$$$$$
"#,
            actual
        );
    }

    #[test]
    fn test_generate_level_10() {
        let art = ClassicAsciiArt::new_level_10(20, "tests/support/test_gundam.png", false);
        let mut buf = BufWriter::new(Vec::new());
        let _ = art.generate(&mut buf);
        let bytes = buf.into_inner().unwrap();
        let actual = String::from_utf8(bytes).unwrap();

        assert_eq!(
            r#"@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@%%%@@@@@@@@
@@@@@%@%@%%%@@@@@@@@
@@@@%%#%@@@@@%%@%@@@
@@@%@@%@@@@@@@#@#@@@
@@%#@@@@@@@@@@@%%%@@
@@%%%@@@%@%@@%%%%%@@
@@@%#%%%%%%%@%%%%@@@
@@@@###@@@@@@@#%@@@@
@@@@%%@@@%%%@@@@@@@@
@@@@%#%%@@#@@@@@@@@@
@@@@@@%%@@%@@@@@@@@@
@@@@%%@@%@@@@@@@@@@@
@@@@@@@@%@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@
"#,
            actual
        );
    }
}
