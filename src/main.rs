extern crate exitcode;

mod arg;
mod unicode_art;

use crate::arg::{BrailleThreshold, NumColumns};
use crate::unicode_art::block::BlockUnicodeArtOption;
use crate::unicode_art::braille::BrailleAsciiArtOption;
use crate::unicode_art::braille::DEFAULT_THRESHOLD;
use crate::unicode_art::classic::ClassicAsciiArtOption;
use crate::unicode_art::error::UnicodeArtError;
use crate::unicode_art::input::Input;
use crate::unicode_art::mandel::MandelAsciiArtOption;
use crate::unicode_art::subpixel::SubpixelUnicodeArtOption;
use crate::unicode_art::UnicodeArtOption;

use std::io::{stdin, stdout, BufWriter};

use clap::lazy_static::lazy_static;
use clap::{Arg, Command};
use image::io::Reader;
use image::{DynamicImage, RgbImage};

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
    static ref ARG_IMAGE_PATH: Arg<'static> = {
        Arg::new("IMAGE_PATH")
            .help("Image path")
            .takes_value(true)
            .required_unless_present("STDIN")
    };
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
//  let mut input = if arg == "--" {
//     Input::Stdin(io::stdin())
// } else {
//     Input::File(fs::File::open(&arg).expect("I should handle that.."))
// };
    static ref ARG_STDIN: Arg<'static> = {
        Arg::new("STDIN")
            .long("stdin")
            .help("Read image from Stdin")
    };
}

fn get_img2_txt_impl<'a>(
    name: &'a str,
    num_cols: u32,
    is_color: bool,
    is_invert: bool,
) -> Result<Box<dyn UnicodeArtOption>, UnicodeArtError> {
    let option: Box<dyn UnicodeArtOption> = match name {
        "standard" => Box::new(ClassicAsciiArtOption::new_standard(
            num_cols, is_color, is_invert,
        )),
        "level_10" => Box::new(ClassicAsciiArtOption::new_level_10(
            num_cols, is_color, is_invert,
        )),
        "level_19" => Box::new(ClassicAsciiArtOption::new_level_19(
            num_cols, is_color, is_invert,
        )),
        "level_16" => Box::new(ClassicAsciiArtOption::new_level_16(
            num_cols, is_color, is_invert,
        )),
        "level_23" => Box::new(ClassicAsciiArtOption::new_level_23(
            num_cols, is_color, is_invert,
        )),
        "block" => Box::new(BlockUnicodeArtOption::new(num_cols, is_color)),
        _ => return Err(UnicodeArtError::UnsupportError),
    };
    Ok(option)
}

fn get_patten_impl<'a>(
    name: &'a str,
    _num_cols: u32,
) -> Result<Box<dyn UnicodeArtOption>, UnicodeArtError> {
    let option: Box<dyn UnicodeArtOption> = match name {
        "mandel" => Box::new(MandelAsciiArtOption::new()),
        _ => return Err(UnicodeArtError::UnsupportError),
    };
    Ok(option)
}

fn main() -> Result<(), UnicodeArtError> {
    let matches = Command::new("unicode_art")
        .about("A Unicode art generator")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(false)
        .allow_invalid_utf8_for_external_subcommands(false)
        .subcommand(
            Command::new(SUB_COMMAND_CLASSIC)
                .about("Generate ASCII art from image")
                .arg(ARG_STDIN.clone())
                .arg(ARG_IMAGE_PATH.clone())
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
                .arg(ARG_INVERT.clone())
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new(SUB_COMMAND_BRAILLE)
                .about("Generate Braille Unicode art from image")
                .arg(ARG_STDIN.clone())
                .arg(ARG_IMAGE_PATH.clone())
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
                .arg(ARG_INVERT.clone())
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new(SUB_COMMAND_SUBPIXEL)
                .about("Generate Subpixel Unicode art from image")
                .arg(ARG_STDIN.clone())
                .arg(ARG_IMAGE_PATH.clone())
                .arg(ARG_NUM_COLS.clone())
                .arg(ARG_COLOR.clone())
                .arg(ARG_INVERT.clone())
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new(SUB_COMMAND_PATTERN)
                .about("Generate ASCII art pattern")
                .arg(ARG_STDIN.clone())
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
                .arg(ARG_NUM_COLS.clone())
                .arg_required_else_help(true),
        )
        .get_matches();

    let mut buf = BufWriter::new(stdout());
    match matches.subcommand() {
        Some(("classic", sub_matches)) => {
            let num_cols = sub_matches.num_cols(MIN_NUM_COLS, DEFAULT_NUM_COLS);
            let is_color = sub_matches.is_present("COLOR");
            let is_invert = sub_matches.is_present("INVERT");
            let is_stdin = sub_matches.is_present("STDIN");

            let image = if is_stdin {
                Reader::new(Input::stdin(stdin()))
                    .with_guessed_format()?
                    .decode()?
            } else {
                let image_path = sub_matches
                    .value_of("IMAGE_PATH")
                    .expect("Missing image path");
                Reader::open(image_path).unwrap().decode()?
            };

            sub_matches
                .value_of(ARG_PRESET)
                .map_or(Err(UnicodeArtError::UnsupportError), |name| {
                    get_img2_txt_impl(name, num_cols, is_color, is_invert)
                })?
                .new_unicode_art(&image)?
                .write_all(&mut buf)?;
            Ok(())
        }
        Some(("pattern", sub_matches)) => {
            let num_cols = sub_matches.num_cols(MIN_NUM_COLS, DEFAULT_NUM_COLS);
            let image = DynamicImage::ImageRgb8(RgbImage::new(1, 1));

            sub_matches
                .value_of(ARG_PRESET)
                .map_or(Err(UnicodeArtError::UnsupportError), |name| {
                    get_patten_impl(name, num_cols)
                })?
                .new_unicode_art(&image)?
                .write_all(&mut buf)?;
            Ok(())
        }
        Some(("braille", sub_matches)) => {
            let num_cols = sub_matches.num_cols(MIN_NUM_COLS, DEFAULT_NUM_COLS);
            let threshold = sub_matches.threshold();
            let is_color = sub_matches.is_present("COLOR");
            let is_invert = sub_matches.is_present("INVERT");
            let is_stdin = sub_matches.is_present("STDIN");

            let image = if is_stdin {
                Reader::new(Input::stdin(stdin()))
                    .with_guessed_format()?
                    .decode()?
            } else {
                let image_path = sub_matches
                    .value_of("IMAGE_PATH")
                    .expect("Missing image path");
                Reader::open(image_path)?.decode()?
            };

            BrailleAsciiArtOption::new(num_cols, threshold, is_color, is_invert)
                .new_unicode_art(&image)?
                .write_all(&mut buf)?;
            Ok(())
        }
        Some(("subpixel", sub_matches)) => {
            let num_cols = sub_matches.num_cols(MIN_NUM_COLS, DEFAULT_NUM_COLS);
            let is_invert = sub_matches.is_present("INVERT");
            let is_stdin = sub_matches.is_present("STDIN");

            let image = if is_stdin {
                Reader::new(Input::stdin(stdin()))
                    .with_guessed_format()?
                    .decode()?
            } else {
                let image_path = sub_matches
                    .value_of("IMAGE_PATH")
                    .expect("Missing image path");
                Reader::open(image_path)?.decode()?
            };

            SubpixelUnicodeArtOption::new(num_cols, is_invert)
                .new_unicode_art(&image)?
                .write_all(&mut buf)?;
            Ok(())
        }
        _ => {
            unreachable!();
        }
    }
}
