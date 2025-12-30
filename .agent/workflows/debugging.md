---
description: workflow for fixing bugs and issues
---

1.  **Reproduce**:
    *   Create a minimal reproduction code snippet.
    *   Create a new test case in a `tests/` file that fails (demonstrates the bug).

2.  **Diagnose**:
    *   Use logging (`trace!`, `debug!`) or `inspect!` to trace execution.
    *   Analyze the stack trace if it's a panic.

3.  **Fix**:
    *   Apply the fix.
    *   Ensure the reproduction test case now passes.

4.  **Regression Test**:
    *   Keep the reproduction test in the codebase to prevent regression.
    *   Run full test suite.
