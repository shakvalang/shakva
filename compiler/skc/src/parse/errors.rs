use skc_ir::source::Span;
use skc_macros::Diagnostic;

#[derive(Diagnostic)]
#[diag(error, "no valid digits found for number")]
pub(super) struct NoDigitsLiteral {
    #[primary_span]
    pub span: Span,
}

#[derive(Diagnostic)]
#[diag(error, "unknown start of token: `{escaped}`")]
pub(super) struct UnknownTokenStart {
    #[primary_span]
    pub span: Span,
    pub escaped: String,
}

/// Pushes a character to a message string for error reporting
pub(super) fn escaped_char(c: char) -> String {
    match c {
        '\u{20}'..='\u{7e}' => {
            // Don't escape \, ' or " for user-facing messages
            c.to_string()
        }
        _ => c.escape_default().to_string(),
    }
}
