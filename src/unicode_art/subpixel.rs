use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::thread;

use super::classic::ClassicAsciiArtOption;
use super::error::UnicodeArtError;
use super::{UnicodeArt, UnicodeArtOption};
use clap::lazy_static::lazy_static;
use image::DynamicImage;
use itertools::Itertools;

// TODO: could be replace with lhs.abs_diff(rhs) later on.
#[inline]
fn abs_diff(slf: u32, other: u32) -> u32 {
    if slf < other {
        other - slf
    } else {
        slf - other
    }
}

#[derive(Debug)]
pub struct SubpixelUnicodeArtOption<'a> {
    num_cols: u32,
    letters: &'a HashMap<u32, [u8; 9]>,
    grid_size: usize,
    is_invert: bool,
}

pub struct SubpixelUnicodeArt<'a> {
    options: &'a SubpixelUnicodeArtOption<'a>,
    image: &'a DynamicImage,
}

impl<'a> SubpixelUnicodeArtOption<'a> {
    pub fn new(num_cols: u32, is_invert: bool) -> Self {
        Self {
            num_cols,
            letters: &LETTER3,
            grid_size: 3,
            is_invert,
        }
    }
}

impl UnicodeArtOption for SubpixelUnicodeArtOption<'static> {
    fn new_unicode_art<'a>(
        &'a self,
        image: &'a DynamicImage,
    ) -> Result<Box<(dyn UnicodeArt + 'a)>, UnicodeArtError> {
        Ok(Box::new(SubpixelUnicodeArt {
            options: self,
            image,
        }))
    }
}

impl<'a> SubpixelUnicodeArt<'a> {
    /**
     * convert classic ascii with 3210 chars list to subpixel
     * in: 360 * 136
     * out: 121 * 46
     */
    fn convert(&self, input: &mut dyn Read, output: &mut dyn Write) -> Result<(), UnicodeArtError> {
        let buf_reader = BufReader::new(input);
        let total_size = self.options.grid_size * self.options.grid_size;
        for lines in &buf_reader.lines().chunks(self.options.grid_size) {
            let lines: Vec<_> = lines.map(|l| l.unwrap()).collect();
            // each column
            for i in (0..lines[0].len()).step_by(self.options.grid_size) {
                let mut block = String::with_capacity(9);
                for line in lines.iter().rev() {
                    // every 3 rows
                    block.push_str(&line[i..line.len().min(i + self.options.grid_size)]);
                    if i + self.options.grid_size > line.len() {
                        block.push_str(&"0000000000"[0..i + self.options.grid_size - line.len()]);
                    }
                }
                if block.len() < total_size {
                    block.push_str(&"0000000000"[0..total_size - block.len()]);
                }
                let block_final: Vec<u32> =
                    block.chars().map(|b| b.to_digit(10).unwrap()).collect();
                if let Some(letter) = self.distance(&block_final) {
                    write!(output, "{}", char::from_u32(letter).unwrap())?;
                    // write!(output, "&#{};", letter)?;
                }
            }
            writeln!(output)?;
        }
        Ok(())
    }

    fn distance<'b>(&self, y: &'b Vec<u32>) -> Option<u32> {
        let mut distances = HashMap::new();

        for (&key, a) in self.options.letters.iter() {
            let mut cur_distance = 0;
            for (index, &aa) in a.iter().enumerate() {
                let bb = y[index];
                cur_distance += abs_diff(aa as u32, bb);
            }
            distances.insert(key, cur_distance);
        }

        let min = distances
            .iter()
            .reduce(|a, b| if a.1 < b.1 { a } else { b })
            .map(|a| *a.0);
        min
    }
}

impl<'a> UnicodeArt for SubpixelUnicodeArt<'a> {
    fn write_all(&self, writer: &mut dyn Write) -> Result<(), UnicodeArtError> {
        let (mut read, mut write) = pipe::pipe();
        let num_cols = self.options.num_cols;
        let is_invert = self.options.is_invert;
        let image = self.image.clone();
        let handler = thread::spawn(move || {
            let _ = ClassicAsciiArtOption::new_level_4(num_cols * 3, false, is_invert)
                .new_unicode_art(&image)
                .unwrap()
                .write_all(&mut write);
        });
        let _ = self.convert(&mut read, writer);
        let _ = handler.join().expect("error");
        Ok(())
    }
}

