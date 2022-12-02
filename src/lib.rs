use std::{env::current_dir, path::PathBuf};
use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};
use tracing::debug;

pub mod day_1;
pub mod day_2;

pub fn get_input_dir() -> PathBuf {
    current_dir()
        .expect("failed to get working directory")
        .join("input")
}

pub fn get_lines(file: &str) -> Lines<BufReader<File>> {
    debug!("Opening input file");
    let input_file_path = get_input_dir().join(file);
    let input_file = File::open(&input_file_path).unwrap_or_else(|err| {
        panic!("failed to open input file @ {input_file_path:?}: {err} ({err:?})")
    });
    BufReader::new(input_file).lines()
}

#[cfg(test)]
#[ctor::ctor]
fn init_logger() {
    use tracing_subscriber::EnvFilter;

    tracing_subscriber::fmt()
        .compact()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}
