use std::io::Write;

use super::{error::UnicodeArtError, UnicodeArt};
use image::{io::Reader as ImageReader, GenericImageView};

// Braille symbol is 2x4 dots
const X_DOTS: u8 = 2;
const Y_DOTS: u8 = 4;

pub const DEFAULT_THRESHOLD: u8 = 127;

pub struct BrailleAsciiArt<'a> {
    image_path: &'a str,
    threshold: u8, // range 0 - 255
    num_cols: u32,
}

impl<'a> BrailleAsciiArt<'a> {
    pub fn new(num_cols: u32, image_path: &'a str, threshold: u8) -> Self {
        BrailleAsciiArt {
            image_path,
            threshold,
            num_cols,
        }
    }
}

fn resize_dimensions(width: u32, height: u32, nwidth: u32, nheight: u32, fill: bool) -> (u32, u32) {
    let wratio = nwidth as f64 / width as f64;
    let hratio = nheight as f64 / height as f64;

    let ratio = if fill {
        f64::max(wratio, hratio)
    } else {
        f64::min(wratio, hratio)
    };

    let nw = 1.max((width as f64 * ratio).round() as u64);
    let nh = 1.max((height as f64 * ratio).round() as u64);

    if nw > u64::from(u32::MAX) {
        let ratio = u32::MAX as f64 / width as f64;
        (u32::MAX, 1.max((height as f64 * ratio).round() as u32))
    } else if nh > u64::from(u32::MAX) {
        let ratio = u32::MAX as f64 / height as f64;
        (1.max((width as f64 * ratio).round() as u32), u32::MAX)
    } else {
        (nw as u32, nh as u32)
    }
}

impl<'a> UnicodeArt for BrailleAsciiArt<'a> {
    fn generate(&self, writer: &mut dyn Write) -> Result<(), UnicodeArtError> {
        let img = ImageReader::open(self.image_path)
            .map_err(|err| UnicodeArtError::from(err))?
            .decode()
            .map_err(|err| UnicodeArtError::from(err))?;

        let (width, height) = resize_dimensions(
            img.width(),
            img.height(),
            self.num_cols * X_DOTS as u32,
            ::std::u32::MAX,
            false,
        );
        let img = img.thumbnail(width, height);
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
                    .map(|dot| ((dot[0] as u32 + dot[1] as u32 + dot[2] as u32) / 3) as u8)
                    .map(|grey| (grey < self.threshold) as u8);
                let mut dec = 0;
                for (i, bit_num) in bits.iter().enumerate() {
                    dec += *bit_num as u32 * u32::pow(2, i as u32)
                }
                // Braille Unicode range starts at U2800 (= 10240 decimal)
                write!(writer, "{}", char::from_u32(dec + 10240).unwrap())?;
            }
            writeln!(writer)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufWriter;

    #[test]
    fn test_generate_braille() {
        let art = BrailleAsciiArt::new(40, "tests/support/test_gundam.png", 12);
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
