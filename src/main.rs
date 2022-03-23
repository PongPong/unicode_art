extern crate exitcode;

mod arg;
mod unicode_art;

use crate::arg::{BrailleThreshold, NumColumns};
use crate::unicode_art::block::BlockUnicodeArtOption;
use crate::unicode_art::braille::BrailleAsciiArtOption;
use crate::unicode_art::braille::DEFAULT_THRESHOLD;
use crate::unicode_art::classic::ClassicAsciiArtOption;
use crate::unicode_art::error::UnicodeArtError;
use crate::unicode_art::mandel::MandelAsciiArtOption;
use crate::unicode_art::subpixel::SubpixelUnicodeArtOption;
use crate::unicode_art::{UnicodeArt, UnicodeArtOption};

use std::io::{stdout, BufWriter};

use clap::lazy_static::lazy_static;
use clap::{arg, Arg, Command};

const MIN_NUM_COLS: u32 = 1;
const ARG_PRESET: &'static str = "PRESET";
const SUB_COMMAND_CLASSIC: &'static str = "classic";
const SUB_COMMAND_BRAILLE: &'static str = "braille";
const SUB_COMMAND_SUBPIXEL: &'static str = "subpixel";
const SUB_COMMAND_PATTERN: &'static str = "pattern";
const DEFAULT_NUM_COLS: u32 = 80;

lazy_static! {
    static ref DEFAULT_NUM_COLS_STR: String = DEFAULT_NUM_COLS.to_string();
    static ref DEFAULT_THRESHOLD_STR: String = DEFAULT_THRESHOLD.to_string();
    static ref ARG_NUM_COLS: Arg<'static> = {
        Arg::new("NUM_COLS")
            .long("width")
            .short('w')
            .help("Number of columns")
            .takes_value(true)
            .default_value(DEFAULT_NUM_COLS_STR.as_str())
            .default_missing_value(DEFAULT_NUM_COLS_STR.as_str())
            .use_value_delimiter(false)
    };
    static ref ARG_COLOR: Arg<'static> = {
        Arg::new("COLOR")
            .long("color")
            .short('c')
            .help("ANSI color output")
            .use_value_delimiter(false)
    };
    static ref ARG_INVERT: Arg<'static> = {
        Arg::new("INVERT")
            .long("invert")
            .short('i')
            .help("Insert color")
            .use_value_delimiter(false)
    };
}

