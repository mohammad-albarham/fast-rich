# Feature: Color System Control

> **Slug:** `feature-color-system-control`
> **Type:** Fundamental
> **Status:** Done
> **Last Updated:** 2025-12-31

## 1. Overview
Implement strict control over the color system to match Python `rich`'s behavior. This ensures that `fast-rich` produces identical ANSI sequences for 16-color, 256-color, and TrueColor environments.

**Motivation:**
Currently, `fast-rich` often defaults to `crossterm`'s behavior, which may emit 256-color codes (`\x1b[38;5;...`) even when standard ANSI codes (`\x1b[31...`) would be more appropriate or required for strict byte-level parity with Python `rich` in standard mode.

## 2. Role in Hierarchy
- **Type:** Fundamental
- **Depends on:** None
- **Impacts:** `console.rs`, `style.rs`, and all rendering outputs.

## 3. Scope
- **Modules:**
  - `src/console.rs`: Add `color_system` field, update `write_span` to downsample colors.
  - `src/style.rs`: Add color conversion/downsampling logic.
- **Out of Scope:**
  - Changing the `crossterm` backend itself.

## 4. Behavior & Design Spec
- **Enum `ColorSystem`**:
  - `NoColor`: No ANSI color codes.
  - `Standard`: 8/16 colors (3-bit/4-bit ANSI).
  - `EightBit`: 256 colors (8-bit ANSI).
  - `TrueColor`: 16.7 million colors (24-bit ANSI).
  - `Windows`: Legacy Windows console support (mapped to Standard for ANSI output).
- **Console Configuration**:
  - `Console::new()` attempts to detect the system.
  - `Console::color_system(ColorSystem)` allows forcing a specific mode.
- **Downsampling Logic**:
  - **TrueColor -> EightBit**: Find nearest 256-color palette match.
  - **TrueColor/EightBit -> Standard**: Find nearest standard ANSI color.
  - **Standard**: Kept as is.

## 5. Tests
- **Unit Tests**:
  - Verify detection logic (mocking env vars).
  - Verify downsampling algorithms (RGB -> 256, RGB -> 16).
  - Verify `Console` output for specific `ColorSystem` settings.
- **Integration Tests**:
  - `ansi_byte_tests.rs`: Add test cases for forced `Standard` mode and verify it produces simple ANSI codes (no `38;5;`).

## 6. Change Log
- 2025-12-31: Feature file created.
