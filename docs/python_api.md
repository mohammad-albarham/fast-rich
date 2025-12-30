# Python API

The `rich_rust` python package provides Drop-in-ish replacements for some Rich components, optimized for speed.

## Installation

```bash
pip install rich-rust
```

## Components

### Console

The entry point for printing.

```python
from rich_rust import Console

console = Console()
console.print("Hello [bold]World[/]")
```

### Table

A high-performance table builder.

```python
from rich_rust import Console, Table

table = Table()
table.add_column("ID")
table.add_column("Value")

for i in range(1000):
    table.add_row([str(i), f"Value {i}"])

console = Console()
console.print_table(table)
```

### Progress

A multi-threaded progress bar system.

```python
from rich_rust import Progress
import time

progress = Progress()
task_id = progress.add_task("Downloading...", total=100)

for i in range(100):
    progress.update(task_id, i + 1)
    time.sleep(0.05)
```

## Performance

`rich-rust` is designed to handle massive amounts of data. Use it when:

*   You have > 10,000 rows in a table.
*   You are constrained by Python's render overhead.
*   You need to verify Rust library correctness from Python.
