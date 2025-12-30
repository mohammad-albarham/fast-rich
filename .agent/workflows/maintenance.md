---
description: workflow for maintaining dependencies and cleanup
---

1.  **Check Outdated Dependencies**:
    *   Run `cargo outdated` (requires `cargo-outdated`) or check crates.io manually.
    *   Update minimal versions in `Cargo.toml` if necessary (e.g. for security fixes).

2.  **Update Lockfile**:
    *   Run `cargo update` to update `Cargo.lock` within compatible semver ranges.
    *   Run tests (`cargo test --all-features`) immediately after updating.

3.  **Clean Build Artifacts**:
    *   Run `cargo clean` occasionally to free disk space or resolve weird build errors.
