---
description: workflow for documentation of the code
---

1.  **Code Comments**:
    *   Ensure all public items (structs, enums, functions, traits) have `///` doc comments.
    *   Use sections in doc comments: `# Examples`, `# Panics`, `# Errors`.

2.  **Generate Docs**:
    *   Run `cargo doc --no-deps --open` to preview documentation.
    *   Check for broken links.

3.  **Project Docs**:
    *   Maintain `README.md` with:
        *   Project overview
        *   Installation instructions
        *   Quick start examples
        *   Feature list
    *   Maintain `CHANGELOG.md`.

4.  **Examples**:
    *   Ensure `examples/` directory contains runnable code demonstrating features.
