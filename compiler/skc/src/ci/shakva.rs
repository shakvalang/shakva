use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::{
    ci::{
        EarlyDiagnosticContext, ErrorsReported, SourceMap,
        diagnostic_emitter::HumanReadableDiagnosticEmitter,
    },
    parse::ParseSession,
};
use clap::ValueEnum;
use skc_diag::{DiagnosticContext, DiagnosticEmitter};
use skc_ir::syntax::SymbolInterner;

pub fn run_ci<R: Send>(
    cfg: ShakvaConfig,
    f: impl FnOnce(&Shakva) -> R + Send,
) -> Result<R, ErrorsReported> {
    let early_diag_ctx = EarlyDiagnosticContext::new();
    let dcx = DiagnosticContext::new();
    let sym_interner = SymbolInterner::new();
    let ci = Shakva {
        cfg,
        sm: SourceMap::new(),
        early_dcx: early_diag_ctx,
        dcx,
        sym_interner,
    };
    let _emit_on_drop_guard = EmitOnDrop(&ci);

    let res = f(&ci);

    if ci.dcx.has_errors() {
        Err(ErrorsReported)
    } else {
        Ok(res)
    }
}

#[derive(Debug)]
pub struct Shakva {
    pub cfg: ShakvaConfig,
    pub sm: SourceMap,
    pub early_dcx: EarlyDiagnosticContext,
    pub dcx: DiagnosticContext,
    pub sym_interner: SymbolInterner,
}

impl Shakva {
    pub fn psess<'ci>(&'ci self) -> ParseSession<'ci> {
        ParseSession::new(&self.sym_interner, &self.dcx)
    }
}

#[derive(Debug)]
pub struct ShakvaConfig {
    pub input: PathBuf,
}

#[derive(Debug)]
struct EmitOnDrop<'ci>(&'ci Shakva);

impl Drop for EmitOnDrop<'_> {
    fn drop(&mut self) {
        HumanReadableDiagnosticEmitter::new(&self.0.sm).emit_all(&self.0.dcx.accumulated());
    }
}
