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

pub(super) fn dispatch(args: Args) -> Result<(), Error> {
    use Command::*;

    match args.cmd {
        Build => todo!(),
        Run => todo!(),
        Test => todo!(),
        #[cfg(feature = "sdk_dev_tools")]
        Generate(args) => sdk_dev_tools::generate::dispatch(args),
    }
}

#[derive(Debug, Subcommand)]
enum Command {
    Build,
    Run,
    Test,
    /// Generate source files for Shakva development.
    #[cfg(feature = "sdk_dev_tools")]
    Generate(sdk_dev_tools::generate::Args),
}

#[derive(Debug, thiserror::Error)]
pub(super) enum Error {
    #[cfg(feature = "sdk_dev_tools")]
    #[error(transparent)]
    GenerateSyntax(#[from] sdk_dev_tools::generate::GenerateSyntaxError),
}

pub(super) fn report_error(e: &Error) {
    eprintln!("error: {}", e);
    let mut source = std::error::Error::source(e);
    while let Some(cause) = source {
        eprintln!("  caused by: {cause}");
        source = cause.source();
    }
}
