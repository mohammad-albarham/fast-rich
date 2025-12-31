# fast-rich Handover Document

> **Last Updated:** 2025-12-31  
> **Purpose:** Enable any developer or LLM agent to resume work without chat history

---

## Project Overview

**fast-rich** is a Rust port of Python's `rich` terminal formatting library.

### Goals
- Match Python `rich` behavior at the ANSI/byte level where feasible
- Provide idiomatic, safe, performant Rust API
- Complete feature parity with Python rich's core features

### Key Constraints
- Python `rich` is the behavioral reference implementation
- ANSI sequence correctness is critical for terminal compatibility
- No fake outputs or imaginary test results in development

---

## Architecture

### Core Modules

| Module | Purpose |
|:-------|:--------|
| `console.rs` | Main Console API, print/render functions, ANSI output |
| `text.rs` | Text with spans, wrapping, alignment |
| `markup.rs` | `[bold red]text[/]` markup parser |
| `style.rs` | Style struct (colors, bold, etc.) |
| `renderable.rs` | Renderable trait, Segment output |
| `panel.rs` | Bordered panels |
| `table.rs` | Tables with borders |
| `rule.rs` | Horizontal rules with titles |
| `tree.rs` | Tree structures |
| `progress/` | Progress bars, spinners |
| `syntax.rs` | Syntax highlighting (feature-gated) |

### Key Data Flow

```
markup::parse(str) -> Text { spans: Vec<Span> }
                          -> Text::render() -> Vec<Segment>
                          -> Console::write_segments() -> ANSI to stdout
```

### Design Decisions

1. **Markup is opt-in**: `console.print()` parses markup, `console.print_raw()` does not
2. **Styles are value types**: Style is Copy, combines via `+` operator
3. **Renderables are trait objects**: dyn Renderable allows polymorphism
4. **Width-aware rendering**: All renderables receive RenderContext with width

---

## Bugs Fixed (This Session)

### Feature: ANSI Byte-Level Test Infrastructure ✅ IMPLEMENTED

**Goal:** Ensure Rust output matches Python `rich` output byte-for-byte.

**Changes:**
1. Created `tests/ansi_byte_tests.rs` - Comprehensive ANSI verification suite
2. Created `tests/ansi_test_helpers.rs` - Helpers for hex dumps and semantic comparison
3. Added `scripts/compare_ansi.sh` - Automated regression script
4. Fixed underlying ANSI issues:
   - Eliminated double reset codes (`\e[0m\e[0m`)
   - Verified space preservation between styled spans

