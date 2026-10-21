use clap::Parser;
use shakva::driver::{self, DriverArgs};

fn main() {
    driver::execute(DriverArgs::parse())
}
