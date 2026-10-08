# CLAUDE.md

Guidance for Claude Code in this repository. What holds in every project is in `~/.claude/CLAUDE.md`.

## What this is

**Canary** is a general-purpose programming language designed for its author and for Claude to build projects in: scripts, CLI tools, services, games. Statically typed with heavy inference, no null (`Option[T]`), typed errors (`!`), immutable by default, green threads with structured scopes and channels, garbage-collected, running on a portable bytecode VM. Tooling (formatter, LSP, package manager) is a first-class deliverable, not an afterthought. The compiler's diagnostics are the primary user interface: an agent writing Canary learns from the error messages, so they must say what is wrong, where, and what to do.

The principles are in [`docs/src/zen.md`](docs/src/zen.md); a feature that contradicts them is wrong, not the principles. [`docs/src/overview.md`](docs/src/overview.md) fixes the design envelope (notation, peers, out of scope). The language reference lives under `docs/src/spec/` as an mdbook (`mdbook serve docs`); the grammar notation is Wirth-style EBNF (`spec/notation.md`).

**[`docs/`](docs/README.md)** holds the book, the rules (`design/`: tables, no prose), the decisions (`adr/`: one per file, superseded, never rewritten) and the references (`prior-art.md`). Update them in the same change as the code they describe, but write only what is new — a spec section, a rule, a real decision, an open question — never what repeats a rule or lists what the code holds ([when a change writes there](docs/README.md#when-a-change-writes-here)). Cite ids (`L6`, `C3`, `D5`, `R1`). An ADR is started by `python3 scripts/docs.py new-adr <slug> "<decision>"`, and `adr/README.md`'s table is left alone: `docs.yml` writes it after the merge. Open questions are GitHub issues labeled `question`.

## Workspace layout

Cargo workspace, nightly toolchain (`rust-toolchain.toml`), `rustfmt.toml` applies.

| crate | what |
|---|---|
| `compiler/cyc` | the compiler driver (`cyc <file>`): CLI (`driver/`), compiler instance and source map (`ci/`), lexer-to-parser bridge and parser (`parse/`), passes (`passes/`) |
| `compiler/cyc_lexer` | the raw lexer: a cursor over bytes producing `Token { kind, len }` with no text, rustc-style |
| `compiler/cyc_ir` | shared data: `source` (positions, spans, source files), `syntax` (tokens with symbols, interner, the `Nest` AST) |
| `compiler/cyc_diag` | diagnostics: `Diagnostic`, `DiagnosticContext`, emitter trait |
| `compiler/cyc_macros` | proc macros: `#[derive(Diagnostic)]` |
| `flock` | the package manager / build tool: placeholder (`Hello, world!`) |

Naming: the compiler is `cyc`, the package manager `flock`, the AST `Nest`. Source files are `.cy` (the lexer/parser crates follow rustc's structure: `cyc_lexer` ≈ `rustc_lexer`, `parse/lexer.rs` ≈ `rustc_parse::lexer`).

## Build & test

- `cargo build` · `cargo run -p cyc -- <file.cy>` · `cargo check --workspace`
- `cargo test --workspace` — all tests. Snapshot tests use `expect-test` (`UPDATE_EXPECT=1 cargo test` rewrites them) and `insta`.
- `mdbook build docs` — the book (`docs/book/` is untracked output). CI builds it on every PR.
- Benches and repeated runs go to `ssh filaco.dev` (global rule); a quick `cargo test` on the Mac is fine.

## Commits

Conventional Commits, enforced by [cocogitto](https://docs.cocogitto.io/) (`cog`) on `commit-msg`: `type(scope): subject` (`feat(cyc): …`, `fix(cyc_macros): …`, `docs(spec): …`; `cog commit feat cyc "subject"` writes one). PRs are squash-merged and CI checks the PR **title** too. Hooks are plain scripts in `.githooks/` (`commit-msg`: `cog verify`; `pre-push`: `cargo fmt --check` and the tests), activated per clone with `git config core.hooksPath .githooks`.

**Worktrees** (`.claude/worktrees/`) go when their PR merges: a `SessionStart` hook removes, in the background, a clean, idle worktree and the local branch whose PR is merged and whose upstream GitHub deleted (`.claude/hooks/prune-merged-worktrees.sh --dry-run` shows what it would).
