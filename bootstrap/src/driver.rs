use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    // Input source file.
    input: PathBuf,
}

pub fn run(args: Args) {
    let Args { input } = args;

    let text = std::fs::read_to_string(
        input
            .canonicalize()
            .expect("unable to canonicalize input path"),
    )
    .expect("unable to read file");
}