// #[cfg(test)]
// mod test {
//     use std::io::BufWriter;
//
//     use fontdue::Font;
//     use image::GrayImage;
//
//     use crate::unicode_art::classic::{ClassicAsciiArt, CHAR_LIST_LEVELS_4};
//
//     use super::*;
//
//     #[test]
//     fn test_generate_subpixel() {
//         let mut buf = BufWriter::new(Vec::new());
//         let _ = SubpixelUnicodeArtOption::new(100, "tests/support/test_gundam2.png", false)
//             .new_unicode_art()
//             .unwrap()
//             .write_all(&mut buf);
//         let bytes = buf.into_inner().unwrap();
//         let actual = String::from_utf8(bytes).unwrap();
//
//         println!("{}", actual);
//     }
//
//     #[test]
//     fn test_generate_font_matrix() -> Result<(), UnicodeArtError> {
//         fn print_range(start: u32, end: u32, font: &Font) {
//             for i in start..end + 1 {
//                 // Rasterize and get the layout metrics for the letter 'g' at 17px.
//                 let char = match char::from_u32(i as u32) {
//                     Some(c) => c,
//                     _ => continue,
//                 };
//                 let idx = font.lookup_glyph_index(char);
//                 if idx == 0 {
//                     continue;
//                 }
//                 let (metrics, bitmap) = font.rasterize(char, 17.0);
//                 if bitmap.len() == 0 {
//                     println!(
//                         "/*000000000*/ ({}, [0,0,0,0,0,0,0,0,0]), // {}",
//                         start,
//                         char::from_u32(start).unwrap()
//                     );
//                     continue;
//                 }
//                 // println!("bitmap = {:?}", bitmap.len());
//                 // println!("metrics = {:?}", metrics);
//                 let image =
//                     GrayImage::from_raw(metrics.width as u32, metrics.height as u32, bitmap)
//                         .unwrap();
//                 let art = ClassicAsciiArt {
//                     image: image::DynamicImage::ImageLuma8(image),
//                     options: ClassicAsciiArtOption {
//                         is_color: false,
//                         image_path: "",
//                         char_list: CHAR_LIST_LEVELS_4,
//                         num_cols: Some(3),
//                         num_rows: Some(3),
//                         is_invert: true,
//                     },
//                 };
//                 let mut buf = BufWriter::new(Vec::new());
//                 let _ = art.write_all(&mut buf);
//                 let bytes = buf.into_inner().unwrap();
//                 {
//                     let v = String::from_utf8(bytes.to_vec().to_owned()).unwrap();
//                     let v = v.split("\n").join("");
//                     let inv_nums = v.chars().map(|c| (c as u8 - (b'0' as u8))).collect_vec();
//                     let sum: u8 = inv_nums.iter().sum();
//                     // let inv_nums = v.chars().map(|c| (c as u8 - (b'0' as u8))).collect_vec();
//                     println!(
//                         "/*{}*/ ({}, [{},{},{},{},{},{},{},{},{}]), // {}",
//                         sum,
//                         i,
//                         inv_nums[0],
//                         inv_nums[1],
//                         inv_nums[2],
//                         inv_nums[3],
//                         inv_nums[4],
//                         inv_nums[5],
//                         inv_nums[6],
//                         inv_nums[7],
//                         inv_nums[8],
//                         char,
//                     );
//                 }
//             }
//         }
//
//         let font = include_bytes!(
//             // "/Users/clam/Library/Fonts/Hack Bold Italic Nerd Font Complete Mono.ttf"
//             "/Users/lampo/Library/Fonts/Sauce Code Pro Nerd Font Complete Mono.ttf"
//         ) as &[u8];
//         // Parse it into the font type.
//         let font = &fontdue::Font::from_bytes(font, fontdue::FontSettings::default()).unwrap();
//         print_range(0x20, 0x7A, &font); // ascii table
//         // print_range(0x2580, 0x259F, &font);
//         print_range(0x2500, 0x257F, &font);
//         // Read the font data.
//         // for (key, _) in LETTER3.iter() {
//         //     print_range(*key, *key + 1, font);
//         // }
//         Ok(())
//     }
// }

