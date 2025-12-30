# Table

Create beautiful tables with formatting, borders, and styles.

## Basic Usage

```python
from fast_rich.console import Console
from fast_rich.table import Table

console = Console()

table = Table()
table.add_column("Name")
table.add_column("Age")
table.add_row("Alice", "30")
table.add_row("Bob", "25")

console.print(table)
```

## Constructor

```python
Table(
    title=None,              # Table title
    caption=None,            # Table caption (bottom)
    width=None,              # Table width
    min_width=None,          # Minimum width
    box=ROUNDED,             # Border style
    safe_box=None,           # ASCII-safe borders
    padding=(0, 1),          # Cell padding
    collapse_padding=False,  # Collapse padding
    pad_edge=True,           # Pad edges
    expand=False,            # Expand to width
    show_header=True,        # Show header
    show_footer=False,       # Show footer
    show_edge=True,          # Show borders
    show_lines=False,        # Show row lines
    leading=0,               # Leading lines
    style=None,              # Table style
    row_styles=None,         # Alternating row styles
    header_style="bold",     # Header style
    footer_style=None,       # Footer style
    border_style=None,       # Border style
    title_style=None,        # Title style
    caption_style=None,      # Caption style
    title_justify="center",  # Title alignment
    caption_justify="center",# Caption alignment
    highlight=False,         # Auto-highlight
)
```

## add_column()

Add a column to the table.

```python
table.add_column(
    header="Name",           # Column header
    footer=None,             # Column footer
    header_style=None,       # Header cell style
    footer_style=None,       # Footer cell style
    style=None,              # Column style
    justify="left",          # Cell alignment: left, center, right
    vertical="top",          # Vertical alignment
    overflow="ellipsis",     # Overflow handling
    width=None,              # Column width
    min_width=None,          # Minimum width
    max_width=None,          # Maximum width
    ratio=None,              # Width ratio
    no_wrap=False,           # Disable wrapping
)
```

## add_row()

Add a row to the table.

```python
table.add_row(
    "value1",
    "value2",
    "value3",
    style=None,    # Row style
    end_section=False,  # End section after row
)
```

## Box Styles

```python
from fast_rich.box import ROUNDED, SQUARE, MINIMAL, SIMPLE, HEAVY, DOUBLE, ASCII

table = Table(box=ROUNDED)   # ╭───╮ Default
table = Table(box=SQUARE)    # ┌───┐
table = Table(box=MINIMAL)   # No borders
table = Table(box=SIMPLE)    # Simple lines
table = Table(box=HEAVY)     # ┏━━━┓
table = Table(box=DOUBLE)    # ╔═══╗
table = Table(box=ASCII)     # +---+ ASCII only
table = Table(box=None)      # No box at all
```

## Examples

### Basic Table

```python
from fast_rich.console import Console
from fast_rich.table import Table

console = Console()

table = Table(title="Employee Data")
table.add_column("ID", justify="right", style="cyan")
table.add_column("Name", style="magenta")
table.add_column("Department", style="green")
table.add_column("Salary", justify="right", style="yellow")

table.add_row("1", "Alice Smith", "Engineering", "$120,000")
table.add_row("2", "Bob Johnson", "Marketing", "$85,000")
table.add_row("3", "Carol White", "Sales", "$95,000")

console.print(table)
```

### Styled Table

```python
from fast_rich.console import Console
from fast_rich.table import Table

console = Console()

table = Table(
    title="🎯 Project Status",
    caption="Updated: 2024-01-15",
    show_lines=True,
    title_style="bold cyan",
    header_style="bold magenta",
)

table.add_column("Task", style="dim")
table.add_column("Status", justify="center")
table.add_column("Priority", justify="center")

table.add_row("Setup", "[green]✓ Done[/]", "[dim]Low[/]")
table.add_row("Development", "[yellow]In Progress[/]", "[yellow]Medium[/]")
table.add_row("Testing", "[red]Not Started[/]", "[red]High[/]")

console.print(table)
```

### Alternating Row Colors

```python
from fast_rich.console import Console
from fast_rich.table import Table

console = Console()

table = Table(row_styles=["", "dim"])

table.add_column("Item")
table.add_column("Price", justify="right")

for i in range(10):
    table.add_row(f"Product {i+1}", f"${(i+1)*10}.00")

console.print(table)
```

### Wide Table

```python
from fast_rich.console import Console
from fast_rich.table import Table

console = Console()

table = Table(expand=True)  # Expand to full width

table.add_column("Name", ratio=2)
table.add_column("Value", ratio=1)

table.add_row("A very long name here", "100")
table.add_row("Short", "200")

console.print(table)
```

### Nested Tables

```python
from fast_rich.console import Console
from fast_rich.table import Table
from fast_rich.panel import Panel

console = Console()

# Inner table
inner = Table(box=None)
inner.add_column("Key")
inner.add_column("Value")
inner.add_row("A", "1")
inner.add_row("B", "2")

# Outer table
outer = Table(title="Nested Example")
outer.add_column("Description")
outer.add_column("Details")
outer.add_row("Configuration", inner)

console.print(outer)
```
