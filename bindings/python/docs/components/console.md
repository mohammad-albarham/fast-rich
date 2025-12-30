# Console

The Console is the main entry point for fast_rich. It handles all terminal output.

## Basic Usage

```python
from fast_rich.console import Console

console = Console()
console.print("Hello, World!")
```

## Constructor

```python
Console(
    file=None,                    # Output file (default: stdout)
    width=None,                   # Terminal width (auto-detect)
    height=None,                  # Terminal height (auto-detect)
    color_system="auto",          # Color system: "auto", "standard", "256", "truecolor"
    force_terminal=False,         # Force terminal mode
    no_color=False,               # Disable colors
    tab_size=8,                   # Tab size
    record=False,                 # Record output for export
    markup=True,                  # Enable Rich markup
    emoji=True,                   # Enable emoji shortcodes
    highlight=True,               # Enable auto-highlighting
    log_time=True,                # Show time in log()
    log_path=True,                # Show path in log()
    log_time_format="[%X]",       # Time format
    theme=None,                   # Custom theme
    stderr=False,                 # Output to stderr
    quiet=False,                  # Suppress output
)
```

## Methods

### print()

Print styled output to the console.

```python
from fast_rich.console import Console

console = Console()

# Basic print
console.print("Hello, World!")

# With styles
console.print("[bold]Bold text[/]")
console.print("[red]Red[/] and [blue]blue[/] text")

# Multiple arguments
console.print("Name:", "Alice", "Age:", 30)

# With options
console.print("Centered", justify="center")
console.print("Styled", style="bold green")
```

### log()

Log with timestamp and path.

```python
console.log("Application started")
console.log("Processing complete", log_locals=True)
```

### rule()

Print a horizontal rule.

```python
console.rule()
console.rule("Section Title")
console.rule("[bold red]Important", style="red")
```

### print_json()

Pretty print JSON data.

```python
# From string
console.print_json('{"name": "Alice", "age": 30}')

# From data
console.print_json(data={"users": [1, 2, 3]})
```

### status()

Show a spinner during operations.

```python
with console.status("Loading..."):
    import time
    time.sleep(2)
```

### input()

Get user input with prompt.

```python
name = console.input("Enter your name: ")
```

### clear()

Clear the terminal screen.

```python
console.clear()
```

## Properties

```python
console = Console()

console.width       # Terminal width
console.height      # Terminal height
console.is_terminal # True if output is a terminal
console.encoding    # Character encoding
console.color_system # Active color system
```

## Export Methods

```python
console = Console(record=True)
console.print("Hello, World!")

# Export as text
text = console.export_text()

# Export as HTML
html = console.export_html()

# Export as SVG
svg = console.export_svg(title="My Output")

# Save to file
console.save_text("output.txt")
console.save_html("output.html")
```

## Markup Reference

| Markup | Description |
| :--- | :--- |
| `[bold]text[/]` | Bold text |
| `[italic]text[/]` | Italic text |
| `[underline]text[/]` | Underlined text |
| `[red]text[/]` | Red text |
| `[on blue]text[/]` | Blue background |
| `[red on white]text[/]` | Red text on white |
| `[link=URL]text[/]` | Clickable link |

## Examples

### Status Messages

```python
from fast_rich.console import Console

console = Console()

console.print("[bold green]✓[/] Task completed")
console.print("[bold yellow]⚠[/] Warning message")
console.print("[bold red]✗[/] Error occurred")
```

### Formatted Output

```python
from fast_rich.console import Console

console = Console()

# Right-aligned
console.print("[cyan]Name:[/]", justify="right")

# Centered
console.print("[bold magenta]= TITLE =[/]", justify="center")

# With emoji
console.print(":rocket: Launching :star:")
```

### Recording Output

```python
from fast_rich.console import Console

console = Console(record=True)
console.print("[bold]Hello[/]")
console.print("[italic]World[/]")

# Get plain text
plain = console.export_text()

# Get styled HTML
html = console.export_html()
```
