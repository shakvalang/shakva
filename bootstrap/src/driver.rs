use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    input: PathBuf,
}

pub fn run(args: Args) {}
