# UnknownLang Compiler Phases

## [ Work On ] Phase 0 — Bootstrap

### Goal

Build the first self-hostable frontend foundation.

### Features

- lexer
- Pratt parser
- AST printer
- source spans
- diagnostics

### Syntax Supported

- integer literals
- identifier expressions
- grouped expressions
- unary operators
- binary operators

### No

- types
- declarations
- functions
- runtime

### Output

- parse tree dump
- syntax diagnostics

### Notes

The AST is syntax-only.

Every node stores:

```rust id="r5y5jv"
Span { start, end }
```

---

## [ ] Phase 1A — Minimal Interpreter

### Goal

Execute simple programs quickly.

### Features

- `let`
- functions
- return
- if expressions
- booleans
- print

### Backend

- tree-walk interpreter

### No

- mutation
- loops
- arrays
- classes
- typing

### Notes

Focus entirely on:

- scopes
- environments
- function calls
- recursion

---

## [ ] Phase 1B — Mutation & Control Flow

### Goal

Add mutable state.

### Features

- `mut`
- assignment
- while
- block scopes

### Notes

This phase introduces:

- mutable bindings
- variable updates
- loop execution

---

## [ ] Phase 1C — Heap Objects

### Goal

Add runtime-managed objects.

### Features

- strings
- arrays
- indexing
- bounds checks

### Runtime

- allocator
- array object
- string object

---

## [ ] Phase 2A — Name Resolution

### Goal

Resolve symbols and scopes.

### Features

- symbol tables
- lexical scopes
- function lookup
- module-level bindings

### Output

Lower AST into semantic HIR.

---

## [ ] Phase 2B — Static Typing

### Goal

Introduce compile-time type checking.

### Features

- primitive types
- local inference
- typed expressions
- operator checking
- nullable types

### No

- generics
- traits/interfaces
- classes

### Output

Typed HIR.

---

## [ ] Phase 2C — Runtime Core

### Goal

Stabilize runtime ABI and memory model.

### Features

- allocator API
- panic system
- runtime metadata
- string runtime
- array runtime

### Decision Required

Choose memory model:

- GC
- ARC
- ownership system

---

## [ ] Phase 3 — Intermediate Representation

### Goal

Lower typed HIR into compiler IR.

### Features

- control flow graph
- SSA-ready IR
- constant folding
- basic optimization passes

### Notes

LLVM is NOT emitted directly from AST/HIR.

---

## [ ] Phase 4 — LLVM Backend

### Goal

Generate native machine code.

### Features

- LLVM lowering
- executable generation
- platform targets
- debug info

### Output

Native binaries.

---

## [ ] Phase 5 — Enums & Match

### Goal

Add algebraic data types.

### Features

- enums
- payload enums
- exhaustive match
- destructuring patterns

### Notes

Pattern matching becomes a core language feature here.

---

## [ ] Phase 6 — Classes & Interfaces

### Goal

Add object-oriented features.

### Features

- classes
- interfaces
- inheritance
- dynamic dispatch
- visibility modifiers

### Notes

Structs may be introduced alongside or before classes.

---

## [ ] Phase 7 — Generics

### Goal

Enable reusable typed abstractions.

### Features

- generic functions
- generic types
- constraints
- monomorphization

### No

- higher-kinded types
- specialization

---

## [ ] Phase 8 — Async

### Goal

Add asynchronous execution.

### Features

- async functions
- await
- task spawning
- coroutine lowering

### Runtime

- scheduler
- async task system

---

## [ ] Phase 9 — Unsafe & FFI

## Goal

Expose low-level systems programming features.

## Features

- unsafe blocks
- raw pointers
- manual allocation
- extern "C"
- unsafe interfaces

## Notes

Unsafe runtime internals may already exist before this phase.
