// Common functions for rat

use anyhow::{Context, Result};
use std::fs::File;
use std::io::{self, Write};
use std::io::{BufRead, BufReader};

pub fn read_file(path: &std::path::PathBuf) -> Result<std::io::BufReader<File>> {
    let file = File::open(path).with_context(|| format!("Failed to open file {:?}", path))?;
    return Ok(BufReader::new(file));
}

pub fn print_file(reader: std::io::BufReader<File>) -> Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for line in reader.lines() {
        let line = line.with_context(|| format!("Failed to read line"))?;
        writeln!(handle, "{}", line)?;
    }
    Ok(())
}

#[cfg(test)]
#[test]
fn read_file_test() {
    let path = std::path::PathBuf::from("src/rat.rs");
    let reader = read_file(&path).unwrap();
    assert_eq!(
        reader.lines().next().unwrap().unwrap(),
        "// Common functions for rat"
    );
}

#[test]
fn print_file_test() {
    let path = std::path::PathBuf::from("src/rat.rs");
    let reader = read_file(&path).unwrap();
    let out = print_file(reader).unwrap();
    assert_eq!(out, ());
}
