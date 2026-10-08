# Conventions

| # | Rule |
|---|---|
| C1 | No test waits for something to happen in time: what the OS, a thread or a timer does when it will comes to the code through an interface its test fakes. A timeout belongs in CI only (`timeout-minutes`), to turn a hang into a failure naming the case. |
| C2 | A performance claim comes from a bench (`criterion` or `divan`, warm-up, error bars) run on the quiet Linux box, never from a timing loop on the Mac; the table goes in the PR, the bench stays beside the code. |
| C3 | A diagnostic says what is wrong, where, and what to do, in that order; its primary span is the smallest that is wrong. The compiler's messages are the first interface a program's author sees, and for an agent the only one. |
| C4 | A grammar rule lives in the spec (`docs/src/spec/`) in the EBNF of `notation.md`, and the parser function that implements it quotes it in its doc comment; the two change in one PR. |
| C5 | Snapshot tests (`expect-test`) for anything whose output is text — tokens, trees, diagnostics; `UPDATE_EXPECT=1 cargo test` rewrites them, and the diff is reviewed like code. |
