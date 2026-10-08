use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
pub struct ShakvaDriver {
    // Input source.
    pub input: PathBuf,
}

impl Default for ShakvaDriver {
    fn default() -> Self {
        ShakvaDriver::parse()
    }
}