fn main() {
    let matches = Command::new("unicode_art")
        .about("A Unicode art generator")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(false)
        .allow_invalid_utf8_for_external_subcommands(false)
        .subcommand(
            Command::new(SUB_COMMAND_CLASSIC)
                .about("Generate ASCII art from image")
                .arg(arg!(<IMAGE_PATH> "Image path"))
                .arg_required_else_help(true)
                .arg(
                    Arg::new(ARG_PRESET)
                        .long("preset")
                        .short('p')
                        .help("Preset chars list")
                        .default_value("standard")
                        .default_missing_value("standard")
                        .possible_values([
                            "standard", "level_10", "level_16", "level_19", "level_23", "block",
                        ])
                        .takes_value(true)
                        .use_value_delimiter(false),
                )
                .arg(ARG_NUM_COLS.clone())
                .arg(ARG_COLOR.clone())
                .arg(ARG_INVERT.clone()),
        )
        .subcommand(
            Command::new(SUB_COMMAND_BRAILLE)
                .about("Generate Braille Unicode art from image")
                .arg(arg!(<IMAGE_PATH> "Image path"))
                .arg_required_else_help(true)
                .arg(
                    Arg::new("THRESHOLD")
                        .long("threshold")
                        .short('t')
                        .help("threshold")
                        .takes_value(true)
                        .default_value(DEFAULT_THRESHOLD_STR.as_str())
                        .default_missing_value(DEFAULT_THRESHOLD_STR.as_str())
                        .use_value_delimiter(false),
                )
                .arg(ARG_NUM_COLS.clone())
                .arg(ARG_COLOR.clone())
                .arg(ARG_INVERT.clone()),
        )
        .subcommand(
            Command::new(SUB_COMMAND_SUBPIXEL)
                .about("Generate Subpixel Unicode art from image")
                .arg(arg!(<IMAGE_PATH> "Image path"))
                .arg_required_else_help(true)
                .arg(ARG_NUM_COLS.clone())
                .arg(ARG_COLOR.clone())
                .arg(ARG_INVERT.clone()),
        )
        .subcommand(
            Command::new(SUB_COMMAND_PATTERN)
                .about("Generate ASCII art pattern")
                .arg_required_else_help(true)
                .arg(
                    Arg::new(ARG_PRESET)
                        .long("preset")
                        .short('p')
                        .help("Preset pattern")
                        .possible_values(["mandel"])
                        .takes_value(true)
                        .default_value("mandel")
                        .default_missing_value("mandel")
                        .use_value_delimiter(false),
                )
                .arg(ARG_NUM_COLS.clone()),
        )
        .get_matches();

    fn get_img2_txt_impl<'a>(
        name: &str,
        num_cols: u32,
        image_path: &'a str,
        is_color: bool,
        is_invert: bool
    ) -> Result<Box<dyn UnicodeArt + 'a>, UnicodeArtError> {
        match name {
            "standard" => ClassicAsciiArtOption::new_standard(num_cols, image_path, is_color, is_invert)
                .new_unicode_art(),
            "level_10" => ClassicAsciiArtOption::new_level_10(num_cols, image_path, is_color, is_invert)
                .new_unicode_art(),
            "level_19" => ClassicAsciiArtOption::new_level_19(num_cols, image_path, is_color, is_invert)
                .new_unicode_art(),
            "level_16" => ClassicAsciiArtOption::new_level_16(num_cols, image_path, is_color, is_invert)
                .new_unicode_art(),
            "level_23" => ClassicAsciiArtOption::new_level_23(num_cols, image_path, is_color, is_invert)
                .new_unicode_art(),
            "block" => BlockUnicodeArtOption::new(num_cols, image_path, is_color).new_unicode_art(),
            _ => Err(UnicodeArtError::UnsupportError),
        }
    }

    fn get_patten_impl<'a>(
        name: &str,
        _num_cols: u32,
    ) -> Result<Box<dyn UnicodeArt + 'a>, UnicodeArtError> {
        match name {
            "mandel" => MandelAsciiArtOption::new().new_unicode_art(),
            _ => Err(UnicodeArtError::UnsupportError),
        }
    }

    let mut buf = BufWriter::new(stdout());
    let _ = match matches.subcommand() {
        Some(("classic", sub_matches)) => {
            let num_cols = sub_matches.num_cols(MIN_NUM_COLS, DEFAULT_NUM_COLS);
            let image_path = sub_matches
                .value_of("IMAGE_PATH")
                .expect("Missing image path");
            let is_color = sub_matches.is_present("COLOR");
            let is_invert = sub_matches.is_present("INVERT");

            sub_matches
                .value_of(ARG_PRESET)
                .map_or(Err(UnicodeArtError::UnsupportError), |name| {
                    get_img2_txt_impl(name, num_cols, image_path, is_color, is_invert)
                })
        }
        Some(("pattern", sub_matches)) => {
            let num_cols = sub_matches.num_cols(MIN_NUM_COLS, DEFAULT_NUM_COLS);

            sub_matches
                .value_of(ARG_PRESET)
                .map_or(Err(UnicodeArtError::UnsupportError), |name| {
                    get_patten_impl(name, num_cols)
                })
        }
        Some(("braille", sub_matches)) => {
            let num_cols = sub_matches.num_cols(MIN_NUM_COLS, DEFAULT_NUM_COLS);
            let image_path = sub_matches
                .value_of("IMAGE_PATH")
                .expect("Missing image path");
            let threshold = sub_matches.threshold();
            let is_color = sub_matches.is_present("COLOR");
            let is_invert = sub_matches.is_present("INVERT");

            BrailleAsciiArtOption::new(num_cols, image_path, threshold, is_color, is_invert)
                .new_unicode_art()
        }
        Some(("subpixel", sub_matches)) => {
            let num_cols = sub_matches.num_cols(MIN_NUM_COLS, DEFAULT_NUM_COLS);
            let image_path = sub_matches
                .value_of("IMAGE_PATH")
                .expect("Missing image path");
            let is_invert = sub_matches.is_present("INVERT");

            SubpixelUnicodeArtOption::new(num_cols, image_path, is_invert).new_unicode_art()
        }
        _ => {
            unreachable!();
        }
    }
    .unwrap()
    .write_all(&mut buf);
}
