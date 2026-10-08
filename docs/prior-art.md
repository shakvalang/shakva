# Prior art

The languages and implementations Shakva is measured against, in two roles: **references**, which Shakva learns from, and **peers**, which a user picks Shakva over or not. Cite them by id: "as R1 does". Facts as of 2026-10; a row that goes stale is updated, not kept.

## References

| # | Project | For Shakva |
|---|---|---|
| R1 | [rustc](https://github.com/rust-lang/rust) (`compiler/rustc_lexer`, `rustc_span`, `rustc_errors`) | **the model for the frontend's structure**: a text-free lexer, the source map, diagnostics as data with a derive |
| R2 | [rust-analyzer](https://github.com/rust-lang/rust-analyzer) | the tooling-grade frontend: a lossless tree, an error-resilient parser, one syntax for compiler, formatter and server |
| R3 | [Roslyn](https://github.com/dotnet/roslyn) | one frontend for the compiler and the IDE from the first day; red-green trees |
| R4 | [Go](https://go.dev) (`gopls`, `go` tool) | goroutines, channels, structured concurrency without colored functions; one tool for build, test, format, vet |
| R5 | [Kotlin](https://kotlinlang.org) | the nearest syntax and semantics: `val`/`var`, classes and interfaces, extensions, named arguments, inference |
| R6 | [Swift](https://swift.org) | `struct` copies and `class` shares identity; `Optional`; typed `throws` |
| R7 | [Cangjie](https://cangjie-lang.cn) | a peer of the same shape; what its tooling got wrong is known first-hand (cjls) |

## Peers

| Language | Shakva's answer |
|---|---|
| Kotlin (R5) | no JVM, no null, no coroutine coloring |
| Swift (R6) | no ARC, no Apple |
| Cangjie (R7) | the same envelope, tooling first |
| Go (R4) | types that carry intent: no `nil`, no `if err != nil`, generics that are not an afterthought |
