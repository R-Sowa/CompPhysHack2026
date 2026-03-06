# Pauli REPL Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build a Rust CLI/REPL that parses and symbolically simplifies Pauli algebra expressions, while showing the rewrite steps used to reach the final form.

**Architecture:** Use a small typed AST plus a deterministic simplifier loop. Keep the parser handwritten and narrow in scope, and keep the REPL focused on one-line inputs that emit `parsed`, `steps`, and `result`.

**Tech Stack:** Rust stable, Cargo, standard library only

---

## Setup Notes

- Current workspace is empty and not a git repository.
- Before Task 1, initialize both Cargo and git:

```bash
cargo init --bin --name pauli_repl .
git init
```

- After initialization, keep all work in this directory and commit after each task.

### Task 1: Scalar and AST Foundation

**Files:**
- Modify: `Cargo.toml`
- Create: `src/lib.rs`
- Create: `src/scalar.rs`
- Create: `src/ast.rs`
- Test: `tests/scalar.rs`

**Step 1: Write the failing test**

```rust
use pauli_repl::scalar::Scalar;

#[test]
fn test_i_squared_is_minus_one() {
    let i = Scalar::i();
    assert_eq!(i * i, Scalar::from_int(-1));
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_i_squared_is_minus_one --test scalar -- --exact`
Expected: FAIL because `pauli_repl::scalar::Scalar` does not exist yet

**Step 3: Write minimal implementation**

```rust
// src/scalar.rs
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Scalar {
    pub re: i32,
    pub im: i32,
}

impl Scalar {
    pub fn from_int(n: i32) -> Self {
        Self { re: n, im: 0 }
    }

    pub fn i() -> Self {
        Self { re: 0, im: 1 }
    }
}
```

```rust
// src/ast.rs
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Pauli {
    I,
    X,
    Y,
    Z,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    Scalar(crate::scalar::Scalar),
    Sym(Pauli),
    Add(Vec<Expr>),
    Mul(Vec<Expr>),
    Commutator(Box<Expr>, Box<Expr>),
}
```

```rust
// src/lib.rs
pub mod ast;
pub mod scalar;
```

Add the required arithmetic trait impls for `Scalar` in `src/scalar.rs`.

**Step 4: Run test to verify it passes**

Run: `cargo test test_i_squared_is_minus_one --test scalar -- --exact`
Expected: PASS

**Step 5: Commit**

```bash
git add Cargo.toml src/lib.rs src/scalar.rs src/ast.rs tests/scalar.rs
git commit -m "feat: add scalar and AST foundation"
```

### Task 2: Basis Product Simplification

**Files:**
- Modify: `src/lib.rs`
- Create: `src/simplifier.rs`
- Test: `tests/simplifier_products.rs`

**Step 1: Write the failing test**

```rust
use pauli_repl::ast::{Expr, Pauli};
use pauli_repl::simplifier::simplify;

#[test]
fn test_xy_reduces_to_iz() {
    let expr = Expr::Mul(vec![Expr::Sym(Pauli::X), Expr::Sym(Pauli::Y)]);
    let simplified = simplify(expr);
    assert_eq!(simplified.to_string(), "iZ");
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_xy_reduces_to_iz --test simplifier_products -- --exact`
Expected: FAIL because `simplifier::simplify` and formatting do not exist yet

**Step 3: Write minimal implementation**

Implement:

```rust
pub fn simplify(expr: Expr) -> Expr
```

Support only:

- `X*X -> I`
- `Y*Y -> I`
- `Z*Z -> I`
- `X*Y -> iZ`
- `Y*Z -> iX`
- `Z*X -> iY`
- reverse-order sign flips

Also implement `Display` for `Scalar`, `Pauli`, and `Expr` so the test can assert exact output like `"iZ"` and `"I"`.

**Step 4: Run test to verify it passes**

Run: `cargo test --test simplifier_products`
Expected: PASS for `XY`, `YX`, and square cases

**Step 5: Commit**

```bash
git add src/lib.rs src/simplifier.rs tests/simplifier_products.rs
git commit -m "feat: add Pauli basis product simplification"
```

### Task 3: Distribution and Like-Term Collection

**Files:**
- Modify: `src/simplifier.rs`
- Modify: `src/ast.rs`
- Test: `tests/simplifier_expansion.rs`

**Step 1: Write the failing test**

```rust
use pauli_repl::ast::{Expr, Pauli};
use pauli_repl::simplifier::simplify;

#[test]
fn test_x_plus_y_squared_reduces_to_2_identity() {
    let expr = Expr::Mul(vec![
        Expr::Add(vec![Expr::Sym(Pauli::X), Expr::Sym(Pauli::Y)]),
        Expr::Add(vec![Expr::Sym(Pauli::X), Expr::Sym(Pauli::Y)]),
    ]);
    let simplified = simplify(expr);
    assert_eq!(simplified.to_string(), "2I");
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_x_plus_y_squared_reduces_to_2_identity --test simplifier_expansion -- --exact`
Expected: FAIL because distribution and collection are not implemented yet

**Step 3: Write minimal implementation**

Add to the simplifier:

- flatten nested `Add` / `Mul`
- distribute multiplication over addition
- combine scalar factors inside a product
- collect identical Pauli terms in a sum

Represent terms canonically so `iZ` and `-iZ` cancel correctly.

**Step 4: Run test to verify it passes**

Run: `cargo test --test simplifier_expansion`
Expected: PASS with final output `2I`

**Step 5: Commit**

```bash
git add src/ast.rs src/simplifier.rs tests/simplifier_expansion.rs
git commit -m "feat: add distribution and term collection"
```

