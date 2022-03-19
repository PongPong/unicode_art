use std::collections::HashMap;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::thread;

use super::classic::ClassicAsciiArt;
use super::error::UnicodeArtError;
use super::UnicodeArt;
use clap::lazy_static::lazy_static;
use itertools::Itertools;

#[derive(Debug)]
pub struct SubpixelUnicodeArt<'a> {
    image_path: &'a str,
    num_cols: u32,
    letters: &'a HashMap<u32, [u8; 9]>,
    grid_size: usize,
}

// TODO: could be replace with lhs.abs_diff(rhs) later on.
fn abs_diff(slf: u32, other: u32) -> u32 {
    if slf < other {
        other - slf
    } else {
        slf - other
    }
}

impl<'a> SubpixelUnicodeArt<'a> {
    pub fn new(num_cols: u32, image_path: &'a str) -> Self {
        Self {
            image_path,
            num_cols,
            letters: &LETTER3,
            grid_size: 3 
        }
    }

    /**
     * convert classic ascii with 3210 chars list to subpixel
     */
    fn convert(&self, input: &mut dyn Read, output: &mut dyn Write) -> Result<(), UnicodeArtError> {
        let buf_reader = BufReader::new(input);
        let total_size = self.grid_size * self.grid_size;
        let mut r = 0;
        let mut c = 0;
        for lines in &buf_reader.lines().chunks(self.grid_size) {
            let lines: Vec<_> = lines.map(|l| l.unwrap()).collect();
            // each column
            for i in (0..lines[0].len()).step_by(self.grid_size) {
                let mut block = String::new();
                c = lines[0].len();
                for line in lines.iter().rev() {
                    // every 3 rows
                    block.push_str(&line[i..line.len().min(i + self.grid_size)]);
                    if i + self.grid_size > line.len() {
                        block.push_str(&"0000000000"[0..i + self.grid_size - line.len()]);
                    }
                }
                if block.len() < total_size {
                    block.push_str(&"0000000000"[0..total_size - block.len()]);
                }
                let block_final: Vec<u32> =
                    block.chars().map(|b| b.to_digit(10).unwrap()).collect();
                if let Some(letter) = self.distance(&block_final) {
                    write!(output, "{}", char::from_u32(letter).unwrap())?;
                }
            }
            writeln!(output)?;
            r+=1;
        }
        println!("r = {}, c = {}", r, c);
        Ok(())
    }

