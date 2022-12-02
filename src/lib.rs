use std::{env::current_dir, path::PathBuf};

pub mod day_1;

pub fn get_input_dir() -> PathBuf {
    current_dir()
        .expect("failed to get working directory")
        .join("input")
}

#[cfg(test)]
#[ctor::ctor]
fn init_logger() {
    tracing_subscriber::fmt().compact().init();
}
