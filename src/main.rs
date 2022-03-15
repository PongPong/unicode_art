extern crate exitcode;

mod arg;
mod unicode_art;

use crate::arg::{BrailleThreshold, NumColumns};
use crate::unicode_art::block::BlockUnicodeArt;
use crate::unicode_art::braille::BrailleAsciiArt;
use crate::unicode_art::simple::SimpleAsciiUnicodeArt;
use crate::unicode_art::UnicodeArt;
use crate::unicode_art::{braille::DEFAULT_THRESHOLD, mandel::MandelAsciiArt};

use std::io::{stdout, BufWriter};

use clap::{arg, Arg, Command};

const DEFAULT_NUM_COLS: u32 = 80;
const MIN_NUM_COLS: u32 = 1;
const ARG_NUM_COLS: &'static str = "NUM_COLS";
const ARG_PRESET: &'static str = "PRESET";
const ARG_COLOR: &'static str = "COLOR";
const ARG_INVERT: &'static str = "INVERT";
const SUB_COMMAND_CLASSIC: &'static str = "classic";
const SUB_COMMAND_BRAILLE: &'static str = "braille";
const SUB_COMMAND_PATTERN: &'static str = "pattern";

fn main() {
    let default_threshold = DEFAULT_THRESHOLD.to_string();
    let default_num_cols = DEFAULT_NUM_COLS.to_string();
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
                .arg(
                    Arg::new(ARG_NUM_COLS)
                        .long("width")
                        .short('w')
                        .help("Number of columns")
                        .takes_value(true)
                        .default_value(default_num_cols.as_str())
                        .default_missing_value(default_num_cols.as_str())
                        .use_value_delimiter(false),
                )
                .arg(
                    Arg::new(ARG_COLOR)
                        .long("color")
                        .short('c')
                        .help("ANSI color output")
                        .use_value_delimiter(false),
                ),
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
                        .default_value(default_threshold.as_str())
                        .default_missing_value(default_threshold.as_str())
                        .use_value_delimiter(false),
                )
                .arg(
                    Arg::new(ARG_NUM_COLS)
                        .long("width")
                        .short('w')
                        .help("Number of columns")
                        .takes_value(true)
                        .default_value(default_num_cols.as_str())
                        .default_missing_value(default_num_cols.as_str())
                        .use_value_delimiter(false),
                )
                .arg(
                    Arg::new(ARG_COLOR)
                        .long("color")
                        .short('c')
                        .help("ANSI color output")
                        .use_value_delimiter(false),
                )
                .arg(
                    Arg::new(ARG_INVERT)
                        .long("invert")
                        .short('i')
                        .help("Insert color")
                        .use_value_delimiter(false),
                ),
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
                .arg(
                    Arg::new(ARG_NUM_COLS)
                        .long("width")
                        .short('w')
                        .help("Number of columns")
                        .takes_value(true)
                        .default_value(default_num_cols.as_str())
                        .default_missing_value(default_num_cols.as_str())
                        .use_value_delimiter(false),
                ),
        )
        .get_matches();

    fn get_img2_txt_impl<'a>(
        name: &str,
        num_cols: u32,
        image_path: &'a str,
        is_color: bool,
    ) -> Option<Box<dyn UnicodeArt + 'a>> {
        match name {
            "standard" => Some(Box::new(SimpleAsciiUnicodeArt::new_standard(
                num_cols, image_path, is_color,
            ))),
            "level_10" => Some(Box::new(SimpleAsciiUnicodeArt::new_level_10(
                num_cols, image_path, is_color,
            ))),
            "level_19" => Some(Box::new(SimpleAsciiUnicodeArt::new_level_19(
                num_cols, image_path, is_color,
            ))),
            "level_16" => Some(Box::new(SimpleAsciiUnicodeArt::new_level_16(
                num_cols, image_path, is_color,
            ))),
            "level_23" => Some(Box::new(SimpleAsciiUnicodeArt::new_level_23(
                num_cols, image_path, is_color,
            ))),
            "block" => Some(Box::new(BlockUnicodeArt::new(
                num_cols, image_path, is_color,
            ))),
            _ => None,
        }
    }

    fn get_patten_impl(name: &str, _num_cols: u32) -> Option<Box<dyn UnicodeArt>> {
        match name {
            "mandel" => Some(Box::new(MandelAsciiArt::new())),
            _ => None,
        }
    }

    let gen = match matches.subcommand() {
        Some(("classic", sub_matches)) => {
            let num_cols = sub_matches.num_cols(MIN_NUM_COLS, DEFAULT_NUM_COLS);
            let image_path = sub_matches
                .value_of("IMAGE_PATH")
                .expect("Missing image path");
            let is_color = sub_matches.is_present(ARG_COLOR);

            sub_matches
                .value_of(ARG_PRESET)
                .and_then(|name| get_img2_txt_impl(name, num_cols, image_path, is_color))
        }
        Some(("pattern", sub_matches)) => {
            let num_cols = sub_matches.num_cols(MIN_NUM_COLS, DEFAULT_NUM_COLS);

            sub_matches
                .value_of(ARG_PRESET)
                .and_then(|name| get_patten_impl(name, num_cols))
        }
        Some(("braille", sub_matches)) => {
            let num_cols = sub_matches.num_cols(MIN_NUM_COLS, DEFAULT_NUM_COLS);
            let image_path = sub_matches
                .value_of("IMAGE_PATH")
                .expect("Missing image path");
            let threshold = sub_matches.threshold();
            let is_color = sub_matches.is_present(ARG_COLOR);
            let is_invert = sub_matches.is_present(ARG_INVERT);

            Some(Box::new(BrailleAsciiArt::new(
                num_cols, image_path, threshold, is_color, is_invert,
            )) as Box<dyn UnicodeArt>)
        }
        _ => {
            unreachable!();
        }
    };

    if let Some(g) = gen {
        let mut buf = BufWriter::new(stdout());
        let _ = g.generate(&mut buf);
    }
}
