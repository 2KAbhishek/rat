use anyhow::{Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::{self, Write};
use std::io::{BufRead, BufReader};

#[derive(Parser)]
struct CommandArgs {
    path: std::path::PathBuf,
}

fn read_file(path: &std::path::PathBuf) -> Result<()> {
    let file = File::open(path).with_context(|| format!("Failed to open file {:?}", path))?;
    let reader = BufReader::new(file);

    print_file(reader)?;

    Ok(())
}

fn print_file(reader: std::io::BufReader<File>) -> Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for line in reader.lines() {
        let line = line.with_context(|| format!("Failed to read line"))?;
        writeln!(handle, "{}", line)?;
    }

    Ok(())
}

fn main() -> Result<()> {
    let args = CommandArgs::parse();

    read_file(&args.path).with_context(|| format!("Failed to read file {:?}", args.path))?;

    Ok(())
}
