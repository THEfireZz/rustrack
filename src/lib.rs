use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Args {
    pub source_path: PathBuf,
    pub target_apth: PathBuf,
}

pub fn read_file(file_path: &PathBuf) -> String {
    std::fs::read_to_string(file_path).expect("could not read file")
}
