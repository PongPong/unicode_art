use super::error::UnicodeArtError;
use super::UnicodeArt;
use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImageView};

const CHAR_LIST_STANDARD: &'static str =
    "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/|()1{}[]?-_+~<>i!lI;:,\"^`'. ";
const CHAR_LIST_LEVELS_10: &'static str = "@%#*+=-:. ";
const CHAR_LIST_LEVELS_19: &'static str =
    "BBQROHETI)7ri=+;:,.";
const CHAR_LIST_LEVELS_16: &'static str =
    "#8XOHLTI)i=+;:,.";

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

    fn mean(&self, img: &DynamicImage, sx: u32, ex: u32, sy: u32, ey: u32) -> u8 {
        let sub_image = img.view(sx, sy, ex - sx, ey - sy);
        let sub_image = sub_image.to_image();

        let len = sub_image.pixels().len();
        debug_assert_ne!(len,0);
        let sum = sub_image.pixels().fold(0u32, |mut sum, &pixel| {
            let image::Rgba(data) = pixel;
            sum += data[0] as u32 + data[1] as u32 + data[2] as u32;
            sum
        });
        (sum / 3 / len  as u32) as u8
    }
}

impl UnicodeArt for SimpleAsciiUnicodeArt {
    fn generate(&self, image_path: &str) -> Result<(), UnicodeArtError> {
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
                let mean = self.mean(&img, sx, ex, sy, ey);
                // let char_idx = (num_chars - 1).min(mean as usize * num_chars / 255);
                let char_idx = mean as usize * num_chars / 255;
                let char = self.char_list.chars().nth(char_idx).unwrap();
                print!("{}", char);
            }
            println!();
        }
        println!();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_level_19() {
        let art = SimpleAsciiUnicodeArt::new_level_19(80);
        let _ = art.generate("tests/support/test_gundam.png");
    }

    //#[test]
    fn test_generate_standard() {
        let art = SimpleAsciiUnicodeArt::new_standard(80);
        // let _ = art.generate("tests/support/test_gundam.png");
    }

    //#[test]
    fn test_generate_level_10() {
        let art = SimpleAsciiUnicodeArt::new_level_10(80);
        // let _ = art.generate("tests/support/test_gundam.png");

    }

}
