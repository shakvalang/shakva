# ADR-0002: A bytecode VM, Sylva, from the first program

Status: accepted, 2026-10-09

## Context

Nothing runs yet, and the first program needs a runtime ([#3](https://github.com/shakvalang/shakva/issues/3)). Shakva's runtime is green threads in structured scopes with channels, on a garbage collector. The question is whether to start with a tree-walking interpreter as a throwaway, compile to an existing VM, or write the VM first. Learning how virtual machines are designed is one of the project's two goals, next to the language itself.

## Decision

Shakva compiles to bytecode for **Sylva**, its own virtual machine written in Rust, from the first program that runs. There is no tree-walking interpreter, not even as a prototype, and no compilation to an existing VM.

Sylva is named after the river the Shakva flows into. How it is built is not decided here: a stack or a register machine, the frame layout, the calling convention, the garbage collector, the scheduler are each an ADR, taken when the vertical slice that needs it lands.

Why: a green thread on a tree-walking interpreter in Rust needs a native stack of its own or continuation-passing; on a VM with frames on the heap a thread is a pointer to its frame. A prototype that is kept is the expensive path, and an existing VM has neither the scheduler nor the collector the language promises.

## Consequences

- Slower to the first program: the first slice is the whole pipeline, source to a run on Sylva, for one expression.
- The VM is a stage-1 deliverable, not a later one; concurrency is built on it, not around it.
- No JIT is promised. One is a decision of its own, taken or not when a measurement asks for it.
- Closes #3.
