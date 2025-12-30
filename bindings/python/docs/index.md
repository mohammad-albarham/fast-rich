# fast_rich

<p align="center">
  <strong>🚀 A drop-in replacement for Python Rich with Rust performance</strong>
</p>

<p align="center">
  <a href="https://github.com/mohammad-albarham/fast-rich">
    <img src="https://img.shields.io/badge/GitHub-fast--rich-blue?logo=github" alt="GitHub">
  </a>
  <img src="https://img.shields.io/badge/version-0.3.0-green" alt="Version">
  <img src="https://img.shields.io/badge/Python-3.10+-blue" alt="Python">
  <img src="https://img.shields.io/badge/coverage-100%25-brightgreen" alt="Coverage">
</p>

---

## What is fast_rich?

**fast_rich** is a drop-in replacement for the popular [Rich](https://github.com/Textualize/rich) Python library, powered by Rust for **11-112x faster performance**.

```python
# Just change your imports!
# Before:
from rich.console import Console
from rich.table import Table

# After:
from fast_rich.console import Console
from fast_rich.table import Table

# Everything works exactly the same, just faster!
```

## Key Features

- ✅ **100% API Compatible** - All 60 modules from Rich are implemented
- 🚀 **Up to 112x Faster** - Rust-powered performance
- 🔄 **Drop-in Replacement** - Just change your imports
- 📦 **Zero Dependencies** - Self-contained package

## Performance Highlights

| Feature | Speedup |
| :--- | ---: |
| JSON | **112.5x** |
| Markdown | **92.6x** |
| Table | **84.9x** |
| Markup | **74.8x** |
| Average | **39.1x** |

## Quick Example

```python
from fast_rich.console import Console
from fast_rich.table import Table
from fast_rich.panel import Panel

console = Console()

# Create a table
table = Table(title="🚀 fast_rich Performance")
table.add_column("Feature", style="cyan")
table.add_column("Speedup", style="green")
table.add_row("JSON", "112.5x")
table.add_row("Markdown", "92.6x")
table.add_row("Table", "84.9x")

console.print(Panel(table, title="Benchmark Results"))
```

## Installation

```bash
pip install fast-rich
```

## Next Steps

- [Installation Guide](getting-started/installation.md)
- [Quick Start Tutorial](getting-started/quickstart.md)
- [Migration from Rich](getting-started/migration.md)
- [Full Benchmarks](benchmarks.md)
