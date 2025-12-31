---
description: Workflow for implementing comprehensive box style parity with Python rich.box
---

1. Analysis & Design
   - Review `src/panel.rs` and `src/table.rs`.
   - Research `rich.box` styles (ASCII, DOUBLE, ROUNDED, HEAVY, etc.).
   - Create `implementation_plan.md`.

2. Implementation
   - Implement `Box` structs/styles ensuring correct Unicode characters.
   - Update `table.rs` and `panel.rs` to use the new system.

3. Verification
   - Create `examples/box_styles.rs` to render a grid of styles.
   - Add unit tests.
   - Run `cargo test` and verify examples.
