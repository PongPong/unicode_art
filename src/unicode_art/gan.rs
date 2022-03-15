use super::error::UnicodeArtError;
use super::UnicodeArt;
use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImageView};
use std::io::Write;
use tch::{nn, nn::Module, nn::OptimizerConfig, Device};

pub struct GanUnicodeArt {
    char_list: &'static str,
    num_cols: u32,
}

impl GanUnicodeArt {
    pub fn new(num_cols: u32) -> Self {
        SimpleAsciiUnicodeArt {
            char_list: CHAR_LIST_LEVELS_16,
            num_cols,
        }
    }
}

impl UnicodeArt for GanUnicodeArt {
    fn generate<W: ?Sized>(&self, image_path: &str, writer: &mut W) -> Result<(), UnicodeArtError>
    where
        W: Write,
    {
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
//             actual
//         );
    }
}
