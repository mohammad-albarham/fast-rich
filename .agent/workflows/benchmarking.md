---
description: detailed workflow for benchmarking code including versioning and report generation
---

1.  **Setup & Versioning**:
    *   **Never Overwrite**: Always create a new versioned entry in `benchmarks/results/<lang>/<version>/`.
    *   Use `scripts/benchmark_runner.py` to ensure consistency.
    *   Ensure `benchmarks/requirements.txt` (if Python) is up to date.

2.  **Execution steps**:
    *   **Baseline**: If optimizing, establish a baseline vX.Y.Z-baseline first.
    *   **Run**: Execute benchmarks targeting the specific component.
    *   **Commit**: `git add benchmarks/results/... && git commit -m "bench: record baseline for <feature>"`

3.  **Analysis**:
    *   Compare new JSON against baseline JSON.
    *   **Optimization Validated**: If speedup > 5%, document in `CHANGELOG.md`.
    *   **Regression Check**: If slowdown > 5%, **STOP** and investigate.

4.  **Reporting**:
    *   Update `docs/benchmarks.md` with new summary tables.
    *   Update `RICH_RUST_PLAN.md` with the latest result path.
    *   **Commit**: `git commit -m "docs: update benchmark report"`
