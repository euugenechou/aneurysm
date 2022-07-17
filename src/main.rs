use aneurysm::Aneurysm;
use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    #[clap(value_parser)]
    path: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    Aneurysm::interpret(args.path)
}
