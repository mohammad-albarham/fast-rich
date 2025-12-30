---
description: workflow for reviewing code quality and correctness
---

When asked to review code:

1.  **Static Analysis**:
    *   Run `cargo clippy --all-features` to catch common mistakes.
    *   Run `cargo check` to ensure compilation.
    *   check for `dead_code`, `unused_imports`, or `unused_variables` warnings.

2.  **Logic & Design Review**:
    *   Check for proper error handling (no `unwrap()` in library code, use `?` or `expect()` with reason).
    *   Verify public API documentation exists.
    *   Check for potential panics or unsafe blocks.

3.  **Tests**:
    *   Ensure new functionality has corresponding unit tests.
    *   Check if tests cover edge cases (empty inputs, large inputs, etc.).

4.  **Feedback**:
    *   Provide actionable feedback.
    *   If fixing issues, verify the fix with tests.
