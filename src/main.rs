use anyhow::{Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Parser)]
struct CommandArgs {
    path: std::path::PathBuf,
}

fn read_file(path: &std::path::PathBuf) -> Result<()> {
    let file = File::open(path).with_context(|| format!("Failed to open file {:?}", path))?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line.with_context(|| "Failed to read line")?);
    }
    Ok(())
}

fn main() -> Result<()> {
    let args = CommandArgs::parse();

    read_file(&args.path).with_context(|| format!("Failed to read file {:?}", args.path))?;
    Ok(())
}
