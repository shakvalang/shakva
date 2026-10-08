# ADR-0001: One lossless syntax tree, built with rowan

Status: accepted, 2026-10-09

## Context

The parser is not written and the tree it builds is open ([#2](https://github.com/shakvalang/shakva/issues/2)). Three programs will read Shakva source: the compiler, the formatter and the language server. A second parser means every grammar change is made twice, so there is one. The formatter and the server need what a classic typed AST drops: comments, whitespace, and a tree for a file that does not parse. Retrofitting a lossless tree onto a compiler that started with an AST is the known expensive path (rustc next to rust-analyzer).

## Decision

The parser builds a lossless concrete syntax tree with [rowan](https://github.com/rust-analyzer/rowan): interned green nodes, red nodes with parents and offsets, every byte of the source in the tree. The parser never fails: what it cannot parse becomes an error node and the tree goes on. The typed AST is a layer of views over that tree with no data of its own, as rust-analyzer's `ast` is.

Semantic passes do not walk the syntax tree. It is lowered to a semantic representation before names are resolved and types are checked; what that representation is, is decided when it is written.

rowan rather than a tree of our own: rowan is the Rust ecosystem's lossless tree, and writing one is justified only where it does not exist (cjls's `ginkgo`, for Cangjie).

## Consequences

- The formatter and the language server run on the compiler's parser: there is no second one.
- A grammar change is made in two places, the parser and the views, and tested once: a parser test is a snapshot of the tree it builds, errors included.
- The compiler pays for the views (a node is read through an accessor, not a field) and for keeping trivia in memory; the languages this tree comes from do, with one frontend for the compiler and the IDE.
- Closes #2.
