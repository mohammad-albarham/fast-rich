---
description: workflow for benchmarking the code
---

1.  **Setup**:
    *   Use `criterion` crate for robust micro-benchmarks.
    *   Create a `benches/` directory.

2.  **Write Benchmarks**:
    *   Identify critical paths (e.g., parsing markup, rendering large tables).
    *   Write benchmark functions comparing different inputs/sizes.

3.  **Run Benchmarks**:
    *   Run `cargo bench`.
    *   Compare results with previous runs (baseline).

4.  **Analyze**:
    *   Look for regressions.
    *   Identify bottlenecks.

5.  **Optimize**:
    *   Profile the code (e.g., using `flamegraph`).
    *   Apply optimizations.
    *   Verify with benchmarks.
