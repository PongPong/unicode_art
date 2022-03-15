use clap::ArgMatches;
use std::process::exit;

use crate::unicode_art::braille::DEFAULT_THRESHOLD;

pub trait NumColumns {
    fn num_cols(&self, min_cols: u32, default_cols: u32) -> u32;
}

impl NumColumns for ArgMatches {
    fn num_cols(&self, min_cols: u32, default_cols: u32) -> u32 {
        let num_cols = self.value_of("NUM_COLS").map_or(default_cols, |val| {
            val.parse::<u32>().expect("Invalid num_cols.")
        });
        if num_cols <= min_cols {
            eprintln!("Invalid num_cols. minimum cols: {}", min_cols);
            exit(exitcode::USAGE)
        }
        num_cols
    }
}

pub trait BrailleThreshold {
    fn threshold(&self) -> u8;
}

impl BrailleThreshold for ArgMatches {
    fn threshold(&self) -> u8 {
        self.value_of("THRESHOLD").map_or(DEFAULT_THRESHOLD, |val| {
            val.parse::<u8>().expect("Invalid threshold")
        })
    }
}
