# Shakva docs

| Where | What | Rule |
|---|---|---|
| [src/](src/) | the book: the language reference (`src/spec/`), empty until the first slice | the spec of a feature is written in the same PR as the feature; `mdbook build docs` builds it |
| [adr/](adr/) | decisions taken | one decision per file, ≤ 1 page; never rewritten, superseded by a new one |
| [issues](https://github.com/shakvalang/shakva/issues?q=is%3Aissue+label%3Aquestion) labeled `question` | open questions | a closed question becomes an ADR or a spec section, which names the issue |

How to build, test and commit: [CLAUDE.md](../CLAUDE.md). How we work across the organization: [shakvalang's CONTRIBUTING](https://github.com/shakvalang/.github/blob/master/CONTRIBUTING.md).

## When a change writes here

Only when it tells the next reader something the docs and the code do not already say.

| Writes | When |
|---|---|
| the spec (`src/spec/`) | the change adds or alters what a Shakva program means: a grammar rule, a typing rule, a library function |
| an ADR | there was a real choice: alternatives weighed, one taken, a reason that is not obvious from the code |
| a question (an issue labeled `question`) | something is knowingly left undone, and the way to do it is open |
| nothing | the list of what exists (tokens, passes, diagnostics): that is the code |
| nothing | the story of the PR, its measurements included: the PR description, with the bench left beside the code; an ADR cites the conclusion and the bench's name |

One fact, one place: an ADR says why; the spec says what a program means; CLAUDE.md and CONTRIBUTING.md link to them rather than repeat them.

## Identifiers

| Prefix | What | Defined in |
|---|---|---|
| D# ≡ ADR-NNNN | decision | [adr/](adr/) |
| #N | open question | its issue |

Ids are never reused: a closed question is a closed issue, and the ADR or spec section that closes it names it.
