---
description: workflow for checking security vulnerabilities
---

1.  **Vulnerability Scanning**:
    *   Install `cargo-audit` if not present: `cargo install cargo-audit`.
    *   Run `cargo audit` to check for specific vulnerabilities in dependencies.

2.  **Unsafe Code Review**:
    *   Search for `unsafe` blocks: `grep -r "unsafe" src/`.
    *   Audit each block for soundness and ensure comments explain safety.

3.  **Fuzzing** (Optional):
    *   Use `cargo-fuzz` for parsing logic (markup, markdown).
