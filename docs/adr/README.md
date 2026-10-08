# ADR

One decision per file, ≤ 1 page: the decision and why, not how it was reached — a benchmark's table goes in the PR, the ADR keeps the result. An accepted ADR is not rewritten: a new one replaces it, saying "supersedes ADR-NNNN". In discussion `D7` ≡ `ADR-0007`.

An ADR is started by `python3 scripts/docs.py new-adr <slug> "<decision>"`, which takes the next number free on master and in the open pull requests; the number is the PR's from then on. The table below is written from the headers after the merge: a PR does not touch it. What an ADR changes in an older one, short of replacing it, is an `Amends` line of the newer one's header, shown in the older one's row.

```md
# ADR-NNNN: <decision>

Status: proposed | accepted | superseded by ADR-NNNN, <date>
Amends ADR-NNNN: <what>        (none, or one per ADR it amends)

## Context
## Decision
## Consequences
```

<!-- written by `python3 scripts/docs.py index` after a merge: never by hand -->

| # | Decision | Status |
|---|---|---|
