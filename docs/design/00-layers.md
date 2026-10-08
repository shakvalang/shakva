# Layers

Crates of the workspace, from the bottom up. A layer knows the ones below it and nothing of the ones above.

| # | Layer | Knows | Does not know |
|---|---|---|---|
| L1 | `skc_lexer` | bytes | text, spans, symbols, diagnostics: a token is a kind and a length, the caller keeps the position |
| L2 | `skc_ir` | positions, spans, source files, symbols, tokens, the syntax tree | I/O, how a diagnostic is shown |
| L3 | `skc_diag`, `skc_macros` | a diagnostic's shape and how it is collected (`DiagnosticContext`) | where it goes: an emitter is a trait, the driver's |
| L4 | `skc` | files, the source map, the parser, the passes, the driver | the editor, the package manager |
| L5 | `flock` | packages, builds, the commands a user types | the compiler's internals: it drives `skc` as a library or a process |

Rules:

| # | Rule |
|---|---|
| L6 | Diagnostics are collected, never printed where they arise: a pass adds to the `DiagnosticContext`, the driver emits once (`EmitOnDrop`). A language server will read the same context. |
| L7 | No global mutable state: the interner, the source map and the diagnostics live in the compiler instance (`Shakva`), handed down by reference. |
