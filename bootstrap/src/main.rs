use clap::Parser;

use shkc::driver::{Args, run};

fn main() {
    run(Args::parse());
}
