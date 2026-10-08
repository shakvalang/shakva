# Shakva

***The compile-time sentinel***

A general-purpose language for scripting, tools, and applications. Statically typed but it stays out of your way; safe by default; concurrent without ceremony. Its whole job is to catch trouble before it ships.

## At a glance

- **Domain** - scripting, tooling, applications.
- **Runtime** - portable bytecode VM.
- **Memory** - garbage-collected.
- **Paradigm** - multi-paradigm: procedural, functional, object.

## Attributes

| Attribute   | Rating        | Notes                                              |
|-------------|---------------|----------------------------------------------------|
| Type safety | ●●●●● 5.0     | no null, typed errors (`!`), strong static types   |
| Ergonomics  | ●●●●◐ 4.5     | heavy inference, low ceremony                      |
| Concurrency | ●●●●◐ 4.5     | green threads, structured scopes, channels         |
| Tooling     | ●●●●◐ 4.5     | formatter, LSP, package manager from day one       |
| Simplicity  | ●●●●○ 4.0     | bounded surface, one obvious way                   |
| Performance | ●●●◐○ 3.5     | managed (GC + JIT), not native                     |
| ~~Tedious~~ | ~~●○○○○ 1.0~~ | outsourced to inference, the GC, and the scheduler |

**Total - 26.0 / 30**

## What it's good at

***the locked strengths you feel day to day***

- **No surprises from absence or failure** - `Option[T]` means no null; typed errors (`!`) mean nothing fails silently.
- **Writes terse, reads clear** - you annotate intent; inference carries the rest.
- **Concurrency without the color** - ordinary functions; the runtime schedules green threads, structured scopes keep tasks honest, channels pass data safely.
- **Predictable data** - immutable by default; `struct` copies, `class` shares identity, and the kind tells you which.

## Notation

***tuned for QWERTY hands***

- `{ }` blocks;
- `[ ]` generics;
- `->` returns;
- `<:` subtyping;
- `and`/`or`/`not`.

## Ideal role

Scripts, CLI tools, services, and long-lived apps that want safety without bureaucracy.

## Out of scope

Hard real-time and bare-metal systems work.

## Closest peers

Kotlin, Swift, Cangjie; shares footwork with Go.

## Motto

> Hide the tedium; surface the hazard.
