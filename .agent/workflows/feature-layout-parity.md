---
description: Workflow for implementing Layout Parity feature
---

# Feature: Layout Parity

1. **Analysis**
   - Review `src/layout.rs` if exists.
   - Understand `rich.layout` from Python documentation/code.
   - Define missing capabilities (split ratios, depth, min_size).

2. **Implementation**
   - Implement `Layout` tree node.
   - Implement `split_row`, `split_column` with `Event` or `Ratio`.
   - Implement rendering logic to respect `Console` dimensions.

3. **Verification**
   - Create unit tests for tree manipulation.
   - Create logical tests for splitting calculations.
   - Create visual verification script (compare with Python `rich`).

4. **Finalization**
   - Run `cargo fmt` and `cargo clippy`.
   - Update `handover.md`.
