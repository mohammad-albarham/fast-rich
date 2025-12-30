# Quick Start

This guide will get you up and running with fast_rich in minutes.

## Basic Console Output

```python
from fast_rich.console import Console

console = Console()

# Simple print
console.print("Hello, World!")

# Styled text
console.print("[bold cyan]Bold cyan text[/]")
console.print("[red on white]Red text on white background[/]")

# Multiple styles
console.print("[bold italic]Bold and italic[/] mixed with [underline]underline[/]")
```

## Create a Table

```python
from fast_rich.console import Console
from fast_rich.table import Table

console = Console()

# Create table with title
table = Table(title="User Data")

# Add columns
table.add_column("ID", style="cyan", justify="right")
table.add_column("Name", style="magenta")
table.add_column("Email", style="green")

# Add rows
table.add_row("1", "Alice", "alice@example.com")
table.add_row("2", "Bob", "bob@example.com")
table.add_row("3", "Charlie", "charlie@example.com")

console.print(table)
```

## Display a Panel

```python
from fast_rich.console import Console
from fast_rich.panel import Panel

console = Console()

# Simple panel
console.print(Panel("Hello from inside a panel!"))

# Panel with title
console.print(Panel(
    "This is important content",
    title="Notice",
    subtitle="Please read carefully"
))

# Panel with custom style
console.print(Panel(
    "[bold yellow]Warning message[/]",
    title="⚠️ Warning",
    border_style="yellow"
))
```

## Show Progress

```python
from fast_rich.progress import track
import time

# Simple progress bar
for item in track(range(100), description="Processing..."):
    time.sleep(0.01)  # Simulate work
```

```python
from fast_rich.progress import Progress

# Advanced progress with multiple tasks
with Progress() as progress:
    task1 = progress.add_task("[red]Downloading...", total=100)
    task2 = progress.add_task("[green]Processing...", total=100)
    
    while not progress.finished:
        progress.update(task1, advance=0.9)
        progress.update(task2, advance=0.5)
        time.sleep(0.02)
```

## Tree Structure

```python
from fast_rich.console import Console
from fast_rich.tree import Tree

console = Console()

# Create tree
tree = Tree("📁 Project")
docs = tree.add("📁 docs")
docs.add("📄 index.md")
docs.add("📄 guide.md")

src = tree.add("📁 src")
src.add("🐍 main.py")
src.add("🐍 utils.py")

tree.add("📄 README.md")

console.print(tree)
```

## Markdown Rendering

```python
from fast_rich.console import Console
from fast_rich.markdown import Markdown

console = Console()

md = Markdown("""
# Welcome to fast_rich

This is **bold** and this is *italic*.

## Features
- Fast performance
- Easy to use
- Drop-in replacement

```python
print("Hello, World!")
```
""")

console.print(md)
```

## Code Syntax Highlighting

```python
from fast_rich.console import Console
from fast_rich.syntax import Syntax

console = Console()

code = '''
def hello(name: str) -> str:
    """Say hello."""
    return f"Hello, {name}!"

if __name__ == "__main__":
    print(hello("World"))
'''

syntax = Syntax(code, "python", theme="monokai", line_numbers=True)
console.print(syntax)
```

## Print JSON

```python
from fast_rich.console import Console

console = Console()

data = {
    "name": "fast_rich",
    "version": "0.3.0",
    "features": ["fast", "compatible", "easy"],
    "benchmarks": {"table": 84.9, "json": 112.5}
}

console.print_json(data=data)
```

## Status Spinner

```python
from fast_rich.console import Console
import time

console = Console()

with console.status("[bold green]Working on something..."):
    time.sleep(2)  # Simulate work

console.print("[bold green]Done!")
```

## Prompt for Input

```python
from fast_rich.prompt import Prompt, Confirm

# Text input
name = Prompt.ask("What is your name?")

# With default
color = Prompt.ask("Favorite color?", default="blue")

# Password (hidden)
password = Prompt.ask("Password", password=True)

# Confirmation
if Confirm.ask("Do you want to continue?"):
    print("Continuing...")
```

## Logging Integration

```python
import logging
from fast_rich.logging import RichHandler

# Setup rich logging
logging.basicConfig(
    level=logging.INFO,
    format="%(message)s",
    handlers=[RichHandler()]
)

log = logging.getLogger("app")
log.info("Application started")
log.warning("This is a warning")
log.error("Something went wrong!")
```

## Next Steps

- [Migration from Rich](migration.md) - Migrate your existing code
- [Console Reference](../components/console.md) - Full Console API
- [Table Reference](../components/table.md) - Full Table API
