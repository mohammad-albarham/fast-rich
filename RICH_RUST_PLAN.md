# rich-rust Plan / Feature Memory

> This file tracks the implementation status of all features. Updated continuously.

## Legend
- `[ ]` Not started
- `[/]` In progress  
- `[x]` Implemented
- `[T]` Tested
- `[D]` Documented

---

## Phase 1: Core Foundation

### Console & Print
- [/] Console abstraction (stdout/stderr, width detection)
- [ ] Console.print() with word wrapping
- [ ] Multiple argument handling
- [ ] Style integration in print

### Styles & Colors
- [ ] Color enum (named colors)
- [ ] Color RGB support
- [ ] Color 256-color support
- [ ] Style struct (fg, bg, bold, italic, underline, dim, strikethrough, reverse)
- [ ] Style combination/merging

### Text & Spans
- [ ] Text type with styled spans
- [ ] Span type for styled regions
- [ ] Word wrapping (soft/hard)
- [ ] Alignment (left, center, right)
- [ ] Truncation with overflow indicator

### Markup
- [ ] Markup parser `[tag]...[/]`
- [ ] Nested tags support
- [ ] Style tag parsing (bold, italic, colors)
- [ ] Self-closing tags

### Emoji
- [ ] Emoji name mapping `:name:` → Unicode
- [ ] Common emoji set (100+)

### Renderable
- [ ] Renderable trait definition
- [ ] Console protocol implementation

---

## Phase 2: Basic Renderables

### Table
- [ ] Basic table with rows
- [ ] Headers with style
- [ ] Per-column width specification
- [ ] Per-column alignment
- [ ] Unicode box borders (multiple styles)
- [ ] Auto-width calculation
- [ ] Content wrapping in cells
- [ ] Nested renderables in cells

### Panel
- [ ] Box with content
- [ ] Title and subtitle
- [ ] Padding control
- [ ] Border styles

### Rule
- [ ] Horizontal line
- [ ] Centered label
- [ ] Custom characters

### Columns
- [ ] Multi-column layout
- [ ] Equal width mode
- [ ] Optimal width mode

---

## Phase 3: Progress System

### Progress Bars
- [ ] Single progress bar
- [ ] Multi-task progress display
- [ ] Percentage column
- [ ] Speed column
- [ ] ETA column
- [ ] Custom columns
- [ ] Flicker-free updates (in-place)

### Spinners
- [ ] Spinner animation
- [ ] Multiple spinner styles (10+)

### Status
- [ ] Status context (spinner + message)
- [ ] RAII guard pattern

### Helpers
- [ ] track() iterator wrapper

---

## Phase 4: Logging & Inspect

### Logging
- [ ] console.log() method
- [ ] Timestamp column
- [ ] File/line capture via macro
- [ ] Pretty-print collections
- [ ] log_locals equivalent

### Log Integration
- [ ] log crate Handler (feature-gated)

### Inspect
- [ ] Inspect arbitrary Debug types
- [ ] Methods listing (limited in Rust)
- [ ] Attributes display

---

## Phase 5: Tree & Layout

### Tree
- [ ] Tree renderable
- [ ] Guide lines (multiple styles)
- [ ] Arbitrary renderable labels
- [ ] Add child nodes API

### Layout
- [ ] Nested renderable measurement
- [ ] Width calculation refinements
- [ ] Padding/margin utilities

---

## Phase 6: Markdown & Syntax

### Markdown (feature: `markdown`)
- [ ] Markdown parsing
- [ ] Headings rendering
- [ ] Emphasis (bold, italic)
- [ ] Lists (ordered, unordered)
- [ ] Code blocks
- [ ] Blockquotes
- [ ] Links (show URL)

### Syntax (feature: `syntax`)
- [ ] Syntax highlighting
- [ ] Line numbers
- [ ] Theme support
- [ ] Language detection

---

## Phase 7: Tracebacks

### Panic Rendering
- [ ] Custom panic hook
- [ ] Backtrace capture
- [ ] Source context display
- [ ] Styled error panels
- [ ] install_panic_hook() helper

---

## Phase 8: Polish

### Documentation
- [ ] rustdoc for all public items
- [ ] README with examples
- [ ] CHANGELOG

### Quality
- [ ] All tests passing
- [ ] clippy clean
- [ ] fmt clean
- [ ] Examples for all features

---

## Notes & Design Decisions

### Rust Adaptations
- Python REPL integration → Skipped (no Rust equivalent)
- `__rich__` protocol → `Renderable` trait
- Context managers → RAII guards
- Decorators → Procedural macros

### Dependencies Chosen
- Terminal: `crossterm` (cross-platform)
- Unicode: `unicode-width`, `unicode-segmentation`
- Terminal size: `terminal_size`
- Markdown: `pulldown-cmark` (feature-gated)
- Syntax: `syntect` (feature-gated)

### TODO / Known Limitations
- (To be filled as implementation progresses)
