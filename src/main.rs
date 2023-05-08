mod rat;
use crate::rat::{print_file, read_file};
use anyhow::{Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::BufReader;

#[derive(Parser)]
struct CommandArgs {
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = CommandArgs::parse();

    let reader: BufReader<File> =
        read_file(&args.path).with_context(|| format!("Failed to read file {:?}", args.path))?;
    print_file(reader).with_context(|| format!("Failed to print file {:?}", args.path))?;
    Ok(())
}
