---
description: detailed workflow for coding tasks ensuring high quality, atomic commits, and stability
---

When asking to write code or implement features, strictly follow this workflow:

1.  **Analyze and Plan**:
    *   Understand the requirements deeply.
    *   Check `RICH_RUST_PLAN.md` for context.
    *   If complex, create an `implementation_plan.md` first.

2.  **Implementation Loop (Per Feature)**:
    *   **Atomic Scope**: Work on one small, verifiable feature at a time.
    *   **Test-Driven**: Write tests *alongside* the code.
    *   **Verify**: Run `cargo check` and `cargo test` immediately.
    *   **Lint**: Run `cargo clippy` and `cargo fmt --all`.
    *   **Strict Check**: You MUST run `cargo fmt --all -- --check` to verify CI compliance.
    *   **Commit**: **IMMEDIATELY** add and commit verification passes.
        *   Command: `git add <files> && git commit -m "feat: <concise description>"`
        *   *Never accumulate multiple features in one uncommitted state.*

3.  **Refactor & Polish**:
    *   Review API ergonomics (Builders, specific types).
    *   Ensure no `unsafe` unless strictly documented and necessary.
    *   **Commit**: `git commit -m "refactor: <description>"`

4.  **Final Verification**:
    *   Run full suite: `cargo test --workspace`.
    *   Check examples: Run a relevant example from `examples/`.

**Rule of Thumb**: If you have written >50 lines of code without a commit, you are doing it wrong.
