# Benchmarks Report

> **Version**: v0.2.0
> **Date**: 2025-12-30
> **Platform**: macOS (Apple Silicon)

This document details the performance of `rich-rust` compared to the reference Python `rich` library. All benchmarks measure the wall-clock time to render a fixed workload (N iterations).

## 🏆 Executive Summary

`rich-rust` delivers **extreme performance improvements** across the board. The core rendering engine, now fully implemented in v0.2.0, provides speedups ranging from **3x to 60x** compared to the Python implementation.

| Component | Python `rich` (s) | `rich-rust` (s) | Speedup | Notes |
| :--- | :---: | :---: | :---: | :--- |
| **Progress** | 0.0309s | **0.0005s** | 🚀 **61.8x** | Low overhead updates |
| **Table** | 0.1900s | **0.0032s** | 🚀 **59.3x** | Auto-sizing logic |
| **Columns** | 1.3308s | **0.0295s** | 🚀 **45.1x** | Layout calculation |
| **Logging** | 2.9915s | **0.1296s** | 🚀 **23.1x** | High-throughput logging |
| **Tree** | 7.9605s | **0.4170s** | 🔥 **19.1x** | Hierarchical rendering |
| **Traceback** | 0.8948s | **0.0499s** | 🔥 **17.9x** | Exception formatting |
| **Markdown** | 2.5394s | **0.2385s** | 🔥 **10.6x** | Parsing & Rendering |
| **Panel** | 0.4737s | **0.0771s** | ⚡️ **6.1x** | Border rendering |
| **Rule** | 0.1564s | **0.0325s** | ⚡️ **4.8x** | Horizontal rules |
| **Syntax** | 11.1843s | **3.2496s** | ⚡️ **3.4x** | Regex-based highlighting |
| **Text** | 1.5497s | **0.5605s** | ⚡️ **2.7x** | Rich text parsing |

---

## 📅 Version History

### **v0.2.0** (Current)
- **Features**: Full feature parity achieved (Layout, Live, Prompt, Inspect).
- **Performance**: Validated massive speedups across all components.
- **Optimizations**: Global syntax theme caching, zero-copy text rendering where possible.

### **v0.1.1**
- **Optimization**: Syntax highlighting speed improved by caching `SyntaxSet` and `ThemeSet` globally.
- **Result**: Reduced Syntax rendering time significantly.

### **v0.1.0**
- Initial release with core features.

---

## 🔍 Methodology

Benchmarks are executed using `pyo3` bindings, meaning the Python interpreter overhead is included in the "Rust" times. This makes these results conservative estimates of the underlying Rust core performance.

- **Machine**: macOS / Apple Silicon
- **Python**: 3.14.0 (via `uv`)
- **Iterations**:
    - Light components (Progress, Table): 10,000 ops
    - Heavy components (Syntax, Tree): 500-1,000 ops
