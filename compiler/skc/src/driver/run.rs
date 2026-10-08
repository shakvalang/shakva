use std::{
    path::PathBuf,
    process::{ExitCode, Termination},
};

use crate::{
    ci::{EarlyDiagnosticContext, FatalErrorMarker, ShakvaConfig, catch_fatal_errors, run_ci},
    driver::ShakvaDriver,
    passes,
};

impl ShakvaDriver {
    pub fn run(self) -> ExitCode {
        install_ice_hook();
        catch_with_exit_code(|| self.run_impl())
    }

    fn run_impl(self) -> ExitCode {
        let early_diag_ctx = EarlyDiagnosticContext::new();

        let input = self.input.canonicalize().unwrap_or_else(|e| {
            early_diag_ctx.fatal(std::format!(
                "unable to canonicalize path `{}`: {e}",
                self.input.to_string_lossy()
            ))
        });

        let cfg = ShakvaConfig { input };

        match run_ci(cfg, |ci| {
            let _ = passes::parse_nest(ci);
        }) {
            Ok(_) => ExitCode::SUCCESS,
            Err(_errors_reported) => ExitCode::FAILURE,
        }
    }
}

fn catch_with_exit_code<T: Termination>(f: impl FnOnce() -> T) -> ExitCode {
    match catch_fatal_errors(f) {
        Ok(status) => status.report(),
        _ => ExitCode::FAILURE,
    }
}

fn install_ice_hook() {
    let next_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        if info.payload().is::<FatalErrorMarker>() {
            // deliberate abort, already diagnosed
            return;
        }

        next_hook(info);
    }));
}
