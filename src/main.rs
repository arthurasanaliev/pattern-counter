use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;
    
    let mut total_count = 0;
    for line in content.lines() {
        let count = line.matches(&args.pattern).count();
        total_count += count;
    }

    println!("Total number of occurrences of {:?} in {:?} is {}", args.pattern, args.path, total_count);

    Ok(())
}