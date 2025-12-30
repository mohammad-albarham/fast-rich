# rich-rust

A Rust port of Python's [Rich](https://github.com/Textualize/rich) library for beautiful terminal formatting.

[![Crates.io](https://img.shields.io/crates/v/rich-rust.svg)](https://crates.io/crates/rich-rust)
[![Documentation](https://docs.rs/rich-rust/badge.svg)](https://docs.rs/rich-rust)
[![License](https://img.shields.io/crates/l/rich-rust.svg)](LICENSE-MIT)

## Features

- 🎨 **Rich text** with colors, styles, and markup
- 📊 **Tables** with Unicode borders and auto-sizing
- 📚 **Columns** layout for arranging content
- 📈 **Progress bars** with multiple tasks and spinners
- 🌳 **Tree views** for hierarchical data
- 📝 **Markdown** rendering (optional)
- 🔍 **Syntax highlighting** (optional)
- 🐛 **Traceback** beautiful error reporting
- 🪵 **Logging** integration

## Quick Start

```rust
use rich_rust::prelude::*;

fn main() {
    let console = Console::new();
    
    // Simple styled output
    console.print("Hello, [bold magenta]World[/]!");
    
    // Tables
    let mut table = Table::new();
    table.add_column("Name");
    table.add_column("Age");
    table.add_row(&["Alice", "30"]);
    table.add_row(&["Bob", "25"]);
    console.print(&table);
}
```

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
rich-rust = "0.1"
```

With optional features:

```toml
[dependencies]
rich-rust = { version = "0.1", features = ["markdown", "syntax"] }
```

## Features

| Feature | Description |
|---------|-------------|
| `markdown` | Markdown rendering support |
| `syntax` | Syntax highlighting support |
| `logging` | Integration with the `log` crate |
| `full` | All optional features |

## Examples

See the [`examples/`](examples/) directory for more usage examples.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