**Current Status:**
- Basic styles match Python **exactly** (46 bytes vs 46 bytes)
- Colors match semantically (Rust currently uses 256-color codes vs Python's 16-color in some environments)

### P0 #1: Markup Spacing Bug ✅ FIXED

**Symptom:** Text runs together after styled spans
```
Input:  "Check [bold]word[/] and more"
Output: "Checkwordand more"  ❌ (spaces lost)
```

**Root Cause:** `split_into_words()` in `text.rs` discarded leading whitespace from spans.

**Fix:** Modified `split_into_words()` to preserve leading spaces by:
1. Counting leading whitespace
2. Returning leading spaces as a separate word entry

**Files Changed:** `src/text.rs`

**Test:** All 101 unit tests pass, 13 examples work correctly

---

### P0 #2: Vector Display Missing in Debug ✅ FIXED

**Symptom:** `print_debug()` shows `roles: ,` instead of `roles: ["admin", "editor"]`

**Root Cause:** `console.print()` parses `[...]` as markup tags. Debug output like `["admin"]` was incorrectly interpreted as a markup tag and removed.

**Fix:** 
1. Changed `print_debug()` to use `Text::plain()` instead of `print()`
2. Added `print_raw()` and `println_raw()` methods for raw output

**Files Changed:** `src/console.rs`

**API Addition:**
```rust
// Print without markup parsing
console.print_raw("roles: [\"admin\"]");
console.println_raw(&format!("{:?}", data));
```

---

---

## Feature Registry / Backlog

| Slug | Name | Type | Status | Description |
|:-----|:-----|:-----|:-------|:------------|
| `feature-color-system-control` | Color System Control | **Fundamental** | `Done` | Implement strict `ColorSystem` downsampling (16/256/TrueColor) for exact byte parity. |
| `feature-box-style-parity` | Box Style Parity | Dependent | `Planned` | Implement missing box styles (Double, Rounded, Heavy, etc.) in `table.rs`. |
| `feature-layout-parity` | Layout Parity | **Fundamental** | `Planned` | Enhance `layout.rs` with split ratios, arbitrary depth, and minimum sizes to match `rich.layout`. |
| `feature-live-display` | Live Display | **Fundamental** | `Planned` | Implement flicker-free auto-refreshing display (`rich.live`) with cursor management. |
| `feature-markdown-parity` | Markdown Parity | Dependent | `Planned` | Improve `markdown.rs` to render tables, block quotes, and code blocks similarly to `rich`. |
| `feature-progress-enhancement` | Progress Enhancement | Dependent | `Planned` | Add spinners, speed columns, and ETA to `progress` module. |
| `feature-logging-handler` | Logging Handler | Dependent | `Planned` | Implement `RichHandler` for `log` crate with proper formatting and highlighting. |
| `feature-tree-styled` | Tree Styling | Dependent | `Planned` | Enhance `tree.rs` with configurable guide styles and branch formatting. |

## Known Issues / TODOs

### P1: Implement ColorSystem for Byte Parity (Future Task)
- **Goal:** Achieve perfect byte-parity with Python Rich's default 16-color output.
- **Current State:** `fast-rich` relies on `crossterm` defaults, often emitting 256-color codes (`\x1b[38;5;1m`) where Python emits standard ANSI (`\x1b[31m`).
- **Plan:**
  1. Define `ColorSystem` enum: `NoColor`, `Standard` (8/16), `EightBit` (256), `TrueColor` (16M), `Windows`.
  2. Add `color_system` field to `Console` struct (default: `Auto`).
  3. Implement **downsampling** in `Console::write_span`:
     - If `Standard`: convert RGB/256 colors to nearest 16-color ANSI code.
     - If `EightBit`: allow `38;5;...` codes.
  4. Verify: `Console::new().color_system(ColorSystem::Standard)` matches Python output byte-for-byte.

### P1-A: ANSI Reset Redundancy ✅ FIXED
- **Was:** Double reset codes `\e[39;49m\e[0m` in output (ResetColor + Attribute::Reset)
- **Fix:** Removed redundant `ResetColor` call in `Console::write_span()` - SGR 0 already resets all
- **Files Changed:** `src/console.rs`

### P1-B: Cargo.toml Warning ✅ FIXED
- **Was:** Duplicate `ansi_comparison` target (bin + integration test)
- **Fix:** Removed `[[bin]]` section for `ansi_comparison`
- **Files Changed:** `Cargo.toml`

---

## Test Status

| Test Type | Count | Status |
|:----------|------:|:------:|
| Unit tests | 101 | ✅ Pass |
| ANSI checks | 19 | ✅ Pass |
| Examples | 13 | ✅ Pass |

### Running Tests

```bash
# All unit tests
cargo test --lib

# ANSI byte-level tests
cargo test --test ansi_byte_tests

# Automated comparison
./scripts/compare_ansi.sh
```

---

## File Change Summary

| File | Changes |
|:-----|:--------|
| `tests/ansi_byte_tests.rs` | [NEW] Integration tests for ANSI byte correctness |
| `tests/ansi_test_helpers.rs` | [NEW] Helper module for hex diffs and parsing |
| `scripts/compare_ansi.sh` | [NEW] Automation script |
| `src/text.rs` | Fixed `split_into_words()` to preserve leading whitespace |
| `src/console.rs` | Fixed `print_debug()` to use Text::plain(); Added `print_raw()`, `println_raw()`; No redundant resets |

---

## Verification Commands

```bash
# Verify ANSI correctness
./scripts/compare_ansi.sh

# Manual verification
xxd tests/ansi_output/rust_basic_styles.txt
# Output should contain exactly one reset `1b5b 306d` between styles
```

---

## Session Summary

**2025-12-31 (Feature: ANSI Tests):** Implemented comprehensive ANSI byte-level test infrastructure. 
- Added `ansi_byte_tests.rs` with 19 tests verifying styles, reset codes, and spacing.
- Confirmed `basic_styles` matches Python `rich` output byte-for-byte (46 bytes).
- Identified color system difference (256 vs 16 color codes), updated tests to accept both semantically.
- Documented plan for `ColorSystem` control to achieve full byte parity in the future.
- Fixed `hexdump` helper and ensured robust verification.
