use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Parser)]
struct CommandArgs {
    path: std::path::PathBuf,
}

fn main() {
    let args = CommandArgs::parse();
    let file = File::open(args.path).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}
