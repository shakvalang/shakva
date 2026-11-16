mod cli;

use clap::Parser;

fn main() {
    cli::dispatch(cli::Args::parse())
}
