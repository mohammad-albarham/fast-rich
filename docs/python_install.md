# Python Bindings Installation

The `rich-rust` Python bindings provide a high-performance alternative to the standard Rich library for specific rendering tasks.

## Status

> ⚠️ **Note**: The Python bindings are currently experimental and under active development.

## Building from Source

To build and install the bindings, you need Rust and `maturin` installed.

1. **Install Maturin**:
   ```bash
   pip install maturin
   ```

2. **Clone and Build**:
   ```bash
   git clone https://github.com/mohammad-albarham/rich-rust
   cd rich-rust/bindings/python
   maturin develop --release
   ```

## Usage

```python
import rich_rust

console = rich_rust.Console()
console.print("[bold green]Hello from Rust![/]")
```
