use std::sync::RwLock;

use crate::diag::{Diag, DiagLvl};
use DiagLvl::*;

#[derive(Debug)]
pub struct DiagCtx {
    inner: RwLock<Vec<Diag>>,
}

impl DiagCtx {
    pub fn new() -> Self {
        let inner = RwLock::new(vec![]);
        Self { inner }
    }

    pub fn accumulate(&self, diag: impl Into<Diag>) {
        let diag = diag.into();
        let mut inner = self.inner.write().expect("unable to acquire write lock");
        inner.push(diag);
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

    pub fn accumulated(&self) -> Vec<Diag> {
        let inner = self.inner.read().expect("unable to acquire read lock");
        inner.clone()
    }
}

impl Default for DiagCtx {
    fn default() -> Self {
        Self::new()
    }
}