lazy_static! {
    static ref LETTER3: HashMap<u32, [u8; 9]> = [
/*000000000*/ (32, [0,0,0,0,0,0,0,0,0]), //
/*10*/ (100, [0,0,1,1,1,2,2,1,2]), // d
/*10*/ (33, [0,1,2,0,1,2,0,2,2]), // !
/*10*/ (54, [1,1,1,2,1,1,1,1,1]), // 6
/*10*/ (66, [1,1,1,2,1,2,1,0,1]), // B
/*10*/ (9485, [2,2,2,2,0,0,2,0,0]), // ┍
/*11*/ (101, [1,1,1,2,1,2,1,1,1]), // e
/*11*/ (61, [2,3,3,0,0,0,1,1,1]), // =
/*11*/ (70, [2,1,1,3,1,1,2,0,0]), // F
/*11*/ (72, [2,0,0,3,1,2,2,0,1]), // H
/*11*/ (77, [2,0,1,2,1,2,2,1,0]), // M
/*12*/ (58, [0,3,3,0,0,0,0,3,3]), // :
/*12*/ (9482, [1,1,2,1,1,2,1,1,2]), // ┊
/*12*/ (9499, [0,0,3,0,0,3,1,2,3]), // ┛
/*12*/ (9550, [2,2,2,0,0,0,2,2,2]), // ╎
/*12*/ (9551, [0,3,3,0,0,0,0,3,3]), // ╏
/*13*/ (44, [0,2,3,0,1,3,0,2,2]), // ,
/*13*/ (9476, [2,2,1,2,2,1,1,1,1]), // ┄
/*13*/ (9487, [2,3,2,2,1,0,2,1,0]), // ┏
/*13*/ (9491, [2,2,3,0,0,3,0,0,3]), // ┓
/*14*/ (9479, [0,3,3,0,2,2,0,2,2]), // ┇
/*15*/ (46, [0,1,2,0,3,3,0,3,3]), // .
/*15*/ (9480, [2,2,2,2,2,2,1,1,1]), // ┈
/*15*/ (9597, [0,1,2,0,3,3,0,3,3]), // ╽
/*16*/ (9483, [0,2,2,0,3,3,0,3,3]), // ┋
/*16*/ (9549, [1,0,1,3,1,3,3,1,3]), // ╍
/*16*/ (9593, [0,2,2,0,3,3,0,3,3]), // ╹
/*17*/ (9477, [1,1,1,3,2,2,3,2,2]), // ┅
/*17*/ (9596, [0,0,1,2,3,3,2,3,3]), // ╼
/*17*/ (9599, [0,3,3,0,3,3,0,2,3]), // ╿
/*18*/ (9474, [1,1,2,2,2,3,2,2,3]), // │
/*18*/ (9475, [0,3,3,0,3,3,0,3,3]), // ┃
/*18*/ (9589, [1,1,2,2,2,3,2,2,3]), // ╵
/*18*/ (9595, [0,3,3,0,3,3,0,3,3]), // ╻
/*19*/ (9481, [1,1,1,2,3,3,2,3,3]), // ┉
/*19*/ (9598, [1,1,0,3,3,3,3,3,2]), // ╾
/*2*/ (9496, [0,0,0,0,0,1,0,0,1]), // ┘
/*2*/ (9508, [0,0,0,0,0,1,0,0,1]), // ┤
/*20*/ (9591, [2,2,2,2,2,3,2,2,3]), // ╷
/*21*/ (39, [2,3,3,2,3,3,1,2,2]), // '
/*21*/ (9473, [1,1,1,3,3,3,3,3,3]), // ━
/*21*/ (95, [3,3,3,3,3,3,1,1,1]), // _
/*21*/ (9553, [3,1,3,3,1,3,3,1,3]), // ║
/*21*/ (9588, [2,3,3,2,3,3,1,2,2]), // ╴
/*22*/ (9472, [2,3,3,2,3,3,2,2,2]), // ─
/*3*/ (47, [0,0,1,0,1,0,0,1,0]), // /
/*3*/ (92, [0,0,0,0,1,0,0,0,2]), // \
/*3*/ (93, [0,0,1,0,0,1,0,0,1]), // ]
/*3*/ (9585, [0,0,1,0,1,0,1,0,0]), // ╱
/*3*/ (9586, [1,0,0,0,1,0,0,0,1]), // ╲
/*4*/ (9524, [0,1,0,0,1,0,0,2,0]), // ┴
/*4*/ (9532, [0,1,0,0,2,0,0,1,0]), // ┼
/*4*/ (9583, [0,0,0,0,0,1,0,1,2]), // ╯
/*5*/ (106, [0,0,1,0,0,2,0,0,2]), // j
/*5*/ (121, [1,0,1,0,1,1,0,1,0]), // y
/*5*/ (62, [1,0,0,0,0,2,1,1,0]), // >
/*5*/ (84, [0,2,1,0,1,0,0,1,0]), // T
/*6*/ (105, [0,0,1,0,1,2,0,0,2]), // i
/*6*/ (36, [0,1,1,0,1,1,0,1,1]), // $
/*6*/ (50, [1,1,1,0,0,2,0,1,0]), // 2
/*6*/ (51, [0,1,1,0,0,2,0,1,1]), // 3
/*6*/ (73, [0,2,1,0,1,0,0,2,0]), // I
/*7*/ (114, [1,1,1,2,0,0,2,0,0]), // r
/*7*/ (122, [1,1,2,0,1,0,1,1,0]), // z
/*7*/ (41, [0,1,1,0,0,2,0,1,2]), // )
/*7*/ (43, [0,1,0,1,2,2,0,1,0]), // +
/*7*/ (49, [0,3,0,0,2,0,0,2,0]), // 1
/*8*/ (113, [1,1,1,2,0,1,0,1,1]), // q
/*8*/ (120, [1,0,1,0,3,0,1,1,1]), // x
/*8*/ (37, [1,1,0,1,1,1,1,1,1]), // %
/*8*/ (52, [0,1,1,0,1,1,1,1,2]), // 4
/*8*/ (79, [1,1,1,1,0,1,1,1,1]), // O
/*9*/ (104, [2,0,0,2,1,1,2,0,1]), // h
/*9*/ (112, [1,1,1,2,0,1,2,1,0]), // p
/*9*/ (35, [0,0,1,0,2,2,1,1,2]), // #
/*9*/ (48, [1,1,1,2,1,0,1,1,1]), // 0
/*9*/ (80, [1,1,1,2,0,2,2,0,0]), // P
    ]
    .iter()
    .copied()
    .collect();
}
