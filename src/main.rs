use clap::Parser;
//mod util;
// mod day1;
// mod day2;
// mod day3;
//mod day4;
// mod day5;
// mod day6;
//mod day7;
// mod day8;
// mod day9;
// mod day10;
mod day11;
//mod day12;

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

    let answer = day11::run(&content);
    println!("Answer: {:?}", answer)
    //day2::part2(content.clone());
}