### Task 4: Commutator Support

**Files:**
- Modify: `src/ast.rs`
- Modify: `src/simplifier.rs`
- Test: `tests/commutator.rs`

**Step 1: Write the failing test**

```rust
use pauli_repl::ast::{Expr, Pauli};
use pauli_repl::simplifier::simplify;

#[test]
fn test_commutator_xy_reduces_to_2iz() {
    let expr = Expr::Commutator(
        Box::new(Expr::Sym(Pauli::X)),
        Box::new(Expr::Sym(Pauli::Y)),
    );
    let simplified = simplify(expr);
    assert_eq!(simplified.to_string(), "2iZ");
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_commutator_xy_reduces_to_2iz --test commutator -- --exact`
Expected: FAIL because commutator lowering is not implemented yet

**Step 3: Write minimal implementation**

Add rewrite support for:

- `[A,B] -> AB + (-1)BA`
- scalar-preserving expansion for commutator arguments

Keep the first implementation narrow: only require exact behavior for the Pauli cases used in the demo.

**Step 4: Run test to verify it passes**

Run: `cargo test --test commutator`
Expected: PASS

**Step 5: Commit**

```bash
git add src/ast.rs src/simplifier.rs tests/commutator.rs
git commit -m "feat: add commutator simplification"
```

### Task 5: Handwritten Parser

**Files:**
- Modify: `src/lib.rs`
- Create: `src/parser.rs`
- Test: `tests/parser.rs`

**Step 1: Write the failing test**

```rust
use pauli_repl::parser::parse_expr;

#[test]
fn test_parse_nested_product_and_sum() {
    let expr = parse_expr("(X+Y)*(X+Y)").unwrap();
    assert_eq!(expr.to_string(), "(X + Y)(X + Y)");
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_parse_nested_product_and_sum --test parser -- --exact`
Expected: FAIL because `parser::parse_expr` does not exist yet

**Step 3: Write minimal implementation**

Implement a small recursive-descent parser for:

- symbols: `I`, `X`, `Y`, `Z`
- scalar literal: integer
- imaginary unit: `i`
- binary `+`
- binary `*`
- grouping with `(`
- commutator syntax: `[A,B]`

Parsing precedence:

1. grouped / atom
2. multiplication
3. addition

Return a dedicated error enum with messages for unknown symbols and bracket mismatches.

**Step 4: Run test to verify it passes**

Run: `cargo test --test parser`
Expected: PASS

**Step 5: Commit**

```bash
git add src/lib.rs src/parser.rs tests/parser.rs
git commit -m "feat: add handwritten parser for Pauli expressions"
```

### Task 6: Step-Tracking Simplifier Output

**Files:**
- Modify: `src/simplifier.rs`
- Create: `src/repl.rs`
- Test: `tests/rewrite_steps.rs`

**Step 1: Write the failing test**

```rust
use pauli_repl::repl::render_once;

#[test]
fn test_render_once_includes_steps_and_result() {
    let output = render_once("(X+Y)*(X+Y)").unwrap();
    assert!(output.contains("step 1:"));
    assert!(output.contains("result: 2I"));
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_render_once_includes_steps_and_result --test rewrite_steps -- --exact`
Expected: FAIL because `render_once` and step tracking do not exist yet

**Step 3: Write minimal implementation**

Change simplifier return type to something like:

```rust
pub struct SimplifyOutput {
    pub parsed: String,
    pub steps: Vec<RewriteStep>,
    pub result: Expr,
}
```

Each `RewriteStep` should record:

- rule name
- before
- after

Implement `render_once(input: &str) -> Result<String, ReplError>` that parses, simplifies, and formats the output block.

**Step 4: Run test to verify it passes**

Run: `cargo test --test rewrite_steps`
Expected: PASS

**Step 5: Commit**

```bash
git add src/simplifier.rs src/repl.rs tests/rewrite_steps.rs
git commit -m "feat: show rewrite steps for each expression"
```

### Task 7: CLI Entry Point and Demo Readiness

**Files:**
- Modify: `src/lib.rs`
- Create: `src/main.rs`
- Create: `README.md`
- Test: `tests/repl_smoke.rs`

**Step 1: Write the failing test**

```rust
use pauli_repl::repl::render_once;

#[test]
fn test_demo_commutator_output_is_stable() {
    let output = render_once("[X,Y]").unwrap();
    assert!(output.contains("result: 2iZ"));
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_demo_commutator_output_is_stable --test repl_smoke -- --exact`
Expected: FAIL until the REPL entrypoint and final formatting are wired together

**Step 3: Write minimal implementation**

Implement `src/main.rs` with:

- welcome banner
- prompt `> `
- `help` command
- `examples` command
- `quit` / `exit`
- single-line expression handling through `render_once`

Document the 3 hackathon demo commands in `README.md`.

**Step 4: Run test to verify it passes**

Run: `cargo test`
Expected: PASS for all test files

Run: `cargo run`
Expected: interactive prompt opens and the three demo expressions produce stable output

**Step 5: Commit**

```bash
git add src/lib.rs src/main.rs README.md tests/repl_smoke.rs
git commit -m "feat: add CLI repl and demo docs"
```

## Final Verification

Run these commands before any demo or handoff:

```bash
cargo fmt --check
cargo test
cargo run
```

Expected outcomes:

- formatting passes
- all tests pass
- REPL accepts `X*Y`, `[X,Y]`, `(X+Y)*(X+Y)`
- output shows `parsed`, numbered `step` lines, and `result`