    fn distance<'b>(&self, y: &'b Vec<u32>) -> Option<u32> {
        let mut distances = HashMap::new();

        for (&key, a) in self.letters.iter() {
            let mut cur_distance = 0;
            let b = y;
            for (index, &aa) in a.iter().enumerate() {
                let bb = b[index];
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
    fn generate(&self, writer: &mut dyn Write) -> Result<(), UnicodeArtError> {
        let (mut read, mut write) = pipe::pipe();
        let image_path = self.image_path.to_string();
        let num_cols = self.num_cols;
        let handler = thread::spawn(move || {
            let gen =
                ClassicAsciiArt::new_level_4_with_num_cols(num_cols * 3, image_path.as_str(), false);
            gen.generate(&mut write)
        });
        let _ = self.convert(&mut read, writer);
        let _ = handler.join().expect("error");
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generate_subpixel() {
        let gen = SubpixelUnicodeArt::new(400, "tests/support/test_gundam.jpeg");
        let mut buf = BufWriter::new(Vec::new());
        let _ = gen.generate(&mut buf);
        let bytes = buf.into_inner().unwrap();
        let actual = String::from_utf8(bytes).unwrap();

        println!("{}", actual);
    }
}

lazy_static! {
    static ref LETTER3: HashMap<u32, [u8; 9]> = [
        (1008640, [0, 1, 0, 2, 2, 2, 0, 1, 0]),
        (8706, [0, 1, 0, 2, 2, 2, 1, 2, 2]),
        (34, [0, 0, 0, 0, 0, 0, 1, 2, 1]),
        (1012849, [0, 0, 0, 1, 2, 2, 1, 1, 1]),
        (9733, [0, 0, 0, 1, 3, 1, 0, 1, 0]),
        (8364, [0, 1, 1, 2, 3, 1, 1, 2, 1]),
        (1011752, [0, 2, 0, 0, 2, 0, 0, 0, 0]),
        (173, [0, 0, 0, 1, 2, 1, 0, 0, 0]),
        (8593, [0, 0, 0, 0, 2, 0, 1, 3, 1]),
        (8721, [1, 1, 1, 1, 2, 0, 1, 2, 1]),
        (8722, [0, 0, 0, 1, 2, 1, 0, 0, 0]),
        (8211, [0, 0, 0, 1, 2, 1, 0, 0, 0]),
        (8212, [0, 0, 0, 2, 2, 2, 0, 0, 0]),
        (994020, [0, 0, 0, 0, 0, 0, 1, 1, 1]),
        (601, [0, 1, 0, 1, 2, 2, 0, 1, 0]),
        (536, [1, 2, 0, 1, 2, 2, 1, 2, 1]),
        (537, [0, 2, 0, 1, 2, 1, 0, 1, 1]),
        (538, [0, 2, 0, 0, 3, 0, 1, 3, 1]),
        (539, [0, 2, 0, 1, 2, 0, 1, 2, 1]),
        (8220, [0, 0, 0, 0, 0, 0, 1, 1, 2]),
        (8221, [0, 0, 0, 0, 0, 0, 1, 2, 2]),
        (8222, [1, 1, 1, 0, 1, 1, 0, 0, 0]),
        (8224, [0, 2, 0, 1, 3, 1, 1, 2, 1]),
        (33, [0, 1, 0, 0, 2, 0, 0, 3, 0]),
        (8226, [0, 0, 0, 1, 3, 1, 0, 0, 0]),
        (35, [1, 1, 0, 2, 2, 1, 1, 2, 2]),
        (1008676, [0, 2, 0, 1, 2, 1, 0, 1, 0]),
        (37, [1, 1, 1, 1, 3, 1, 2, 2, 1]),
        (38, [1, 1, 1, 2, 2, 2, 1, 2, 0]),
        (39, [0, 0, 0, 0, 0, 0, 0, 2, 0]),
        (40, [0, 2, 0, 1, 2, 0, 0, 2, 0]),
        (41, [0, 2, 0, 0, 2, 1, 0, 2, 0]),
        (43, [0, 1, 0, 1, 3, 1, 0, 1, 0]),
        (44, [0, 2, 0, 0, 1, 0, 0, 0, 0]),
        (45, [0, 0, 0, 1, 2, 1, 0, 0, 0]),
        (983263, [1, 1, 1, 2, 2, 2, 2, 2, 1]),
        (47, [2, 1, 0, 0, 2, 0, 0, 1, 2]),
        (48, [0, 1, 0, 2, 2, 2, 2, 2, 2]),
        (49, [1, 1, 1, 0, 2, 1, 1, 2, 0]),
        (50, [1, 1, 1, 0, 2, 1, 1, 2, 1]),
        (563, [0, 2, 0, 1, 2, 1, 1, 1, 1]),
        (52, [0, 0, 0, 2, 2, 2, 1, 2, 1]),
        (53, [0, 1, 0, 1, 2, 2, 1, 2, 1]),
        (54, [0, 1, 0, 2, 2, 2, 1, 2, 0]),
        (567, [1, 2, 0, 0, 2, 1, 0, 1, 0]),
        (56, [0, 1, 0, 2, 2, 2, 1, 2, 1]),
        (8249, [0, 0, 0, 0, 2, 0, 0, 0, 0]),
        (58, [0, 1, 0, 0, 1, 0, 0, 2, 0]),
        (59, [0, 2, 0, 0, 1, 0, 0, 2, 0]),
        (60, [0, 0, 0, 1, 2, 1, 0, 1, 0]),
        (61, [0, 0, 0, 1, 2, 1, 0, 1, 0]),
        (62, [0, 0, 0, 1, 2, 1, 0, 1, 0]),
        (63, [0, 1, 0, 0, 2, 0, 1, 2, 1]),
        (64, [1, 2, 1, 2, 2, 2, 1, 2, 2]),
        (65, [1, 0, 1, 2, 2, 2, 1, 3, 1]),
        (66, [1, 1, 0, 2, 2, 2, 2, 2, 1]),
        (67, [0, 1, 1, 2, 1, 0, 1, 2, 1]),
        (68, [1, 1, 0, 2, 1, 2, 2, 2, 1]),
        (69, [1, 1, 1, 2, 2, 1, 2, 2, 1]),
        (70, [1, 0, 0, 2, 2, 1, 1, 2, 1]),
        (71, [0, 1, 1, 2, 1, 2, 1, 2, 1]),
        (72, [1, 0, 1, 2, 2, 2, 2, 1, 2]),
        (73, [1, 1, 1, 0, 3, 0, 1, 3, 1]),
        (74, [1, 1, 0, 0, 2, 1, 1, 2, 1]),
        (75, [1, 0, 1, 2, 2, 1, 2, 2, 1]),
        (76, [0, 1, 1, 1, 2, 0, 1, 1, 0]),
        (77, [1, 0, 1, 2, 2, 2, 2, 1, 2]),
        (78, [1, 0, 1, 2, 2, 2, 2, 1, 2]),
        (79, [0, 1, 0, 2, 1, 2, 2, 2, 2]),
        (80, [1, 0, 0, 2, 2, 1, 2, 2, 1]),
        (81, [0, 1, 1, 2, 1, 2, 2, 2, 2]),
        (82, [1, 0, 1, 2, 2, 1, 2, 2, 1]),
        (83, [1, 1, 0, 1, 2, 2, 1, 2, 1]),
        (84, [0, 1, 0, 0, 3, 0, 1, 3, 1]),
        (85, [0, 1, 0, 2, 1, 2, 2, 1, 2]),
        (86, [0, 1, 0, 1, 3, 1, 2, 1, 2]),
        (87, [1, 0, 1, 2, 2, 2, 2, 2, 2]),
        (88, [1, 0, 1, 1, 3, 1, 2, 2, 2]),
        (89, [0, 1, 0, 0, 3, 0, 2, 1, 2]),
        (90, [1, 1, 1, 1, 2, 0, 1, 2, 1]),
        (91, [0, 2, 0, 1, 2, 0, 1, 2, 0]),
        (92, [0, 1, 2, 0, 2, 0, 2, 1, 0]),
        (93, [0, 2, 1, 0, 2, 1, 0, 2, 1]),
        (94, [0, 0, 0, 2, 1, 2, 0, 2, 0]),
        (95, [1, 2, 1, 0, 0, 0, 0, 0, 0]),
        (8800, [0, 0, 0, 1, 3, 1, 0, 1, 1]),
        (97, [1, 1, 1, 1, 2, 2, 0, 1, 0]),
        (98, [1, 1, 0, 2, 2, 2, 2, 1, 0]),
        (99, [0, 1, 1, 2, 2, 1, 0, 1, 0]),
        (100, [0, 1, 1, 2, 2, 2, 0, 1, 2]),
        (101, [0, 1, 0, 2, 2, 1, 0, 1, 0]),
        (102, [1, 1, 1, 1, 3, 1, 0, 3, 1]),
        (103, [1, 2, 1, 2, 2, 2, 0, 1, 1]),
        (104, [1, 0, 1, 2, 2, 2, 2, 1, 0]),
        (9660, [0, 0, 0, 1, 3, 1, 1, 1, 1]),
        (107, [1, 0, 1, 2, 3, 1, 2, 1, 1]),
        (108, [1, 1, 1, 0, 3, 0, 1, 3, 0]),
        (109, [1, 1, 1, 2, 2, 2, 1, 1, 1]),
        (110, [1, 0, 1, 2, 2, 2, 1, 1, 0]),
        (111, [0, 1, 0, 2, 1, 2, 0, 1, 0]),
        (112, [2, 1, 0, 2, 2, 2, 1, 1, 0]),
        (113, [0, 1, 2, 2, 2, 2, 0, 1, 1]),
        (114, [1, 1, 1, 1, 3, 1, 1, 1, 1]),
        (115, [0, 1, 0, 1, 2, 1, 0, 1, 1]),
        (116, [0, 1, 0, 1, 2, 0, 1, 2, 1]),
        (117, [0, 1, 1, 2, 2, 2, 1, 0, 1]),
        (118, [0, 1, 0, 1, 2, 1, 1, 0, 1]),
        (119, [1, 0, 1, 2, 2, 2, 1, 0, 1]),
        (120, [1, 1, 1, 1, 3, 1, 1, 1, 1]),
        (121, [0, 2, 0, 1, 2, 1, 1, 0, 1]),
        (122, [0, 1, 1, 0, 2, 0, 0, 1, 0]),
        (1012856, [0, 0, 0, 1, 2, 1, 1, 1, 0]),
        (124, [0, 2, 0, 0, 2, 0, 0, 2, 0]),
        (126, [0, 0, 0, 2, 2, 2, 0, 0, 0]),
        (9664, [0, 0, 0, 1, 3, 1, 0, 1, 1]),
        (8225, [0, 2, 0, 1, 3, 1, 1, 2, 1]),
        (1012843, [0, 0, 0, 1, 2, 1, 1, 2, 1]),
        (1011874, [1, 2, 0, 0, 2, 0, 0, 0, 0]),
        (1011762, [0, 2, 1, 0, 2, 0, 0, 0, 0]),
        (1012844, [0, 0, 0, 0, 2, 0, 0, 2, 0]),
        (1011765, [1, 3, 1, 0, 2, 0, 0, 0, 0]),
        (1012845, [0, 0, 0, 1, 1, 1, 1, 1, 1]),
        (8216, [0, 0, 0, 0, 0, 0, 0, 2, 0]),
        (1012846, [0, 0, 0, 1, 1, 1, 1, 1, 1]),
        (8729, [0, 0, 0, 0, 2, 0, 0, 0, 0]),
        (993441, [0, 1, 0, 0, 3, 0, 0, 2, 0]),
        (8734, [0, 0, 0, 2, 3, 2, 1, 1, 1]),
        (1012847, [0, 0, 0, 1, 2, 1, 1, 1, 1]),
        (8218, [0, 2, 0, 0, 1, 0, 0, 0, 0]),
        (7838, [1, 1, 1, 2, 1, 2, 2, 2, 1]),
        (7840, [1, 1, 1, 2, 2, 2, 1, 3, 1]),
        (7841, [1, 2, 1, 1, 2, 2, 0, 1, 0]),
        (162, [0, 1, 0, 2, 2, 1, 1, 2, 1]),
        (163, [1, 1, 1, 2, 2, 1, 1, 2, 1]),
        (164, [0, 1, 1, 2, 3, 1, 1, 2, 1]),
        (165, [0, 1, 0, 1, 3, 1, 2, 1, 2]),
        (166, [0, 2, 0, 0, 1, 0, 0, 2, 0]),
        (167, [1, 2, 1, 2, 2, 2, 1, 2, 1]),
        (169, [0, 1, 0, 2, 1, 2, 1, 1, 1]),
        (170, [0, 0, 0, 1, 2, 1, 1, 2, 1]),
        (171, [0, 0, 0, 2, 2, 2, 0, 0, 0]),
        (172, [0, 0, 0, 0, 1, 2, 1, 1, 0]),
        (626, [2, 0, 1, 2, 1, 2, 1, 1, 0]),
        (174, [0, 0, 0, 2, 2, 2, 1, 2, 1]),
        (1011756, [0, 1, 0, 0, 0, 0, 0, 0, 0]),
        (176, [0, 0, 0, 0, 0, 0, 1, 2, 1]),
        (177, [1, 1, 1, 1, 2, 1, 1, 2, 1]),
        (178, [0, 0, 0, 0, 2, 1, 0, 2, 1]),
        (179, [0, 0, 0, 0, 2, 1, 0, 2, 0]),
        (57, [0, 1, 0, 1, 2, 1, 1, 2, 1]),
        (182, [0, 1, 1, 1, 2, 2, 2, 2, 2]),
        (183, [0, 0, 0, 0, 2, 0, 0, 0, 0]),
        (7864, [1, 2, 1, 2, 2, 1, 2, 2, 1]),
        (7865, [0, 2, 0, 2, 2, 1, 0, 1, 0]),
        (186, [0, 0, 0, 1, 2, 1, 1, 2, 1]),
        (187, [0, 0, 0, 2, 2, 2, 0, 0, 0]),
        (993314, [0, 0, 0, 0, 0, 0, 1, 1, 1]),
        (191, [1, 2, 1, 0, 2, 0, 0, 1, 0]),
        (1012854, [0, 0, 0, 0, 2, 0, 1, 1, 1]),
        (198, [1, 1, 1, 2, 3, 2, 1, 3, 2]),
        (199, [0, 3, 1, 2, 1, 0, 1, 2, 1]),
        (1012468, [1, 3, 1, 0, 2, 1, 0, 0, 0]),
        (1012848, [0, 0, 0, 2, 2, 1, 1, 1, 1]),
        (7882, [1, 2, 1, 0, 3, 0, 1, 3, 1]),
        (1012855, [0, 0, 0, 1, 2, 1, 1, 1, 1]),
        (7884, [0, 2, 0, 2, 1, 2, 2, 2, 2]),
        (7885, [0, 2, 0, 2, 1, 2, 0, 1, 0]),
        (1012852, [0, 0, 0, 0, 2, 0, 1, 2, 0]),
        (1008683, [0, 1, 0, 1, 3, 1, 0, 1, 0]),
        (208, [1, 1, 0, 2, 2, 3, 2, 2, 2]),
        (8804, [1, 1, 0, 1, 2, 1, 0, 1, 1]),
        (8594, [0, 1, 0, 1, 2, 2, 0, 1, 0]),
        (1012792, [0, 0, 0, 1, 2, 1, 1, 2, 1]),
        (215, [0, 0, 0, 1, 3, 1, 0, 0, 0]),
        (8719, [1, 0, 1, 2, 1, 2, 2, 2, 2]),
        (1011748, [1, 3, 1, 0, 2, 0, 0, 0, 0]),
        (181, [2, 1, 1, 2, 1, 2, 1, 0, 1]),
        (1012858, [0, 0, 0, 0, 2, 0, 0, 1, 0]),
        (222, [1, 0, 0, 2, 2, 2, 2, 2, 1]),
        (223, [1, 1, 1, 2, 2, 2, 2, 2, 1]),
        (994017, [0, 0, 0, 0, 1, 0, 0, 2, 0]),
        (1012777, [0, 0, 0, 0, 2, 0, 0, 2, 0]),
        (994019, [0, 0, 0, 0, 0, 0, 1, 1, 1]),
        (7908, [0, 2, 0, 2, 1, 2, 2, 1, 2]),
        (7909, [0, 2, 1, 2, 2, 2, 1, 0, 1]),
        (230, [1, 1, 1, 2, 3, 2, 1, 1, 1]),
        (231, [0, 3, 1, 2, 2, 1, 0, 1, 0]),
        (994024, [0, 0, 0, 0, 2, 0, 0, 1, 0]),
        (1012853, [0, 0, 0, 1, 2, 1, 1, 1, 1]),
        (1009389, [0, 0, 0, 1, 2, 1, 0, 0, 0]),
        (1009390, [0, 1, 0, 1, 3, 1, 0, 1, 0]),
        (8805, [0, 1, 1, 1, 2, 1, 1, 1, 0]),
        (240, [0, 1, 0, 2, 2, 2, 1, 2, 1]),
        (1009393, [0, 1, 0, 1, 2, 1, 1, 1, 0]),
        (1012850, [0, 0, 0, 0, 2, 0, 0, 1, 1]),
        (994018, [0, 0, 0, 0, 0, 0, 0, 2, 0]),
        (1009396, [0, 1, 1, 2, 3, 1, 0, 1, 1]),
        (247, [0, 1, 0, 1, 2, 1, 0, 1, 0]),
        (248, [1, 1, 0, 2, 1, 2, 0, 1, 1]),
        (254, [2, 1, 0, 2, 1, 2, 2, 1, 0]),
        (63743, [0, 2, 1, 2, 3, 2, 1, 2, 1]),
        (8601, [1, 1, 1, 2, 2, 1, 0, 0, 0]),
        (257, [1, 1, 1, 1, 2, 2, 1, 2, 1]),
        (8747, [1, 2, 0, 0, 2, 0, 0, 2, 1]),
        (260, [1, 1, 2, 2, 2, 2, 1, 3, 1]),
        (261, [1, 1, 2, 1, 2, 2, 0, 1, 0]),
        (1012833, [0, 0, 0, 1, 2, 1, 0, 1, 1]),
        (993319, [0, 0, 0, 0, 0, 0, 0, 2, 0]),
        (1012780, [0, 0, 0, 0, 1, 0, 0, 0, 0]),
        (1008855, [0, 0, 0, 1, 3, 1, 0, 0, 0]),
        (1012781, [0, 0, 0, 0, 1, 0, 0, 1, 0]),
        (272, [1, 1, 0, 2, 2, 3, 2, 2, 2]),
        (273, [0, 1, 1, 2, 2, 2, 0, 2, 2]),
        (1032218, [0, 1, 0, 2, 2, 2, 2, 2, 2]),
        (275, [0, 1, 0, 2, 2, 1, 1, 2, 1]),
        (46, [0, 1, 0, 0, 1, 0, 0, 0, 0]),
        (8470, [1, 0, 0, 2, 2, 2, 2, 2, 2]),
        (280, [1, 2, 2, 2, 2, 1, 2, 2, 1]),
        (281, [0, 2, 1, 2, 2, 1, 0, 1, 0]),
        (1012840, [0, 0, 0, 1, 1, 1, 1, 2, 1]),
        (8240, [1, 1, 1, 2, 2, 2, 2, 2, 0]),
        (290, [0, 2, 1, 2, 1, 2, 1, 2, 1]),
        (1012851, [0, 0, 0, 0, 2, 1, 0, 1, 0]),
        (1012834, [0, 0, 0, 1, 2, 1, 1, 2, 1]),
        (294, [1, 0, 1, 2, 2, 2, 2, 1, 2]),
        (295, [1, 0, 1, 2, 2, 2, 2, 2, 0]),
        (299, [1, 1, 1, 0, 3, 0, 1, 2, 0]),
        (1008690, [1, 1, 1, 0, 2, 1, 0, 1, 0]),
        (302, [1, 3, 1, 0, 3, 0, 1, 3, 1]),
        (305, [1, 1, 1, 0, 3, 0, 1, 1, 0]),
        (306, [1, 1, 0, 2, 1, 2, 2, 1, 2]),
        (51, [1, 1, 0, 0, 2, 2, 1, 2, 1]),
        (993467, [0, 0, 0, 1, 2, 2, 1, 1, 1]),
        (310, [1, 1, 1, 2, 2, 1, 2, 2, 1]),
        (311, [1, 1, 1, 2, 3, 1, 2, 1, 1]),
        (312, [1, 0, 1, 2, 3, 1, 1, 1, 1]),
        (1008692, [1, 2, 1, 1, 3, 1, 0, 1, 0]),
        (315, [0, 2, 1, 1, 2, 0, 1, 1, 0]),
        (316, [1, 2, 1, 0, 3, 0, 1, 3, 0]),
        (319, [0, 1, 1, 1, 2, 2, 1, 1, 0]),
        (320, [1, 1, 1, 0, 3, 1, 1, 3, 0]),
        (321, [0, 1, 1, 2, 2, 0, 1, 1, 0]),
        (322, [1, 1, 1, 1, 3, 0, 1, 3, 0]),
        (1012835, [0, 0, 0, 1, 2, 0, 0, 1, 0]),
        (325, [1, 1, 1, 2, 2, 2, 2, 1, 2]),
        (326, [1, 1, 1, 2, 2, 2, 1, 1, 0]),
        (1012784, [0, 0, 0, 1, 2, 1, 1, 2, 1]),
        (8467, [0, 1, 0, 0, 2, 0, 0, 2, 0]),
        (329, [0, 1, 1, 1, 2, 2, 2, 1, 1]),
        (330, [1, 1, 2, 3, 2, 3, 2, 1, 2]),
        (331, [1, 1, 2, 2, 1, 2, 1, 1, 0]),
        (333, [0, 1, 0, 2, 1, 2, 1, 2, 1]),
        (1008696, [0, 1, 0, 2, 2, 2, 1, 2, 1]),
        (338, [0, 1, 1, 2, 2, 2, 2, 2, 2]),
        (339, [1, 1, 1, 2, 3, 2, 1, 1, 1]),
        (993451, [0, 0, 0, 2, 2, 1, 1, 1, 1]),
        (342, [1, 1, 1, 2, 2, 1, 2, 2, 1]),
        (343, [1, 2, 1, 1, 3, 1, 1, 1, 1]),
        (1012837, [0, 0, 0, 1, 2, 1, 1, 1, 0]),
        (1011769, [0, 3, 1, 1, 2, 1, 0, 0, 0]),
        (8250, [0, 0, 0, 0, 2, 0, 0, 0, 0]),
        (350, [1, 2, 0, 1, 2, 2, 1, 2, 1]),
        (351, [1, 2, 0, 1, 2, 1, 0, 1, 1]),
        (1012836, [0, 0, 0, 1, 2, 1, 1, 2, 1]),
        (354, [0, 2, 0, 0, 3, 0, 1, 3, 1]),
        (355, [0, 2, 0, 1, 2, 0, 1, 2, 1]),
        (358, [0, 1, 0, 0, 3, 0, 1, 3, 1]),
        (359, [0, 1, 0, 1, 3, 1, 1, 2, 1]),
        (1008700, [0, 0, 0, 1, 3, 1, 0, 0, 0]),
        (363, [0, 1, 1, 2, 2, 2, 1, 2, 1]),
        (1009042, [1, 2, 0, 0, 3, 0, 0, 2, 1]),
        (1008701, [0, 0, 0, 1, 2, 1, 0, 1, 0]),
        (370, [0, 3, 0, 2, 1, 2, 2, 1, 2]),
        (371, [0, 2, 2, 2, 2, 2, 1, 0, 1]),
        (1012857, [0, 0, 0, 0, 3, 0, 1, 1, 1]),
        (1008702, [0, 0, 0, 1, 3, 1, 0, 0, 0]),
        (1008693, [1, 2, 1, 1, 2, 1, 0, 1, 1]),
        (8230, [1, 1, 1, 1, 1, 1, 0, 0, 0]),
        (1008677, [1, 1, 1, 1, 3, 1, 2, 2, 1]),
        (1008691, [1, 2, 1, 0, 2, 1, 1, 1, 0]),
        (1011763, [0, 2, 1, 0, 2, 0, 0, 0, 0]),
        (8486, [1, 1, 1, 2, 1, 2, 2, 2, 2]),
        (1011753, [0, 2, 0, 0, 2, 0, 0, 0, 0]),
        (1012793, [0, 0, 0, 0, 2, 0, 1, 2, 1]),
        (1012782, [0, 0, 0, 0, 1, 0, 0, 0, 0]),
        (399, [0, 1, 0, 2, 2, 2, 1, 2, 1]),
        (8592, [0, 1, 0, 2, 2, 1, 0, 1, 0]),
        (1012776, [0, 0, 0, 0, 2, 0, 0, 2, 0]),
        (402, [1, 2, 0, 0, 3, 0, 0, 2, 1]),
        (8595, [0, 1, 0, 1, 3, 1, 0, 1, 0]),
        (916, [1, 1, 1, 2, 2, 2, 0, 3, 0]),
        (8598, [0, 0, 0, 2, 2, 1, 1, 2, 1]),
        (8599, [0, 0, 0, 1, 2, 2, 1, 2, 1]),
        (8600, [0, 1, 1, 1, 2, 2, 0, 0, 0]),
        (1008695, [1, 2, 0, 0, 2, 1, 0, 1, 1]),
        (1009391, [0, 0, 0, 1, 2, 1, 0, 0, 0]),
        (413, [2, 0, 1, 3, 2, 3, 2, 1, 2]),
        (1011764, [1, 2, 1, 1, 2, 0, 0, 0, 0]),
        (1011760, [1, 2, 1, 1, 2, 1, 0, 0, 0]),
        (1009392, [1, 1, 0, 1, 2, 1, 0, 1, 1]),
        (9829, [0, 0, 0, 2, 3, 2, 2, 1, 2]),
        (937, [1, 1, 1, 2, 1, 2, 2, 2, 2]),
        (993471, [0, 1, 0, 1, 2, 0, 0, 2, 0]),
        (1011768, [1, 3, 1, 1, 2, 1, 0, 0, 0]),
        (8776, [0, 0, 0, 1, 2, 1, 1, 1, 0]),
        (9650, [0, 0, 0, 1, 2, 1, 0, 2, 0]),
        (9654, [0, 0, 0, 1, 3, 1, 1, 1, 0]),
        (185, [0, 0, 0, 0, 2, 1, 0, 2, 0]),
        (1012839, [0, 1, 0, 1, 2, 2, 0, 1, 1]),
        (956, [2, 1, 1, 2, 1, 2, 1, 0, 1]),
        (1008688, [0, 1, 0, 2, 1, 2, 0, 1, 0]),
        (1008689, [1, 1, 1, 1, 3, 1, 0, 1, 0]),
        (1012788, [0, 0, 0, 1, 2, 1, 1, 2, 1]),
        (960, [1, 0, 1, 2, 2, 2, 1, 1, 1]),
        (1013492, [0, 0, 0, 1, 2, 1, 1, 2, 1]),
        (1012898, [0, 0, 0, 0, 2, 0, 1, 2, 0]),
        (161, [0, 2, 0, 0, 2, 0, 0, 1, 0]),
        (9674, [1, 3, 1, 2, 1, 2, 1, 3, 1]),
        (1008887, [0, 1, 0, 1, 2, 1, 0, 1, 0]),
        (1008802, [0, 2, 0, 2, 2, 1, 0, 1, 0]),
        (1008803, [1, 1, 0, 1, 2, 1, 0, 1, 0]),
        (1008697, [0, 2, 0, 2, 2, 2, 0, 1, 0]),
        (55, [0, 1, 0, 0, 2, 0, 1, 2, 1]),
        (1008804, [0, 1, 1, 2, 3, 1, 0, 1, 1]),
        (1011766, [1, 3, 1, 0, 2, 0, 0, 0, 0]),
        (1012789, [0, 0, 0, 0, 2, 1, 1, 2, 1]),
        (1011757, [0, 1, 0, 0, 0, 0, 0, 0, 0]),
        (1008805, [0, 1, 0, 2, 3, 2, 1, 1, 1]),
        (1011761, [0, 3, 1, 0, 2, 0, 0, 0, 0]),
        (1012791, [0, 0, 0, 0, 2, 0, 1, 2, 0]),
        (11089, [0, 0, 0, 0, 2, 0, 0, 0, 0]),
        (1008817, [1, 1, 1, 1, 2, 1, 1, 2, 1]),
        (490, [1, 3, 0, 2, 1, 2, 2, 2, 2]),
        (491, [1, 3, 0, 2, 1, 2, 0, 1, 0]),
        (1012838, [0, 0, 0, 0, 2, 1, 0, 3, 1]),
        (1011758, [0, 1, 0, 0, 0, 0, 0, 0, 0]),
        (8217, [0, 0, 0, 0, 0, 0, 0, 2, 0]),
        (1008694, [0, 1, 0, 2, 2, 2, 0, 2, 0]),
        (994023, [0, 0, 0, 0, 2, 0, 0, 1, 0]),
        (1011767, [0, 2, 0, 1, 2, 0, 0, 0, 0]),
        (1012790, [0, 0, 0, 1, 2, 1, 0, 2, 0]),
    ]
    .iter()
    .copied()
    .collect();
}
