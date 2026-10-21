use clap::{Parser, Subcommand};

pub fn execute(driver_args: DriverArgs) {
    match driver_args.cmd {
        Command::Repl => repl(),
    }
}

fn repl() {
    println!("hello, repl")
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct DriverArgs {
    #[command(subcommand)]
    pub cmd: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Repl,
}
