use proc_macro::TokenStream;

mod diagnostic;

#[proc_macro_derive(Diagnostic, attributes(diag, primary_span, label, note))]
pub fn derive_diagnostic(input: TokenStream) -> TokenStream {
    diagnostic::derive(input.into()).into()
}
