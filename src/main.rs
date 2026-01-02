use clap::Parser;
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
    //println!("path: {:?}", args.path)

    let content =
        std::fs::read_to_string(&args.path).expect("could not read file");

    let answer = days::day2::run(&content);
    println!("Answer: {:?}", answer)
    //day2::part2(content.clone());
}
