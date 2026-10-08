use skc_diag::DiagnosticContext;
use skc_ir::syntax::SymbolInterner;

#[derive(Clone, Copy, Debug)]
pub struct ParseSession<'ci> {
    pub sym_interner: &'ci SymbolInterner,
    pub dcx: &'ci DiagnosticContext,
}

impl<'ci> ParseSession<'ci> {
    pub fn new(sym_interner: &'ci SymbolInterner, dcx: &'ci DiagnosticContext) -> Self {
        Self { sym_interner, dcx }
    }
}
