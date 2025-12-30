---
description: detailed workflow for coding tasks ensuring high quality and stability
---

When asking to write code or implement features, follow this workflow:

1.  **Analyze and Plan**:
    *   Understand the requirements deeply.
    *   Check `RICH_RUST_PLAN.md` (or equivalent) for existing plans.
    *   Create or update a detailed `implementation_plan.md` if the task is complex.
    *   Break down the work into phases.

2.  **Implementation**:
    *   **Phase-by-Phase**: Implement one logical phase at a time.
    *   **Compile Often**: Run `cargo check` frequently to catch errors early.
    *   **Test-Driven**: Write tests *before* or *alongside* the code.
    *   **No Broken Windows**: Do not proceed to the next phase if the current one has errors.

3.  **Verification**:
    *   Run `cargo test` after every major change.
    *   Create a demo/example file in `examples/` to visually verify the feature.
    *   Fix any compiler warnings immediately.

4.  **Refactor**:
    *   Check for strict ownership issues or lifetime problems.
    *   Ensure the API is idiomatic Rust (use Builders, Options, Results).

5.  **Commit**:
    *   Commit changes after each successful phase.
    *   Use descriptive commit messages.
