use clap::Parser;

#[derive(Parser)]
struct CommandArgs {
    path: std::path::PathBuf,
}

fn main() {
    let args = CommandArgs::parse();
    let content = std::fs::read_to_string(&args.path).expect("Could not read file!");

    for line in content.lines() {
        println!("{}", line);
    }
}
