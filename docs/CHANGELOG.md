# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed
- **Build**: The `markdown` feature no longer fails to compile without `syntax`;
  `markdown` now implies `syntax` (matching Rich, where markdown renders code blocks).
- **Build**: Removed unused `terminal_size` dependency (crossterm is used for
  terminal size) and unused `insta`/`tempfile` dev-dependencies.
- **MSRV**: Added `rust-version = "1.81"` to `Cargo.toml`, an MSRV job to CI, and
  pinned lockfile dependencies to versions compatible with Rust 1.81. The README
  badge now claims the verified MSRV (1.81+, was 1.70+).
- **CI**: Added a feature-matrix job (`--no-default-features`, each feature alone,
  and combinations) so single-feature breakage can never ship again, plus a
  security-audit job and a `cargo publish --dry-run` job. Fixed token-less codecov
  upload and the broken empty-token README badge.
- **Docs**: Deduplicated the "Macro Summary" table in crate documentation.
- **Text**: Embedded `\n` in printed text is now a hard line break instead of
  collapsing to a space; wrapping preserves runs of inner spaces and lines no
  longer exceed the available width by a trailing space.
- **Text**: `print!`/`println!` now match `std` semantics — `print` adds no
  trailing newline, `println` exactly one (previously `println` double-spaced).
- **Markup**: Unknown `:word:` shortcodes keep their colons (`"12:30:45"` no
  longer renders as `"1230:45"`); `:+1:` resolves; `]]` no longer breaks tag
  parsing; the emoji-syntax doc was corrected.
- **Style**: `Color::parse` no longer panics on non-ASCII hex (e.g. `#日本`);
  `not bold` negates the attribute instead of enabling it.
- **Layout**: Ratio distribution clamps to the remaining space, fixing a `u16`
  underflow panic with many flexible children. `Layout` sizes are now `usize`.
- **Console**: Color detection honors TTY state (piped output is plain text),
  `NO_COLOR`, `TERM=dumb`, `CLICOLOR_FORCE`, and `FORCE_COLOR` (1/2/3 levels).
- **Prompt**: EOF on stdin returns the default or an error instead of spinning
  forever (`Prompt::try_ask` / `PromptError`); `secret()` reads hidden input via
  raw mode; `case_sensitive` builder wired into choice matching.
- **Pager**: raw mode is restored on panics via an RAII `RawModeGuard`; the
  cursor is always re-shown; key release events no longer double-scroll;
  resize events update the page height.
- **Traceback**: `RUST_BACKTRACE=1` now delegates to the previous panic hook so
  backtraces are preserved.
- **Progress**: `auto_refresh`/`refresh_per_second` spawn a real background
  refresh thread; `BarColumn::expand(true)` receives its share of the spare
  width; Live/Progress height tracking counts soft-wrapped terminal lines.
- **Table**: cells keep their styles and support multi-line content with
  aligned borders; `ColumnWidth::Fixed/Min/Max` are honored; `Column::wrap`
  wraps at the column width; `Row::style` is applied; truncated cells are
  exactly the column width (ellipsis off-by-one).
- **Markdown**: nested ordered lists keep their numbering after an inner list
  closes.
- **Tree**: `from_directory` no longer follows symlink cycles, hides dotfiles
  by default, and supports `DirectoryTreeOptions` (max depth, hidden files,
  symlink following).
- **Width**: rules, SVG export, bar-chart labels/values and panel padding now
  measure display width (CJK/emoji) instead of bytes.
- **Panel**: subtitle styling is separate (`subtitle_style`).

### Changed
- **Features**: `regex`, `serde`, and `serde_json` are now optional, gated behind a
  new `json` feature (the `Json` and `highlighter` modules require it). `full`
  includes `json`. The incoherent `std` feature was removed (`Tree::from_directory`
  is now always available).
- **API**: `Json` and the highlighter module require the `json` feature.
- **API**: `Prompt::ask` panics only on EOF without a default; prefer
  `Prompt::try_ask` which returns `Result<T, PromptError>`.
- **API**: Deprecated alignment enums `table::ColumnAlign` and `rule::RuleAlign`
  in favor of `text::Alignment`; `box_drawing::Box` renamed to `BoxStyle` (old
  name kept as a deprecated alias); `Table::set_title` deprecated in favor of
  `Table::title`; `Measurement::measure` deprecated in favor of the free
  function `measure::measure`.
- **API**: Removed placeholder options that never worked:
  `InspectConfig::methods`, `TracebackConfig::show_locals`, `Live::refresh_rate`,
  and the dead `CapturedOutput` type. `NestedProgress` now implements
  `Renderable` (indented bar tree).
