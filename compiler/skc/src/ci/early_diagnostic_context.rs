use yansi::Paint;

use crate::ci::FatalError;

#[derive(Debug)]
pub struct EarlyDiagnosticContext;

impl EarlyDiagnosticContext {
    pub fn new() -> Self {
        Self
    }

    pub fn fatal(&self, msg: impl std::fmt::Display) -> ! {
        eprintln!("{}: {msg}", "error".bright_red().bold());
        FatalError.raise()
    }
}

impl Default for EarlyDiagnosticContext {
    fn default() -> Self {
        Self::new()
    }
}
