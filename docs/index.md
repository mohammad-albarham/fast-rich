# fast-rich

**fast-rich** is a high-performance Rust port of the famous [Rich](https://github.com/Textualize/rich) Python library.

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

## Quick Start

### Rust

Get started with the [Rust Library](rust_getting_started.md).

```toml
[dependencies]
fast-rich = "0.2.0"
```

### Python

See the [Python Bindings Installation](python_install.md) guide.

```python
import fast_rich
console = fast_rich.Console()
console.print("Fast!")
```

## Comparisons

Check out the [Benchmarks](benchmarks.md) to see how `fast-rich` compares to the original Python implementation.
