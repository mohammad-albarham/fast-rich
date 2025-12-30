---
description: workflow for managing code versions and changelog
---

1.  **Check Current Version**:
    *   Read `Cargo.toml` [package] section.

2.  **Determine Bump Type**:
    *   **Major**: Breaking API changes.
    *   **Minor**: New features, backwards compatible.
    *   **Patch**: Bug fixes, backwards compatible.

3.  **Update Files**:
    *   Update `version` in `Cargo.toml`.
    *   Update `CHANGELOG.md` with a new header `[Version] - YYYY-MM-DD`.
    *   Move "Unreleased" changes to the new version section.

4.  **Tagging (Git)**:
    *   Create a git commit: `chore: Bump version to X.Y.Z`.
    *   Create a git tag: `git tag vX.Y.Z`.
