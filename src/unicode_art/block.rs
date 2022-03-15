use super::error::UnicodeArtError;
use super::mean::Mean;
use super::UnicodeArt;
use image::io::Reader as ImageReader;
use image::GenericImageView;
use std::io::Write;

/// ANSI background colour escapes.
const ANSI_BG_COLOUR_ESCAPES: [&str; 8] = [
    "\x1B[40m", "\x1B[41m", "\x1B[42m", "\x1B[43m", "\x1B[44m", "\x1B[45m", "\x1B[46m", "\x1B[47m",
];

#[derive(Default, Clone)]
pub struct BlockUnicodeArt<'a> {
    image_path: &'a str,
    char_list: &'static str,
    num_cols: u32,
}

impl<'a> BlockUnicodeArt<'a> {
    pub fn is_color(&mut self, enable: bool) {
        self.is_color = enable;
    }

    pub fn new(num_cols: u32, image_path: &'a str) -> Self {
        Self {
            image_path,
            char_list: CHAR_LIST_LEVELS_10,
            num_cols,
            ..Default::default()
        }
    }
}

impl<'a> UnicodeArt for BlockUnicodeArt<'a> {
    fn generate(&self, writer: &mut dyn Write) -> Result<(), UnicodeArtError> {
        let num_chars = self.char_list.len();
        let img = ImageReader::open(self.image_path)
            .map_err(|err| UnicodeArtError::from(err))?
            .decode()
            .map_err(|err| UnicodeArtError::from(err))?;
        let (width, height) = (img.width(), img.height());
        let img = img.thumbnail(
            self.num_cols,
            ((self.num_cols as f64 / width as f64) * height as f64) as u32,
        );
        let (num_rows, num_cols) = (img.height() / 2, img.width());

        for y in 0..num_rows {
            let upper_y = y * 2;
            let lower_y = upper_y + 1;
            for x in 0..num_cols {
                let upper_pixel = img.get_pixel(x, upper_y);
                let lower_pixel = img.get_pixel(x, lower_y);
                let mean =
                    (upper_pixel[0] as u32 + upper_pixel[1] as u32 + upper_pixel[2] as u32) / 3;
                let char_idx = (num_chars - 1).min(mean as usize * num_chars / 255);
                let char = self.char_list.chars().nth(char_idx).unwrap();
                write!(writer,
                       "\x1B[38;2;{};{};{}m\
                        \x1B[48;2;{};{};{}m\u{2580}", // ▀
                       upper_pixel[0],
                       upper_pixel[1],
                       upper_pixel[2],
                       lower_pixel[0],
                       lower_pixel[1],
                       lower_pixel[2])?;
            }
            writeln!(writer, "{}", ANSI_BG_COLOUR_ESCAPES[0])?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::BufWriter;

    use super::*;

    #[test]
    fn test_generate_level_19() {
        let art = SimpleAsciiUnicodeArt::new_level_19(20, "tests/support/test_gundam.png");
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
        let art = SimpleAsciiUnicodeArt::new_standard(20, "tests/support/test_gundam.png");
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
        let art = SimpleAsciiUnicodeArt::new_level_10(20, "tests/support/test_gundam.png");
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
