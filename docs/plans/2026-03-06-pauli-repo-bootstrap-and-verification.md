# Pauli Repo Bootstrap and Verification Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Initialize a Rust repository around the approved Pauli REPL design, sync it to GitHub with `gh`, track human verification in a GitHub issue, and keep user-visible behavior covered by Rust integration tests.

**Architecture:** Treat the existing design and engine implementation plans as source documents, then layer repository bootstrap, GitHub publishing, human QA, and end-to-end integration coverage on top. Keep the implementation narrow: 2x2 Pauli algebra only, CLI/REPL only, and all outward-facing verification should live under Rust `tests/` plus one human verification issue.

**Tech Stack:** Rust stable, Cargo, Git, GitHub CLI (`gh`), Markdown

---

## Dependency Notes

- Existing design documents:
  - `docs/plans/2026-03-06-pauli-repl-design.md`
  - `docs/plans/2026-03-06-pauli-repl-design-en.md`
- Existing engine implementation plan:
  - `docs/plans/2026-03-06-pauli-repl-implementation.md`
- Current workspace is not yet a git repository.
- This plan assumes `gh auth status` succeeds before remote operations.

### Task 1: Bootstrap the Local Rust Repository

**Files:**
- Create: `Cargo.toml`
- Create: `.gitignore`
- Create: `src/main.rs`
- Create: `src/lib.rs`
- Create: `README.md`
- Keep: `docs/plans/2026-03-06-pauli-repl-design.md`
- Keep: `docs/plans/2026-03-06-pauli-repl-design-en.md`
- Keep: `docs/plans/2026-03-06-pauli-repl-implementation.md`

**Step 1: Initialize the Cargo project**

Run: `cargo init --bin --name pauli_repl .`
Expected: `Cargo.toml` and `src/main.rs` are created in the current directory

**Step 2: Initialize git and add a Rust-focused ignore file**

Run: `git init`
Expected: `.git/` is created

Create `.gitignore` with at least:

```gitignore
/target
Cargo.lock
.DS_Store
```

**Step 3: Write a minimal README that points to the design docs**

Create `README.md` with:

```markdown
# Pauli REPL

Small Rust CLI/REPL for symbolic simplification of 2x2 Pauli algebra.

## Design Docs

- `docs/plans/2026-03-06-pauli-repl-design.md`
- `docs/plans/2026-03-06-pauli-repl-design-en.md`
- `docs/plans/2026-03-06-pauli-repl-implementation.md`
```

**Step 4: Verify the repository bootstraps cleanly**

Run: `cargo test`
Expected: PASS with `0 passed; 0 failed` or only the default Cargo smoke test passing

**Step 5: Commit**

```bash
git add Cargo.toml .gitignore src/main.rs src/lib.rs README.md docs/plans
git commit -m "chore: bootstrap pauli repl repository"
```

### Task 2: Implement the Engine From the Existing Plan

**Files:**
- Create: `src/scalar.rs`
- Create: `src/ast.rs`
- Create: `src/simplifier.rs`
- Create: `src/parser.rs`
- Create: `src/repl.rs`
- Modify: `src/lib.rs`
- Modify: `src/main.rs`
- Test: `tests/scalar.rs`
- Test: `tests/simplifier_products.rs`
- Test: `tests/simplifier_expansion.rs`
- Test: `tests/commutator.rs`
- Test: `tests/parser.rs`
- Test: `tests/rewrite_steps.rs`
- Test: `tests/repl_smoke.rs`

**Step 1: Open the source-of-truth implementation plan**

Read: `docs/plans/2026-03-06-pauli-repl-implementation.md`
Expected: all engine tasks are visible before writing code

**Step 2: Execute Tasks 1-7 from that plan in order**

Run the exact TDD loop already defined there:

- scalar and AST foundation
- basis product simplification
- distribution and like-term collection
- commutator support
- handwritten parser
- step-tracking simplifier output
- CLI entry point and demo readiness

Expected: the repository now contains the `src/` and `tests/` files listed above

**Step 3: Verify the engine locally**

Run: `cargo test`
Expected: PASS for all unit and integration tests introduced by the engine plan

**Step 4: Commit**

```bash
git add src tests
git commit -m "feat: implement pauli repl engine"
```

### Task 3: Promote Demo Behaviors to Explicit Integration Tests

**Files:**
- Create: `tests/design_examples.rs`
- Create: `tests/e2e_cli.rs`
- Modify: `src/repl.rs`
- Modify: `src/main.rs`

**Step 1: Write the failing integration tests for the three design examples**

Create `tests/design_examples.rs` with:

```rust
use pauli_repl::repl::render_once;

#[test]
fn test_design_example_xy() {
    let output = render_once("X*Y").unwrap();
    assert!(output.contains("result: iZ"));
}

#[test]
fn test_design_example_commutator() {
    let output = render_once("[X,Y]").unwrap();
    assert!(output.contains("result: 2iZ"));
}

#[test]
fn test_design_example_square() {
    let output = render_once("(X+Y)*(X+Y)").unwrap();
    assert!(output.contains("result: 2I"));
}
```

