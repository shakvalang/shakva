#[derive(Debug)]
pub struct ErrorsReported;

/// Used as a return value to signify a fatal error occurred.
#[derive(Copy, Clone, Debug)]
#[must_use]
pub struct FatalError;

pub(crate) struct FatalErrorMarker;

impl FatalError {
    pub fn raise(self) -> ! {
        std::panic::resume_unwind(Box::new(FatalErrorMarker))
    }
}

impl std::fmt::Display for FatalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fatal error")
    }
}

impl std::error::Error for FatalError {}

/// Runs a closure and catches unwinds triggered by fatal errors.
///
/// The compiler currently unwinds with a special sentinel value to abort
/// compilation on fatal errors. This function catches that sentinel and turns
/// the panic into a `Result` instead.
pub fn catch_fatal_errors<F: FnOnce() -> R, R>(f: F) -> Result<R, FatalError> {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)).map_err(|value| {
        if value.is::<FatalErrorMarker>() {
            FatalError
        } else {
            std::panic::resume_unwind(value);
        }
    })
}
