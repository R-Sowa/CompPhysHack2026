# Pauli Symbolic REPL Design

**Date:** 2026-03-06

## Goal

Build a Rust-based symbolic simplification engine for Pauli algebra, scoped for a 3-day hackathon, and expose it through a CLI/REPL so users can interactively explore expression rewrites. For the demo, the main value is that the system makes non-commutativity visible on the spot and shows which rewrite rules were used at each step.

## Product Shape

- Format: CLI-based REPL
- Supported objects: `I`, `X`, `Y`, `Z`, `i`, integer scalars, addition, multiplication, parentheses, and commutators
- Primary use case: demo, learning, and explaining foundational quantum mechanics concepts
- Non-goals:
  - general quantum circuit simplification
  - arbitrary non-commutative operator algebras
  - GUI / Web UI
  - floating-point numerical computation

## MVP Scope

The MVP should reliably handle the following inputs:

- `X*Y`
- `[X,Y]`
- `(X+Y)*(X+Y)`

Success criteria:

- it correctly simplifies representative Pauli expressions
- the REPL displays `parsed`, `steps`, and `result`
- the team can demo 2-3 stable example inputs during the presentation

## Architecture

The system is split into four layers.

1. `parser`
   - converts input strings into an AST
   - only implements the minimal grammar needed for the MVP
2. `core ast`
   - represents expressions with explicit types
   - stores `Add` and `Mul` as flattened `Vec`s
3. `simplifier`
   - handles normalization, local rewrites, and repeated application until a fixed point
   - attaches a rule name to each rewrite step
4. `repl`
   - reads one line of input and displays the parse result, rewrite trace, and final simplified form

## Data Model

Keep the internal Rust representation intentionally small.

```rust
enum Pauli {
    I,
    X,
    Y,
    Z,
}

struct Scalar {
    re: i32,
    im: i32,
}

enum Expr {
    Scalar(Scalar),
    Sym(Pauli),
    Add(Vec<Expr>),
    Mul(Vec<Expr>),
    Commutator(Box<Expr>, Box<Expr>),
}
```

`Scalar` only needs Gaussian integers of the form `a + bi`. That is enough to represent results such as `i^2 = -1` and `[X,Y] = 2iZ` exactly, without numerical error.

## Simplification Rules

The minimum useful rule set is:

- identity:
  - `I*A -> A`
  - `A*I -> A`
- squares:
  - `X*X -> I`
  - `Y*Y -> I`
  - `Z*Z -> I`
- ordered products:
  - `X*Y -> iZ`
  - `Y*Z -> iX`
  - `Z*X -> iY`
- reverse-order sign flips:
  - `Y*X -> -iZ`
  - `Z*Y -> -iX`
  - `X*Z -> -iY`
- distributivity:
  - `A*(B+C) -> AB + AC`
  - `(A+B)*C -> AC + BC`
- commutator:
  - `[A,B] -> AB - BA`
- scalar cleanup:
  - `i*i -> -1`
  - combine integer coefficients
- normalization:
  - flatten `Add` / `Mul`
  - remove zero and single-item wrappers
  - collect like terms

Instead of a fully generic rewrite engine, use a simple `normalize -> local rewrite -> repeat until fixed point` loop. That is much safer for a 3-day hackathon and makes step-trace output easier to implement.

## REPL UX

For each line of input, return three things:

- `parsed`
- `steps`
- `result`

Expected output shape:

```text
> (X+Y)*(X+Y)

parsed: (X + Y)(X + Y)
step 1: distribute -> XX + XY + YX + YY
step 2: square/product -> I + iZ - iZ + I
step 3: collect scalars -> 2I
result: 2I
```

The presentation should use these fixed demo inputs:

- `X*Y`
- `[X,Y]`
- `(X+Y)*(X+Y)`

## Error Handling

Limit errors to three categories:

- `parse error`
  - mismatched brackets, invalid tokens
- `unsupported expression`
  - unsupported symbols or unsupported syntax
- `simplification stuck`
  - the input was parsed correctly, but the current rule set cannot simplify it further

Error messages should be easy to act on immediately.

```text
unknown symbol: H (supported: I, X, Y, Z, i)
```

## Verification Strategy

Keep testing focused on a small set of strong cases.

- unit tests:
  - `X*X -> I`
  - `X*Y -> iZ`
  - `Y*X -> -iZ`
- compound-expression test:
  - `(X+Y)*(X+Y) -> 2I`
- commutator test:
  - `[X,Y] -> 2iZ`
- parser tests:
  - parentheses, addition, multiplication, commutators
- REPL smoke tests:
  - representative inputs do not crash and produce formatted output

If needed, add a test-only 2x2 matrix representation behind the scenes to validate equivalence of simplified results.

## Hackathon Narrative

Present the demo in this order:

1. `X*Y` to introduce the Pauli product rules
2. `[X,Y]` to show non-commutativity and the quantum-mechanics flavor
3. `(X+Y)*(X+Y)` to show distribution and tracked simplification over a compound expression

This sequence communicates three things quickly: there are algebraic rules, order matters, and the system can simplify larger expressions while exposing the reasoning path.
