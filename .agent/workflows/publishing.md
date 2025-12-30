---
description: workflow for publishing the code
---

1.  **Pre-flight Checks**:
    *   Run `cargo test --all-features`.
    *   Run `cargo doc`.
    *   Run `cargo clippy`.
    *   Check `Cargo.toml` metadata (authors, description, license, repository, etc.).

2.  **Dry Run**:
    *   Run `cargo publish --dry-run --allow-dirty`.
    *   Inspect the list of packaged files.

3.  **Authentication**:
    *   Ensure logged in via `cargo login`.

4.  **Publish**:
    *   Run `cargo publish`.

5.  **Post-Publish**:
    *   Create a GitHub release corresponding to the version tag.
