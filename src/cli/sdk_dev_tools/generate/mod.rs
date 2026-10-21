use crate::cli;

mod syntax;

pub(in crate::cli) use syntax::Error as GenerateSyntaxError;

pub(in crate::cli) fn dispatch(args: Args) -> Result<(), cli::Error> {
    use Command::*;
    match args.cmd {
        Syntax(args) => syntax::generate(args).map_err(cli::Error::from),
    }
}

#[derive(Debug, clap::Args)]
pub(in crate::cli) struct Args {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Debug, clap::Subcommand)]
enum Command {
    /// Generate Rust syntax types from an ungrammar specification.
    Syntax(syntax::Args),
}
