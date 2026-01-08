use clap::Parser;
use std::fs::read_to_string;
mod days;
mod util;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Arguments {
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    let args = Arguments::parse();

    let content = read_to_string(&args.path).expect("could not read file");

    let answer = days::day3::run(&content);
    println!("Answer: {:?}", answer)
}
