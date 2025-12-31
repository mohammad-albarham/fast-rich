# Fast-Rich

[![Crates.io](https://img.shields.io/crates/v/fast-rich.svg)](https://crates.io/crates/fast-rich)
[![Documentation](https://docs.rs/fast-rich/badge.svg)](https://docs.rs/fast-rich)
[![License](https://img.shields.io/crates/l/fast-rich.svg)](https://github.com/mohammad-albarham/fast-rich/blob/main/LICENSE)
[![Build Status](https://github.com/mohammad-albarham/fast-rich/workflows/CI/badge.svg)](https://github.com/mohammad-albarham/fast-rich/actions)

> [!IMPORTANT]
> This project is currently in **Alpha**. APIs are subject to change, and some features may be incomplete.

**Fast-Rich** is a Rust port of Python's popular [Rich](https://github.com/Textualize/rich) library. It empowers your terminal applications with rich text, tables, syntax highlighting, markdown rendering, and more, all with an idiomatic and safe Rust API.

## Features

- **Rich Text**: Use styled text with bold, italic, underline, and RGB/TrueColor support.
- **Tables**: Render beautiful tables with Unicode borders, column alignment, and auto-sizing.
- **Progress Bars**: Track tasks with multi-bar support, spinners, ETA, and customizable columns.
- **Live Display**: Create flicker-free, auto-updating displays for dashboards and dynamic content.
- **Syntax Highlighting**: Highlight code snippets using `syntect` with multiple themes (Monokai, Solarized, etc.).
- **Markdown**: Render Markdown content (headers, lists, code blocks, blockquotes) directly in the terminal.
- **Hierarchical Data**: Display tree structures and nested data.
- **Beautiful Logging**: structured, colored, and timestamped logging compatible with the `log` crate.
- **Layouts**: Split the screen into customizable layouts (vertical, horizontal, nested).
- **Panels & Rules**: Organize content with bordered panels and horizontal rules.

## Installation

Add `fast-rich` to your `Cargo.toml`:

```toml
[dependencies]
fast-rich = "0.2.0"
```

To enable all features (including Syntax Highlighting and Markdown, which bring in extra dependencies):

```toml
[dependencies]
fast-rich = { version = "0.2.0", features = ["full"] }
```

Or pick specific features:

```toml
[dependencies]
fast-rich = { version = "0.2.0", features = ["syntax", "markdown", "logging"] }
```

## Quick Start

```rust
use fast_rich::prelude::*;

fn main() {
    let console = Console::new();

    // Styled text using markup
    console.print("[bold red]Hello[/] [blue]World[/]!");

    // Create a table
    let mut table = Table::new();
    table.add_column("Features");
    table.add_column("Status");
    table.add_row_strs(&["Rich Text", "✅ Ready"]);
    table.add_row_strs(&["Tables", "✅ Ready"]);
    
    // Render the table
    console.print_renderable(&table);
}
```

## Documentation

- [Rust API Docs](docs/api.md)
- [Getting Started](docs/rust_getting_started.md)
- [Rust Examples](docs/rust_examples.md)

## Examples

Run the included examples to see features in action:

```bash
cargo run --example showcase
cargo run --example syntax_highlighting --features syntax
cargo run --example live_clock
cargo run --example progress_rich
cargo run --example logging --features logging
```

## Comparisons

| Feature | Python Rich | Fast-Rich |
|:--------|:------------|:----------|
| Rich Text | ✅ | ✅ |
| Tables | ✅ | ✅ |
| Progress | ✅ | ✅ |
| Live Display | ✅ | ✅ |
| Syntax Highlighting | ✅ | ✅ |
| Markdown | ✅ | ✅ |
| Layouts | ✅ | ✅ |
| Tracebacks | ✅ | ✅ |
| Inspect | ✅ | ✅ |

## Contributing

Contributions are welcome! Please check out [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on how to get started.

## License

MIT or Apache-2.0
