use clap::Parser;

#[derive(Parser)]
struct CommandArgs {
    path: std::path::PathBuf,
}

fn main() {
    let args = CommandArgs::parse();
    println!("Path: {:?}", args.path);
}
