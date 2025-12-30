---
description: workflow for reviewing code quality, correctness, and enforcing professional standards
---

When asked to review code:

1.  **Strict Linting & Formatting**:
    *   **Format**: Run `cargo fmt --all`.
    *   **CI Compliance**: Run `cargo fmt --all -- --check` to guarantee CI passes.
    *   **Lint**: Run `cargo clippy --workspace -- -D warnings`.
    *   **Commit Fixes**: If the review triggers changes, `git commit -m "style: apply review fixes"` immediately.

2.  **Safety Logic**:
    *   **Unsafe Audit**: Grep for `unsafe`. If found, it MUST have a `// SAFETY:` comment explaining why it holds.
    *   **Error Handling**: No `unwrap()` in `src/` (library code). Use `?` and custom errors.
    *   **Public API**: Ensure all `pub` items have doc strings.

3.  **Test Coverage**:
    *   Run `cargo test --workspace`.
    *   If a bug is found, write a reproduction test case *first*.

4.  **Documentation**:
    *   Check `README.md` and `CHANGELOG.md` are consistent with the code.
    *   Ensure `examples/` compile and run.

5.  **Final Polish**:
    *   Does code look "Professional"? (Consistent naming, no commented-out blocks, no "TODOs" left in critical paths).
