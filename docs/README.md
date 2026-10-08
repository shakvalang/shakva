# Shakva docs

| Where | What | Rule |
|---|---|---|
| [src/](src/) | the book: [zen](src/zen.md), [overview](src/overview.md), the language reference (`src/spec/`) | the spec of a feature is written in the same PR as the feature; `mdbook build docs` builds it |
| [design/](design/) | living rules of the implementation | tables and lists, no prose; changed in the same PR as the code they describe |
| [adr/](adr/) | decisions taken | one decision per file, ≤ 1 page; never rewritten, superseded by a new one |
| [issues](https://github.com/shakvalang/shakva/issues?q=is%3Aissue+label%3Aquestion) labeled `question` | open questions | a closed question becomes an ADR, a rule or a spec section, which names the issue |
| [prior-art.md](prior-art.md) | the languages and compilers Shakva is measured against, and what comes from which | updated when a reference changes, or a part of Shakva starts following one |

How to build, test and commit: [CLAUDE.md](../CLAUDE.md). How we work across the organization: [shakvalang's CONTRIBUTING](https://github.com/shakvalang/.github/blob/master/CONTRIBUTING.md).

## When a change writes here

Only when it tells the next reader something the docs and the code do not already say. A change that follows the rules written here writes nothing in `design/` or `adr/`.

| Writes | When |
|---|---|
| the spec (`src/spec/`) | the change adds or alters what a Shakva program means: a grammar rule, a typing rule, a library function |
| a rule (`design/`) | the change sets a constraint future code must keep, and the compiler does not check it |
| an ADR | there was a real choice: alternatives weighed, one taken, a reason that is not obvious from the code — or a departure from a rule or from the zen |
| a question (an issue labeled `question`) | something is knowingly left undone, and the way to do it is open |
| nothing | the list of what exists (tokens, passes, diagnostics): that is the code |
| nothing | the story of the PR, its measurements included: the PR description, with the bench left beside the code; a rule or ADR cites the conclusion and the bench's name |

One fact, one place: a rule is stated once, in `design/`; an ADR says why; the spec says what a program means; CLAUDE.md and CONTRIBUTING.md link to them rather than repeat them.

## Design

| File | Topic |
|---|---|
| [00-layers.md](design/00-layers.md) | crates, what each layer knows |
| [01-conventions.md](design/01-conventions.md) | how code is written, where Rust leaves a choice |

## Identifiers

| Prefix | What | Defined in |
|---|---|---|
| L# | layer | [00-layers.md](design/00-layers.md) |
| C# | code convention | [01-conventions.md](design/01-conventions.md) |
| D# ≡ ADR-NNNN | decision | [adr/](adr/) |
| #N | open question | its issue |
| R# | reference | [prior-art.md](prior-art.md) |

Ids are never reused. A rule dropped keeps its number, struck through; a closed question is a closed issue, and the ADR, rule or spec section that closes it names it.