Create `tests/e2e_cli.rs` with:

```rust
use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn test_cli_accepts_single_expression_from_stdin() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_pauli_repl"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("binary should launch");

    child
        .stdin
        .as_mut()
        .expect("stdin should be available")
        .write_all(b"X*Y\nexit\n")
        .expect("stdin write should succeed");

    let output = child.wait_with_output().expect("binary should exit cleanly");

    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("result: iZ"));
}
```

**Step 2: Run the integration tests to verify they fail**

Run: `cargo test --test design_examples --test e2e_cli`
Expected: FAIL until deterministic CLI and example coverage are wired correctly

**Step 3: Write the minimal implementation needed for deterministic integration coverage**

Make sure:

- `render_once` is stable enough for string assertions
- `src/main.rs` supports non-interactive single-expression input from stdin
- the three hackathon demo expressions can be asserted from `tests/`

**Step 4: Run the integration tests to verify they pass**

Run: `cargo test --test design_examples --test e2e_cli`
Expected: PASS

**Step 5: Commit**

```bash
git add src/main.rs src/repl.rs tests/design_examples.rs tests/e2e_cli.rs
git commit -m "test: add integration coverage for demo behaviors"
```

### Task 4: Write the Human Verification Plan as a Local Artifact

**Files:**
- Create: `docs/qa/human-verification.md`

**Step 1: Write the failing verification checklist as a document**

Create `docs/qa/human-verification.md` with:

```markdown
# Human Verification Plan

## Local CLI walkthrough

- Run `cargo run`
- Enter `X*Y` and confirm the result is `iZ`
- Enter `[X,Y]` and confirm the result is `2iZ`
- Enter `(X+Y)*(X+Y)` and confirm the result is `2I`

## Error handling

- Enter `H` and confirm an unsupported-symbol message appears
- Enter `(X+Y` and confirm a parse error appears

## Demo readability

- Confirm the output includes `parsed`, numbered `step` lines, and `result`
- Confirm the examples are readable when projected in a terminal
```

**Step 2: Verify the checklist against the local binary**

Run: `cargo run`
Expected: the manual checklist can be walked through without missing commands

**Step 3: Refine the checklist until it is complete enough to hand to a teammate**

Add any missing sections needed for:

- startup instructions
- expected outputs
- recovery steps if the CLI gets into a bad state

**Step 4: Commit**

```bash
git add docs/qa/human-verification.md
git commit -m "docs: add human verification checklist"
```

### Task 5: Publish the Repository to GitHub with `gh`

**Files:**
- Modify: `README.md` (only if remote URL or badges are added)

**Step 1: Verify GitHub CLI authentication**

Run: `gh auth status`
Expected: authenticated status for the target GitHub account

**Step 2: Create the remote repository and push the current branch**

Run: `gh repo create pauli_repl --source=. --private --remote=origin --push`
Expected: a new GitHub repository is created and the current branch is pushed

If the repo name should match the directory instead, use:

```bash
gh repo create CompPhysHack2026 --source=. --private --remote=origin --push
```

**Step 3: Verify the remote configuration**

Run: `gh repo view --json nameWithOwner,url,defaultBranchRef`
Expected: JSON output with the created repository name and URL

### Task 6: File the Human Verification Issue

**Files:**
- Use: `docs/qa/human-verification.md`

**Step 1: Create the GitHub issue from the local checklist**

Run: `gh issue create --title "Human verification plan for Pauli REPL MVP" --body-file docs/qa/human-verification.md`
Expected: GitHub returns the created issue URL

**Step 2: Verify the issue exists remotely**

Run: `gh issue list --search "Human verification plan for Pauli REPL MVP"`
Expected: the new issue appears in the result set

### Task 7: Final Verification Before Handoff

**Files:**
- Verify: `README.md`
- Verify: `docs/plans/2026-03-06-pauli-repl-design.md`
- Verify: `docs/plans/2026-03-06-pauli-repl-design-en.md`
- Verify: `docs/qa/human-verification.md`
- Verify: `tests/design_examples.rs`
- Verify: `tests/e2e_cli.rs`

**Step 1: Run the full local verification suite**

Run: `cargo fmt --check`
Expected: PASS

Run: `cargo test`
Expected: PASS

**Step 2: Verify the remote repo and issue one more time**

Run: `gh repo view --json url`
Expected: repository URL is returned

Run: `gh issue view --json number,title,url`
Expected: the human verification issue metadata is returned

**Step 3: Commit any final doc-only cleanup**

```bash
git add README.md docs tests
git commit -m "chore: finalize repo bootstrap and verification assets"
```
