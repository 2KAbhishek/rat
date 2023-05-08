use anyhow::{Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::{self, Write};
use std::io::{BufRead, BufReader};

#[derive(Parser)]
struct CommandArgs {
    path: std::path::PathBuf,
}

fn read_file(path: &std::path::PathBuf) -> Result<std::io::BufReader<File>> {
    let file = File::open(path).with_context(|| format!("Failed to open file {:?}", path))?;
    return Ok(BufReader::new(file));
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

    let reader: BufReader<File> =
        read_file(&args.path).with_context(|| format!("Failed to read file {:?}", args.path))?;
    print_file(reader).with_context(|| format!("Failed to print file {:?}", args.path))?;
    Ok(())
}