- **API**: `Panel::new` now fits the content by default (`expand(false)`,
  matching Rich); `Console` is `Clone` (cheap, Arc-backed) and gained
  `theme()`/`get_theme()` — theme `custom` colors are usable as markup tags
  (e.g. `[brand]`). Prelude/root re-exports now cover `Align`, `Padding`,
  `Theme`, `Span`, `Segment`, `Layout`, `Live`, `Direction`, `BarChart`,
  `RenderGroup` and `Measurement`.
- **Performance**: wrapping merges adjacent same-style spans (one span per
  style run, not per word); console output is queued and flushed once per
  print; JSON highlighter regexes compile once per process; JSON rendering no
  longer deep-clones the value tree unless sorting; Markdown and Syntax cache
  parsed/highlighted output across frames; 256-color mapping is closed-form
  (proven identical to the brute-force scan by test).

- **Markup (breaking)**: tag grammar now matches Rich. A tag is `[` followed by
  `a-z`, `#`, `/` or `@`, so `[1, 2]`, `[INFO]` and `list[]` print literally;
  unknown lowercase tags are consumed like in Rich. The escape is now `\[`
  (Rich's) instead of `[[`/`]]`, which no longer have special meaning.
- **Theme**: markup tags resolve theme style names (`[warning]`), including
  the semantic colors (`primary`, `success`, ...).

### Added
- **Markup**: `markup::escape()`, the counterpart of Rich's `rich.markup.escape`.
- **Theme**: named styles — `Theme::styles`, `Theme::add_style`,
  `Theme::lookup_style` (like Rich's `Theme({"warning": "bold red"})`).
- **Testing**: golden-file snapshot tests (`tests/snapshot_golden.rs`,
  regenerate with `FAST_RICH_UPDATE_SNAPSHOTS=1`); unit tests for all
  previously uncovered modules (`live`, `inspect`, `progress/columns`, `align`,
  `theme`, `padding`, `box_drawing`, `progress/mod`, `progress/spinner_data`).

## [0.3.3] - 2026-08-09

### Changed
- **Repository Cleanup**: Removed duplicate GitHub Actions CI workflow to avoid duplicated checks.

### Removed
- **Python Bindings**: Removed legacy Python binding surface from the crate codebase while keeping Python-only benchmark and verification tooling.

### Fixed
- **CI Stability**: Resolved duplicate `Rustfmt` status checks on pushes.

## [0.3.2] - 2026-01-11

### Added
- **Print Macros**: Drop-in `print!`/`println!` replacements that shadow std macros with rich markup support.
  - `print!`, `println!`, `eprint!`, `eprintln!` - styled output macros
  - `print_raw!`, `println_raw!`, `eprint_raw!`, `eprintln_raw!` - raw output (no parsing)
  - `rprint!`, `rprintln!` - non-shadowing aliases
- **Spinner Styles**: 80+ spinner styles for `SpinnerColumn`:
  - `SpinnerColumn::new().with_style(SpinnerStyle::Moon)`
  - Emoji spinners: Moon, Earth, Clock, Hearts, Star
  - ASCII spinners: Arrow, BouncingBar, GrowHorizontal, Aesthetic
  - `SpinnerStyle::all_names()` to list available styles

### Enhanced
- **Progress Bars**: Significantly improved `Progress` module (Python-rich parity):
  - Added `BarColumn` with distinct characters for filled, pointer, and empty states.
  - Implemented pulsing animation for indeterminate tasks (unknown total).
  - Added `refresh()`, `start()`, and `stop()` methods for flicker-free rendering.
  - Added `run()` method for context-manager style usage.
  - Added sub-character precision (smooth updates) using 8th-block unicode characters.
  - Added `expand` option for full-width progress bars.

### Fixed
- **Documentation**: Fixed `track()` demo glitch where output would wrap and stack incorrectly.
- **Example**: `docs_track` example works reliably on all terminal widths.

## [0.3.1] - 2026-01-01

### Fixed
- **CI**: Resolved `Rustfmt` failures and Windows-specific line ending mismatches in byte-level tests.

### Added
- **README**: Polished Documentation section with high-fidelity badges and corrected links.

## [0.3.0] - 2025-12-31

### Added
- **Professional Visuals**: Integrated high-fidelity terminal animations via VHS.
- **Visual README**: Overhauled landing page with centered branding and feature gallery.
- **Interactive Tapes**: Added `.tape` files for automating all project demonstrations.
- **Security Audit**: Completed full codebase scan for secrets and sensitive data.
- **Project Logo**: Official Rocket Crab branding integrated across all docs.

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
- **Python Bindings**: Full ABI3-compatible Python extension module (`fast_rich`) exposing all core components.
- **Benchmarks**: Comprehensive suite proving 2x-24x performance improvements over Python `rich`.
  - Logging: 24x faster.
  - Columns: 23x faster.
  - Table: 17x faster.
  - Tree: 11x faster.
  - Markdown: 9x faster.

### Fixed
- Fixed duplicate `dev-dependencies` in `Cargo.toml`.
- Resolved all Clippy warnings in core and bindings.
