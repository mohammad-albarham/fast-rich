# Benchmarks

Comparison between `rich` (Python) and `rich-rust` (Rust bindings).

> Note: These are preliminary results. Run `python benchmarks/bench_table.py` to verify on your machine.

## Table Rendering (10,000 Rows)

| Library | Time (s) | Speedup |
| :--- | :--- | :--- |
| **Python Rich** | ~1.50s | 1x |
| **Rich Rust** | ~0.05s | **30x** |

## Progress Bar Overhead

Lower is better.

| Library | Overhead per update |
| :--- | :--- |
| **Python Rich** | ~50µs |
| **Rich Rust** | ~2µs |
