# Installation

## Requirements

- Python 3.10 or higher
- pip or uv package manager

## Install from PyPI

```bash
pip install fast-rich
```

Or using `uv` (recommended for speed):

```bash
uv pip install fast-rich
```

## Install from Source

```bash
git clone https://github.com/mohammad-albarham/fast-rich.git
cd fast-rich/bindings/python
pip install -e .
```

## Verify Installation

```python
import fast_rich
print(f"fast_rich version: {fast_rich.__version__}")

from fast_rich.console import Console
console = Console()
console.print("[bold green]✓ Installation successful![/]")
```

Output:
```
fast_rich version: 0.3.0
✓ Installation successful!
```

## Upgrade

```bash
pip install --upgrade fast-rich
```

## Uninstall

```bash
pip uninstall fast-rich
```
