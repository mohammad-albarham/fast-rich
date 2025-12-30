---
description: workflow for testing the library
---

1.  **Unit Tests**:
    *   Run `cargo test --lib` to test core logic.
    *   Ensure all modules have a `#[cfg(test)] mod tests` section.

2.  **Integration Tests**:
    *   Run integration tests in `tests/` directory (if any) via `cargo test --test '*'`.

3.  **Doc Tests**:
    *   Run `cargo test --doc` to verify code examples in comments.

4.  **Feature Permutations**:
    *   Run tests with all features: `cargo test --all-features`.
    *   Run tests with no features: `cargo test --no-default-features`.

5.  **Visual Verification**:
    *   Run examples to ensure output looks correct: `cargo run --example <name>`.
