# Benchmarks Report

> **Version**: v0.1.1
> **Date**: 2025-12-30
> **Platform**: macOS (Apple Silicon)

This document details the performance of `rich-rust` compared to the reference Python `rich` library. All benchmarks measure the wall-clock time to render a fixed workload (N iterations).

## 🏆 Executive Summary

`rich-rust` delivers **extreme performance improvements** for layout and core rendering tasks (10x-24x faster). For heavy parsing tasks (Markdown, Syntax), it provides significant 2x-10x speedups.

| Component | Python `rich` (s) | `rich-rust` (s) | Speedup | Notes |
| :--- | :---: | :---: | :---: | :--- |
| **Logging** | 3.66s | **0.15s** | 🚀 **24.4x** | High-throughput logging |
| **Columns** | 1.81s | **0.08s** | 🚀 **23.4x** | Layout calculation |
| **Table** | 0.25s | **0.01s** | 🚀 **17.7x** | Auto-sizing logic |
| **Traceback** | 1.67s | **0.09s** | 🚀 **17.6x** | Exception formatting |
| **Tree** | 11.61s | **0.90s** | 🔥 **12.9x** | Hierarchical rendering |
| **Markdown** | 3.37s | **0.39s** | 🔥 **8.6x** | Parsing & Rendering |
| **Progress** | 0.034s | **0.005s** | 🔥 **7.1x** | Low overhead updates |
| **Panel** | 0.60s | **0.18s** | ⚡️ **3.4x** | Border rendering |
| **Text** | 2.03s | **1.06s** | ⚡️ **1.9x** | Rich text parsing |
| **Syntax** | 14.85s | **8.37s** | ⚡️ **1.8x** | Regex-based highlighting |

---

## 📅 Version History

### **v0.1.1** (Current)
- **Optimization**: Syntax highlighting speed improved by caching `SyntaxSet` and `ThemeSet` globally using `OnceLock`.
- **Result**: Reduced Syntax rendering time from ~12s (v0.1.0) to ~8.4s.

### **v0.1.0** (Baseline)
- Initial release with full feature parity.
- Established baseline for all components.

---

## 🔍 Methodology

Benchmarks are executed using `pyo3` bindings, meaning the Python interpreter overhead is included in the "Rust" times, making these results conservative estimates of the underlying Rust core performance.

- **Machine**: macOS / Apple Silicon
- **Python**: 3.10+
- **Iterations**:
    - Light components (Progress, Table): 10,000 ops
    - Heavy components (Syntax, Tree): 500-1,000 ops

### Workloads
- **Syntax**: 50 copies of a Rust function (approx 500 lines) with Monokai theme.
- **Tree**: Deeply nested directory structure simulation.
- **Table**: Large table with auto-sizing columns and unicode borders.
- **Markdown**: Parsing a document with headers, lists, and code blocks.
