# Fast-Rich Examples

This directory contains demonstration programs for all fast-rich features.

## Running Examples

### Run a single example:
```bash
cargo run --example hello
cargo run --example bar_demo
cargo run --example theme_demo
# etc.
```

### Run all examples at once:
```bash
./scripts/run_all_examples.sh
```

This will execute all 13 examples sequentially and provide a summary report.

---

## Available Examples

### Basic Examples
- **hello** - Simple "Hello, World!" example
- **markup** - Demonstrates markup tag parsing and rendering
- **console_print** - Shows console printing API

### Tier 1 Features
- **pretty_demo** - Pretty print with syntax highlighting
- **export_demo** - HTML/SVG export functionality

### Tier 2 Features  
- **align_demo** - Text alignment (left/center/right, top/middle/bottom)
- **padding_demo** - Padding wrapper with various configurations
- **bar_demo** - Bar chart visualization
- **theme_demo** - Color theme system (Default, Monokai, Night Owl)
- **highlighter_demo** - Pattern-based syntax highlighting

### Tier 3 Features
- **group_demo** - Render groups with spacing and dividers
- **measure_demo** - Measuring renderable dimensions
- **nested_progress_demo** - Hierarchical progress tracking

---

## Output Examples

Most examples produce formatted terminal output with colors, styles, and layouts. 

**Note:** Some examples (like `export_demo`) generate files:
- `export.html` - HTML output with ANSI rendering
- `export.svg` - SVG terminal output

---

## Tips

- Use `cargo run --quiet --example <name>` to suppress compilation output
- Examples are non-interactive (except prompt_demo which requires user input)
- Each example demonstrates a specific feature set
- Check example source code for implementation details

---

## Troubleshooting

If an example doesn't run:
1. Make sure you're in the project root: `cd /path/to/fast_rich`
2. Build the project first: `cargo build --examples`
3. Check for compilation errors: `cargo check`

For issues, see the main README.md or file an issue.
