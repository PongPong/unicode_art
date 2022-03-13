extern crate exitcode;

mod unicode_art;

use crate::unicode_art::simple::SimpleAsciiUnicodeArt;
use crate::unicode_art::UnicodeArt;

use std::{process::exit, io::{stdout, BufWriter}};

use clap::{arg, Arg, Command};

const DEFAULT_NUM_COLS: u32 = 80;
const MIN_NUM_COLS: u32 = 1;

fn main() {
    let matches = Command::new("unicode_art")
        .about("A Unicode art generator")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .allow_invalid_utf8_for_external_subcommands(true)
        .subcommand(
            Command::new("ascii")
                .about("Generate ascii art")
                .arg(arg!(<IMAGE_PATH> "Image path"))
                .arg_required_else_help(true)
                .arg(
                    Arg::new("PRESET")
                        .long("preset")
                        .short('p')
                        .help("Preset chars list")
                        .takes_value(true)
                        .default_missing_value("standard")
                        .use_value_delimiter(false),
                )
                .arg(
                    Arg::new("NUM_COLS")
                        .long("num_cols")
                        .short('c')
                        .help("Number of columns")
                        .takes_value(true)
                        .default_missing_value(DEFAULT_NUM_COLS.to_string().as_str())
                        .use_value_delimiter(false),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("ascii", sub_matches)) => {
            let num_cols = sub_matches
                .value_of("NUM_COLS")
                .map_or(DEFAULT_NUM_COLS, |val| {
                    val.parse::<u32>().expect("Invalid num_cols")
                });
            if num_cols <= MIN_NUM_COLS {
                eprintln!("Invalid num_cols");
                exit(exitcode::USAGE)
            }

            let image_path = sub_matches
                .value_of("IMAGE_PATH")
                .expect("Missing image path");

            let gen = sub_matches.value_of("PRESET").map_or_else(
                || SimpleAsciiUnicodeArt::new_standard(num_cols),
                |val| match val {
                    "standard" => SimpleAsciiUnicodeArt::new_standard(num_cols),
                    "level_10" => SimpleAsciiUnicodeArt::new_level_10(num_cols),
                    "level_19" => SimpleAsciiUnicodeArt::new_level_19(num_cols),
                    "level_16" => SimpleAsciiUnicodeArt::new_level_16(num_cols),
                    _ => {
                        eprintln!("Unknown preset");
                        exit(exitcode::USAGE)
                    }
                },
            );
            let mut buf = BufWriter::new(stdout());
            let _ = gen.generate(image_path, &mut buf);
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }
}
