use super::error::UnicodeArtError;
use super::UnicodeArt;
use super::mean::Mean;
use image::io::Reader as ImageReader;
use std::io::Write;

const CHAR_LIST_STANDARD: &'static str =
    "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/|()1{}[]?-_+~<>i!lI;:,\"^`'. ";
const CHAR_LIST_LEVELS_10: &'static str = "@%#*+=-:. ";
const CHAR_LIST_LEVELS_19: &'static str = "BBQROHETI)7ri=+;:,.";
const CHAR_LIST_LEVELS_16: &'static str = "#8XOHLTI)i=+;:,.";

pub struct SimpleAsciiUnicodeArt {
    char_list: &'static str,
    num_cols: u32,
}

impl SimpleAsciiUnicodeArt {
    pub fn new_level_10(num_cols: u32) -> Self {
        SimpleAsciiUnicodeArt {
            char_list: CHAR_LIST_LEVELS_10,
            num_cols,
        }
    }

    pub fn new_standard(num_cols: u32) -> Self {
        SimpleAsciiUnicodeArt {
            char_list: CHAR_LIST_STANDARD,
            num_cols,
        }
    }

    pub fn new_level_19(num_cols: u32) -> Self {
        SimpleAsciiUnicodeArt {
            char_list: CHAR_LIST_LEVELS_19,
            num_cols,
        }
    }

    pub fn new_level_16(num_cols: u32) -> Self {
        SimpleAsciiUnicodeArt {
            char_list: CHAR_LIST_LEVELS_16,
            num_cols,
        }
    }

}

impl UnicodeArt for SimpleAsciiUnicodeArt {
    fn generate(&self, image_path: &str, writer: &mut dyn Write) -> Result<(), UnicodeArtError>
    {
        let num_chars = self.char_list.len();
        let img = ImageReader::open(image_path)
            .map_err(|err| UnicodeArtError::from(err))?
            .decode()
            .map_err(|err| UnicodeArtError::from(err))?;
        let img = img.grayscale();
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
                // let char_idx = (num_chars - 1).min(mean as usize * num_chars / 255);
                let char_idx = mean as usize * num_chars / 255;
                let char = self.char_list.chars().nth(char_idx).unwrap();
                write!(writer, "{}", char)?
            }
            writeln!(writer)?
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
        let art = SimpleAsciiUnicodeArt::new_level_19(20);
        let mut buf = BufWriter::new(Vec::new());
        let _ = art.generate("tests/support/test_gundam.png", &mut buf);
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
        let art = SimpleAsciiUnicodeArt::new_standard(20);
        let mut buf = BufWriter::new(Vec::new());
        let _ = art.generate("tests/support/test_gundam.png", &mut buf);
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
        let art = SimpleAsciiUnicodeArt::new_level_10(20);
        let mut buf = BufWriter::new(Vec::new());
        let _ = art.generate("tests/support/test_gundam.png", &mut buf);
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
