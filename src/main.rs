use clap::Parser;
use rustrack::{Args, read_file};

fn main() {
    let args = Args::parse();

    println!("{}", read_file(&args.target_apth));
    println!("{}", read_file(&args.source_path));
}
