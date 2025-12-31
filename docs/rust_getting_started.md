# Getting Started with Fast-Rich

**fast-rich** is a library for writing beautiful terminal software in Rust. It allows you to add color and style to text, create complex layouts, display tables, render markdown, and more.

## Installation

Add `fast-rich` to your `Cargo.toml`:

```toml
[dependencies]
fast-rich = "0.2.0"
```

To enable all features (Markdown, Syntax Highlighting, etc.):

```toml
[dependencies]
fast-rich = { version = "0.2.0", features = ["full"] }
```

## Basic Usage

The entry point for most operations is the `Console` struct.

```rust
use fast_rich::prelude::*;

fn main() {
    let console = Console::new();
    
    // Simple printing with styles
    console.print("[bold red]Hello[/] [blue]World[/]!");
    
    // Using a rule
    console.rule("[bold]Chapter 1[/]");
}
```

## Next Steps

- Explore the [Examples](rust_examples.md) to see what's possible.
- Check the [API Reference](api.md) for detailed documentation.
