# Shakva lang

## Project goals

Shakva's primary goal is to be the perfect language for its creator. The creator aims to write all future personal projects in Shakva. Intended uses are as follows (but not limited to):
1. Everyday tools
2. Desktop and web applications
3. Backend services
4. Small indie games.

The long-term goal is self-hosting. The planned compiler target is bytecode for Sylva, a virtual machine to be developed as part of this project.

## Language design

Read [The Zen of Shakva](docs/zen.md) before proposing a language change. It records the project's design principles.

Use [the grammar](docs/shakva.ebnf) to look up syntax described so far. Read the comments alongside the productions for trailing lambdas, `match`, `break`, and `do … while`. The comments specify how to resolve cases where the productions alone allow multiple parses.

[shkc/parse.shk](shkc/parse.shk) is a sketch of how a parser might be written in Shakva. It explores possible grammar extensions such as `try … catch`. Both artifacts are partial. The immediate design goal is to use them together to define enough of the grammar for bootstrapping. Ask the author when a sketch leaves a design choice open.

## Implementation

- [bootstrap/src/driver.c](bootstrap/src/driver.c) implements the interactive prompt. It currently reads and echoes each line.
- [bootstrap/include/shakva/ir.h](bootstrap/include/shakva/ir.h) defines the C token and syntax tree types. [bootstrap/src/parse.c](bootstrap/src/parse.c) contains the operator binding powers and an unfinished expression parser.

## Building the C bootstrap

Run `cmake --workflow --preset build` from the repository root. The workflow configures a Ninja debug build in `build/dev` and builds the `shakva` executable. The project requires CMake 3.25 or later and defaults to C23. On a fresh build, CMake fetches `sds` and `stb` from GitHub.
