---
description: strict workflow for publishing releases to crates.io and PyPI
---

1.  **Pre-Release Check**:
    *   **Tests**: `cargo test --workspace` MUST pass.
    *   **Lints**: `cargo clippy --workspace -- -D warnings` MUST be clean.
    *   **Docs**: `cargo doc --no-deps` MUST succeed.
    *   **Clean Git**: `git status` MUST be clean.

2.  **Versioning**:
    *   Bump version in `Cargo.toml`.
    *   Bump version in `bindings/python/Cargo.toml`.
    *   Update `CHANGELOG.md` with release notes and date.
    *   **Commit**: `git commit -am "chore: release vX.Y.Z"`

3.  **Dry Run**:
    *   `cargo publish --dry-run`
    *   `maturin build` (ensure wheels generated)

4.  **Release Interaction**:
    *   **ASK USER** for confirmation to publish.
    *   On approval:
        1.  `cargo publish`
        2.  `git tag vX.Y.Z`
        3.  `git push origin vX.Y.Z`

5.  **Post-Release**:
    *   Update `RICH_RUST_PLAN.md` start next iteration.
