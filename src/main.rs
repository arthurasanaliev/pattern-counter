use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: PathBuf,
}

fn main() {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");
    
    let mut total_count = 0;
    for line in content.lines() {
        let count = line.matches(&args.pattern).count();
        total_count += count;
    }

    println!("Total number of occurrences of {:?} is {}", args.pattern, total_count);
}
