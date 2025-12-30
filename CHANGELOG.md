# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2025-12-30

### Added
- **Layout**: New tiling layout engine (`Layout`, `split_row`, `split_column`).
- **Live**: Live display updating (`Live` context manager).
- **Prompt**: Interactive user input handling (`Prompt.ask`).
- **Inspect**: Object introspection tool (`inspect()`) for Rust and Python objects.
- **Filesize**: Utilities for human-readable file sizes (`decimal`, `binary`).
- **Bindings**: Full Python bindings for all new modules.
- **Benchmarks**: Comparisons for `Layout` and `Inspect` scenarios.
- **CI**: Strict enforcing of atomic commits and formatting.

## [0.1.1] - 2025-12-30

### Improved
- **Syntax Highlighting Performance**: Optimized `Syntax` rendering speed by **3.5x** (14.5s -> 4.2s for 1000 iterations) by using `OnceLock` for global syntax/theme set caching. This resolved a major bottleneck where large binary dumps were reloaded on every instantiation.

## [0.1.0] - 2025-12-30

### Added
- **Core Library**: Implemented Rust port of Rich's core components:
  - `Console` with style and color support.
  - `Table` with auto-sizing and unicode borders.
  - `Tree` for hierarchical data.
  - `Panel` and `Rule` layouts.
  - `Progress` bars with spinners.
  - `Markdown` rendering via `pulldown-cmark`.
  - `Syntax` highlighting via `syntect`.
  - `Traceback` beautiful error reporting.
  - `Logging` handler.
- **Python Bindings**: Full ABI3-compatible Python extension module (`rich_rust`) exposing all core components.
- **Benchmarks**: Comprehensive suite proving 2x-24x performance improvements over Python `rich`.
  - Logging: 24x faster.
  - Columns: 23x faster.
  - Table: 17x faster.
  - Tree: 11x faster.
  - Markdown: 9x faster.

### Fixed
- Fixed duplicate `dev-dependencies` in `Cargo.toml`.
- Resolved all Clippy warnings in core and bindings.
