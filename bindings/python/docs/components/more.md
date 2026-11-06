# More Components

Additional components available in fast_rich.

## Columns

Display items in columns.

```python
from fast_rich.console import Console
from fast_rich.columns import Columns

console = Console()

items = [f"Item {i}" for i in range(20)]
console.print(Columns(items))
```

## Rule

Horizontal divider line.

```python
from fast_rich.console import Console
from fast_rich.rule import Rule

console = Console()

console.print(Rule())
console.print(Rule("Section Title"))
console.print(Rule("Important", style="red"))
```

## Align

Align content.

```python
from fast_rich.console import Console
from fast_rich.align import Align

console = Console()

console.print(Align.center("Centered text"))
console.print(Align.right("Right aligned"))
console.print(Align.left("Left aligned"))
```

## Padding

Add padding around content.

```python
from fast_rich.console import Console
from fast_rich.padding import Padding

console = Console()

console.print(Padding("Padded content", (2, 4)))  # (top/bottom, left/right)
```

## Prompt

Interactive prompts.

```python
from fast_rich.prompt import Prompt, Confirm, IntPrompt

name = Prompt.ask("Name")
age = IntPrompt.ask("Age")
proceed = Confirm.ask("Continue?")
```

## Layout

Split terminal into regions.

```python
from fast_rich.console import Console
from fast_rich.layout import Layout

console = Console()

layout = Layout()
layout.split_column(
    Layout(name="header"),
    Layout(name="body"),
    Layout(name="footer"),
)

layout["header"].update("Header")
layout["body"].update("Body")
layout["footer"].update("Footer")

console.print(layout)
```

## Live

Live updating display.

```python
from fast_rich.live import Live
from fast_rich.table import Table
import time

def generate_table():
    table = Table()
    table.add_column("Time")
    table.add_row(str(time.time()))
    return table

with Live(generate_table(), refresh_per_second=4) as live:
    for _ in range(10):
        time.sleep(0.4)
        live.update(generate_table())
```

## Traceback

Rich tracebacks.

```python
from fast_rich.traceback import install
install()  # Install globally

# Now exceptions will be beautifully formatted!
```

## Emoji

Use emoji shortcodes.

```python
from fast_rich.console import Console
from fast_rich.emoji import Emoji

console = Console()

console.print(":rocket: Launch!")
console.print(Emoji("star"))
```

## JSON

Pretty-print JSON.

```python
from fast_rich.console import Console
from fast_rich.json import JSON

console = Console()

data = JSON('{"name": "Alice", "age": 30}')
console.print(data)
```

## Pretty

Pretty-print Python objects.

```python
from fast_rich.pretty import pprint

data = {"users": [{"name": "Alice"}, {"name": "Bob"}]}
pprint(data)
```

## Spinner

Animated spinners.

```python
from fast_rich.spinner import Spinner

spinner = Spinner("dots")
# Use with Status or Live
```

## Logging

Rich log handler.

```python
import logging
from fast_rich.logging import RichHandler

logging.basicConfig(handlers=[RichHandler()])
log = logging.getLogger()
log.info("Application started")
```

## Inspect

Inspect objects.

```python
from fast_rich.inspect import inspect

inspect(str)  # Inspect string class
```

## All 60 Modules

| Module | Description |
| :--- | :--- |
| abc | Abstract base classes |
| align | Alignment |
| ansi | ANSI parsing |
| bar | Bar renderable |
| box | Border styles |
| cells | Cell width utilities |
| color | Colors |
| color_triplet | RGB triplets |
| columns | Column layout |
| console | Console output |
| console_options | Console options |
| constrain | Width constraints |
| containers | Container classes |
| control | Control codes |
| default_styles | Default styles |
| diagnose | Diagnostics |
| emoji | Emoji support |
| errors | Error classes |
| file_proxy | File proxy |
| filesize | File size formatting |
| highlighter | Syntax highlighters |
| inspect | Object inspection |
| json | JSON rendering |
| jupyter | Jupyter support |
| layout | Layout splitting |
| live | Live display |
| live_render | Live rendering |
| logging | Logging handler |
| markdown | Markdown rendering |
| markup | Markup parsing |
| measure | Measurement |
| padding | Padding |
| pager | Pager support |
| palette | Color palettes |
| panel | Bordered panels |
| pretty | Pretty printing |
| progress | Progress bars |
| progress_bar | Progress bar widget |
| prompt | User prompts |
| protocol | Rich protocol |
| region | Screen regions |
| repr | Repr utilities |
| rule | Horizontal rules |
| scope | Variable scope |
| screen | Screen handling |
| segment | Text segments |
| spinner | Loading spinners |
| status | Status display |
| style | Text styles |
| styled | Styled wrapper |
| syntax | Code syntax |
| table | Data tables |
| terminal_theme | Terminal themes |
| text | Styled text |
| theme | Theming |
| themes | Theme definitions |
| traceback | Rich tracebacks |
| tree | Tree structures |
