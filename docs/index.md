# rich-rust

**rich-rust** is a high-performance Rust port of the famous [Rich](https://github.com/Textualize/rich) Python library.

It provides beautiful terminal formatting for Rust applications, and also offers Python bindings that aim to be significantly faster than the pure Python implementation for heavy rendering tasks.

## Features

*   **Console**: Standardized output with styling and capabilities detection.
*   **Text**: Rich text with styles, colors (ANSI, 256, RGB), and emoji.
*   **Tables**: Flexible tables with multiple border styles.
*   **Progress**: Multi-task progress bars with ETA and speed.
*   **Tree**: Hierarchical data visualization.
*   **Markdown**: Render Markdown directly to the terminal.
*   **Syntax Highlighting**: Highlight code snippets.
*   **Traceback**: Pretty print panic tracebacks.

## Quick Start (Rust)

Add to `Cargo.toml`:

```toml
[dependencies]
rich-rust = "0.1.0"
```

Use in `main.rs`:

```rust
use rich_rust::prelude::*;

fn main() {
    let console = Console::new();
    console.print("[bold red]Hello[/] [blue]World[/]!");
}
```

## Quick Start (Python)

See [Python API](python_api.md) for details.

```python
import rich_rust

console = rich_rust.Console()
console.print("[bold red]Hello[/] [blue]World[/]!")
# Speed up your table rendering:
table = rich_rust.Table()
# ... add 100k rows
console.print_table(table)
```
