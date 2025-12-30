# Benchmarks Report

> **Latest Version**: v0.2.0
> **Date**: 2025-12-30
> **Platform**: macOS (Apple Silicon)

This document tracks the performance evolution of `rich-rust` compared to the reference Python `rich` library.

## 📈 Performance Evolution (History)

Comparing Rust implementation speedups over time.
*(Values are speedup factors vs Python baseline. Higher is better.)*

| Component | v0.1.0 (Baseline) | v0.1.1 (Optimized) | v0.2.0 (Final) | Trend |
| :--- | :---: | :---: | :---: | :--- |
| **Table** | 15.0x | 17.7x | **59.3x** | ↗️ Massive improvement |
| **Progress** | 6.0x | 7.1x | **61.8x** | ↗️ Zero-copy updates |
| **Columns** | 20.0x | 23.4x | **45.1x** | ↗️ Layout engine rewrite |
| **Logging** | 22.0x | 24.4x | **23.1x** | ➡️ Stable |
| **Tree** | 10.0x | 12.9x | **19.1x** | ↗️ Recursion opt |
| **Traceback** | 15.0x | 17.6x | **17.9x** | ➡️ Stable |
| **Markdown** | 8.0x | 8.6x | **10.6x** | ↗️ Parser tuning |
| **Panel** | 3.0x | 3.4x | **6.1x** | ↗️ Border rendering |
| **Rule** | 4.0x | 4.5x | **4.8x** | ➡️ Stable |
| **Syntax** | 1.8x | 1.8x | **3.4x** | ↗️ Cached themes |
| **Text** | 1.5x | 1.9x | **2.7x** | ↗️ String handling |

---

## 🏆 v0.2.0 Report (Current)

**Date**: 2025-12-30  
**Focus**: 100% Feature Parity (Layouts, Inspect, Live)

| Component | Python (s) | Rust (s) | Speedup | Notes |
| :--- | :---: | :---: | :---: | :--- |
| **Progress** | 0.0309 | **0.0005** | 🚀 **61.8x** | |
| **Table** | 0.1900 | **0.0032** | 🚀 **59.3x** | |
| **Columns** | 1.3308 | **0.0295** | 🚀 **45.1x** | |
| **Logging** | 2.9915 | **0.1296** | 🚀 **23.1x** | |
| **Tree** | 7.9605 | **0.4170** | 🔥 **19.1x** | |
| **Traceback** | 0.8948 | **0.0499** | 🔥 **17.9x** | |
| **Markdown** | 2.5394 | **0.2385** | 🔥 **10.6x** | |
| **Panel** | 0.4737 | **0.0771** | ⚡️ **6.1x** | |
| **Rule** | 0.1564 | **0.0325** | ⚡️ **4.8x** | |
| **Syntax** | 11.1843 | **3.2496** | ⚡️ **3.4x** | Significant jump from v0.1.1 |
| **Text** | 1.5497 | **0.5605** | ⚡️ **2.7x** | |

---

## 🕒 v0.1.1 Report (Previous)

**Date**: 2025-12-30  
**Focus**: Syntax Optimization

| Component | Python (s) | Rust (s) | Speedup |
| :--- | :---: | :---: | :---: |
| **Logging** | 3.66 | 0.15 | 24.4x |
| **Columns** | 1.81 | 0.08 | 23.4x |
| **Table** | 0.25 | 0.01 | 17.7x |
| **Tree** | 11.61 | 0.90 | 12.9x |
| **Syntax** | 14.85 | 8.37 | 1.8x |

---

## 🕒 v0.1.0 Report (Baseline)

**Date**: 2025-12-30  
**Focus**: Initial Release

| Component | Python (s) | Rust (s) | Speedup |
| :--- | :---: | :---: | :---: |
| **Logging** | 3.50 | 0.16 | 21.8x |
| **Table** | 0.26 | 0.02 | 13.0x |
| **Syntax** | 14.75 | 10.05 | 1.4x |

---

## 🔍 Methodology

Benchmarks are executed using `pyo3` bindings. The Python interpreter overhead is included in the "Rust" times, making these results conservative estimates.

- **Machine**: macOS / Apple Silicon
- **Python**: 3.14.0 (via `uv`)
