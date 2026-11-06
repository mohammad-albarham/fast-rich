# Text & Style

Create styled text with rich formatting.

## Text

### Basic Usage

```python
from fast_rich.text import Text

text = Text("Hello, World!")
```

### Constructor

```python
Text(
    text="",                    # Initial text
    style=None,                 # Base style
    justify=None,               # Justification: left, center, right
    overflow=None,              # Overflow handling
    no_wrap=False,              # Disable wrapping
    end="\n",                   # Line ending
    tab_size=None,              # Tab size
)
```

### Methods

#### append()

```python
from fast_rich.text import Text

text = Text()
text.append("Hello ", style="bold")
text.append("World", style="italic red")
text.append("!")

# Can chain
text.append("A").append("B").append("C")
```

#### stylize()

Apply style to a range.

```python
text = Text("Hello World")
text.stylize("bold", 0, 5)   # Make "Hello" bold
text.stylize("red", 6, 11)   # Make "World" red
```

#### assemble() (class method)

Create text from multiple parts.

```python
text = Text.assemble(
    "Normal ",
    ("bold", "bold"),
    " ",
    ("red text", "red"),
)
```

#### split()

Split text by separator.

```python
text = Text("one two three")
parts = text.split(" ")  # ["one", "two", "three"]
```

#### join()

Join texts with separator.

```python
texts = [Text("a"), Text("b"), Text("c")]
result = Text(", ").join(texts)  # "a, b, c"
```

### Properties

```python
text = Text("Hello")

text.plain    # Get plain text without styles
len(text)     # Length of text
```

## Style

### Basic Usage

```python
from fast_rich.style import Style

style = Style(bold=True, color="red")
```

### Constructor

```python
Style(
    color=None,           # Foreground color
    bgcolor=None,         # Background color
    bold=None,            # Bold
    dim=None,             # Dim
    italic=None,          # Italic
    underline=None,       # Underline
    blink=None,           # Blink
    blink2=None,          # Fast blink
    reverse=None,         # Reverse
    conceal=None,         # Hidden
    strike=None,          # Strikethrough
    underline2=None,      # Double underline
    frame=None,           # Framed
    encircle=None,        # Encircled
    overline=None,        # Overline
    link=None,            # Hyperlink URL
)
```

### parse()

Parse style from string.

```python
style = Style.parse("bold red on white")
style = Style.parse("italic underline cyan")
```

### Combining Styles

```python
style1 = Style(bold=True)
style2 = Style(color="red")
combined = style1 + style2  # bold red
```

## Examples

### Rich Text

```python
from fast_rich.console import Console
from fast_rich.text import Text

console = Console()

# Build styled text
text = Text()
text.append("Error: ", style="bold red")
text.append("File not found: ", style="yellow")
text.append("/path/to/file", style="cyan underline")

console.print(text)
```

### Highlighting

```python
from fast_rich.console import Console
from fast_rich.text import Text

console = Console()

# Create and highlight
code = Text("function hello() { return 'world'; }")
code.stylize("cyan", 0, 8)      # function
code.stylize("yellow", 9, 14)    # hello
code.stylize("green", 27, 34)    # 'world'

console.print(code)
```

### Text from Markup

```python
from fast_rich.console import Console
from fast_rich.text import Text

console = Console()

# Using markup in Text
text = Text.from_markup("[bold]Hello[/] [italic red]World[/]!")
console.print(text)
```

### Styled Output

```python
from fast_rich.console import Console
from fast_rich.style import Style

console = Console()

# Create style
error_style = Style(bold=True, color="red")
warning_style = Style(color="yellow", italic=True)
success_style = Style(color="green", bold=True)

console.print("Error!", style=error_style)
console.print("Warning!", style=warning_style)
console.print("Success!", style=success_style)
```

### Color Names

Available colors:

| Standard | Bright |
| :--- | :--- |
| `black` | `bright_black` (gray) |
| `red` | `bright_red` |
| `green` | `bright_green` |
| `yellow` | `bright_yellow` |
| `blue` | `bright_blue` |
| `magenta` | `bright_magenta` |
| `cyan` | `bright_cyan` |
| `white` | `bright_white` |

Plus 256 colors and RGB:

```python
# 256 color
style = Style(color="color(196)")

# RGB
style = Style(color="rgb(255,128,0)")

# Hex
style = Style(color="#ff8000")
```
