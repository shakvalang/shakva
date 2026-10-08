use std::sync::RwLock;

use crate::{Diagnostic, DiagnosticLevel};
use DiagnosticLevel::*;

#[derive(Debug)]
pub struct DiagnosticContext {
    inner: RwLock<Vec<Diagnostic>>,
}

impl DiagnosticContext {
    pub fn new() -> Self {
        let inner = RwLock::new(Vec::new());
        Self { inner }
    }

    pub fn accumulate(&self, diag: impl Into<Diagnostic>) {
        let mut inner = self.inner.write().expect("unable to acquire write lock");
        inner.push(diag.into());
    }

    pub fn has_errors(&self) -> bool {
        let inner = self.inner.read().expect("unable to acquire read lock");
        inner.iter().any(|d| d.lvl == Error)
    }

    pub fn error_count(&self) -> usize {
        let inner = self.inner.read().expect("unable to acquire read lock");
        inner.iter().filter(|d| d.lvl == Error).count()
    }

    pub fn warning_count(&self) -> usize {
        let inner = self.inner.read().expect("unable to acquire read lock");
        inner.iter().filter(|d| d.lvl == Warning).count()
    }

    pub fn accumulated(&self) -> Vec<Diagnostic> {
        let inner = self.inner.read().expect("unable to acquire read lock");
        inner.clone()
    }
}

impl Default for DiagnosticContext {
    fn default() -> Self {
        Self::new()
    }
}
