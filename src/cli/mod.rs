mod repl;
#[cfg(feature = "sdk_dev_tools")]
mod sdk_dev_tools;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub(super) struct Args {
    #[command(subcommand)]
    cmd: Command,
}

pub(super) fn dispatch(args: Args) {
    use Command::*;

    match args.cmd {
        Repl => todo!(),
        Build => todo!(),
        Run => todo!(),
        Test => todo!(),
        #[cfg(feature = "sdk_dev_tools")]
        Generate(args) => sdk_dev_tools::generate::dispatch(args),
    }
}

#[derive(Debug, Default, Subcommand)]
enum Command {
    #[default]
    Repl,
    Build,
    Run,
    Test,
    /// Generate source files for Shakva development.
    #[cfg(feature = "sdk_dev_tools")]
    Generate(sdk_dev_tools::generate::Args),
}
