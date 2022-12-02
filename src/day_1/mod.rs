use crate::get_input_dir;
use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};
use tracing::debug;

pub mod part_1;
pub mod part_2;

pub fn get_lines() -> Lines<BufReader<File>> {
    debug!("Opening input file");
    let input_file_path = get_input_dir().join("day_1.txt");
    let input_file = File::open(&input_file_path).unwrap_or_else(|err| {
        panic!("failed to open input file @ {input_file_path:?}: {err} ({err:?})")
    });
    BufReader::new(input_file).lines()
}
