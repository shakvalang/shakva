use crate::Diagnostic;

pub trait DiagnosticEmitter {
    fn emit(&self, diag: &Diagnostic);
    fn emit_all(&self, diags: &[Diagnostic]) {
        for diag in diags {
            self.emit(diag);
        }
    }
}
