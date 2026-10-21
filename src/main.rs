mod cli;

use std::process::ExitCode;

use clap::Parser;

fn main() -> ExitCode {
    match cli::dispatch(cli::Args::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            cli::report_error(&e);
            ExitCode::FAILURE
        }
    }
}
