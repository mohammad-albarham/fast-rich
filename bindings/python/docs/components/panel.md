# Panel

Display content in bordered panels.

## Basic Usage

```python
from fast_rich.console import Console
from fast_rich.panel import Panel

console = Console()
console.print(Panel("Hello, World!"))
```

## Constructor

```python
Panel(
    renderable,              # Content to display
    title=None,              # Panel title
    subtitle=None,           # Panel subtitle
    title_align="center",    # Title alignment: left, center, right
    subtitle_align="center", # Subtitle alignment
    box=ROUNDED,             # Border style
    safe_box=True,           # ASCII-safe borders
    expand=True,             # Expand to full width
    style=None,              # Panel style
    border_style=None,       # Border style
    width=None,              # Panel width
    height=None,             # Panel height
    padding=(0, 1),          # Content padding
    highlight=False,         # Auto-highlight
)
```

## Class Methods

### fit()

Create a panel that fits content width.

```python
panel = Panel.fit("Short content")
console.print(panel)
```

## Examples

### Titled Panel

```python
from fast_rich.console import Console
from fast_rich.panel import Panel

console = Console()

console.print(Panel(
    "Important information goes here",
    title="Notice",
    subtitle="Read carefully",
))
```

### Styled Panel

```python
from fast_rich.console import Console
from fast_rich.panel import Panel

console = Console()

# Error panel
console.print(Panel(
    "[bold]Error:[/] File not found",
    title="❌ Error",
    border_style="red",
))

# Success panel
console.print(Panel(
    "[bold]Success:[/] Operation completed",
    title="✅ Success",
    border_style="green",
))

# Warning panel
console.print(Panel(
    "[bold]Warning:[/] Low disk space",
    title="⚠️ Warning",
    border_style="yellow",
))
```

### Nested Panels

```python
from fast_rich.console import Console
from fast_rich.panel import Panel

console = Console()

inner = Panel("Inner content", title="Inner")
outer = Panel(inner, title="Outer")

console.print(outer)
```

### Panel with Table

```python
from fast_rich.console import Console
from fast_rich.panel import Panel
from fast_rich.table import Table

console = Console()

table = Table()
table.add_column("Key")
table.add_column("Value")
table.add_row("Name", "Alice")
table.add_row("Age", "30")

console.print(Panel(table, title="User Info"))
```
