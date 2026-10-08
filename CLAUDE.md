# CLAUDE.md

Guidance for Claude Code in this repository. What holds in every project is in `~/.claude/CLAUDE.md`.

## What this is

**Shakva** is a general-purpose programming language for its author and for Claude to build their projects in: games, web backends, desktop apps, tools. The project has two goals: that language, and learning programming language design, virtual machines included. What reads well to its author wins over what is general.

Names: the language is Shakva (`.sk` files); `sk` is the one command a user types (build, run, test, format, add a package); `skc` the compiler; **Sylva** the virtual machine. The decisions taken are the ADRs in [`docs/adr/`](docs/adr/README.md): a lossless syntax tree built with rowan (ADR-0001), a bytecode VM from the first program (ADR-0002). The zen is being rewritten ([#6](https://github.com/shakvalang/shakva/issues/6)) and the overview is gone until it is: a language feature is weighed against the ADRs and the author's taste, iteratively, not against a fixed list.

The repository is being built from scratch in **vertical slices**: a slice takes one program through every layer, source to a run on Sylva, and lands with its spec section, its tests and its docs. There is no code yet: the first slice brings the Cargo workspace, the toolchain and the CI build job with it.

## Docs

[`docs/`](docs/README.md) holds the book (`mdbook build docs`; empty until the first slice writes a spec section), the decisions (`adr/`: one per file, superseded, never rewritten) and the open questions (GitHub issues labeled `question`). A change writes there only what is new ([when a change writes here](docs/README.md#when-a-change-writes-here)). An ADR is started by `python3 scripts/docs.py new-adr <slug> "<decision>"`, and `adr/README.md`'s table is left alone: `docs.yml` writes it after the merge. `python3 scripts/docs.py check` and `python3 -m unittest discover -s scripts -p '*_test.py'` are what CI runs on the docs.

## Commits

Conventional Commits, enforced by [cocogitto](https://docs.cocogitto.io/) (`cog`) on `commit-msg`: `type(scope): subject` (`feat(skc): …`, `fix(sylva): …`, `docs(spec): …`; `cog commit feat skc "subject"` writes one). PRs are squash-merged and CI checks the PR **title** too. Hooks are plain scripts in `.githooks/` (`commit-msg`: `cog verify`; `pre-push`: the docs checks, and the build and tests once there is code), activated per clone with `git config core.hooksPath .githooks`.

**Worktrees** (`.claude/worktrees/`) go when their PR merges: a `SessionStart` hook removes, in the background, a clean, idle worktree and the local branch whose PR is merged and whose upstream GitHub deleted (`.claude/hooks/prune-merged-worktrees.sh --dry-run` shows what it would).
