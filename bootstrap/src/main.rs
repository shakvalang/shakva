use clap::Parser;

use crate::driver::{Args, run};

mod driver;
mod parse;

fn main() {
    run(Args::parse());
}
