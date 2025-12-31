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
        *   **Recommended**: Use `./scripts/auto_commit.sh "<message>"` to run fmt, clippy, test, and commit in one go.
        *   Manual: `git add <files> && git commit -m "feat: <concise description>"`
        *   *Never accumulate multiple features in one uncommitted state.*

3.  **Refactor & Polish**:
    *   Review API ergonomics (Builders, specific types).
    *   Ensure no `unsafe` unless strictly documented and necessary.
    *   **Commit**: `git commit -m "refactor: <description>"`

4.  **Final Verification**:
    *   Run full suite: `cargo test --workspace`.
    *   Check examples: Run a relevant example from `examples/`.

**Rule of Thumb**: Commit **immediately** after completing a logical, verifiable step (e.g., a passing test, a single function, or a refactor). Prioritize **frequency and stability** over "feature completeness" in a single commit.

