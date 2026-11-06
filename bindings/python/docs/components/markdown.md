# Markdown

Render Markdown with syntax highlighting.

## Basic Usage

```python
from fast_rich.console import Console
from fast_rich.markdown import Markdown

console = Console()

md = Markdown("# Hello World")
console.print(md)
```

## Constructor

```python
Markdown(
    markup,                       # Markdown string
    code_theme="monokai",         # Code block theme
    justify=None,                 # Text justification
    style="none",                 # Base style
    hyperlinks=True,              # Render hyperlinks
    inline_code_lexer=None,       # Inline code lexer
    inline_code_theme=None,       # Inline code theme
)
```

## Examples

### Full Markdown

```python
from fast_rich.console import Console
from fast_rich.markdown import Markdown

console = Console()

md = Markdown("""
# Main Heading

This is a paragraph with **bold** and *italic* text.

## Subheading

Here's a list:

- Item one
- Item two
- Item three

### Code Block

```python
def hello(name):
    return f"Hello, {name}!"
```

> This is a blockquote

| Col 1 | Col 2 |
|-------|-------|
| A     | B     |
| C     | D     |
""")

console.print(md)
```

### Reading from File

```python
from fast_rich.console import Console
from fast_rich.markdown import Markdown

console = Console()

with open("README.md") as f:
    md = Markdown(f.read())

console.print(md)
```
