use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    //println!("path: {:?}", args.path)

    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    let mut dial: i32 = 50;
    let mut counter: i32 = 0;

    for line in content.lines() {
        dial += match line.split_at(1) {
            ("R", amount) => amount.parse::<i32>().unwrap(),
            ("L", amount) => -amount.parse::<i32>().unwrap(),
            _ => 0,
        };
        let digits = dial.to_string().chars().rev().take(2).collect::<String>();

        println!("line: {:?}, dial: {:?}, digits: {:?}", line, dial, digits);

        if dial == 0 || digits == "00" {
            counter += 1;
        }
    }
    println!("zeros: {:?}", counter)
}
