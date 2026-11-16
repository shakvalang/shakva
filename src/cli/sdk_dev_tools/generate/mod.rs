mod syntax;

pub(in crate::cli) fn dispatch(args: Args) {
    use Command::*;

    match args.cmd {
        Syntax(args) => syntax::generate(args),
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
