# Syntax

Syntax highlighting for source code.

## Basic Usage

```python
from fast_rich.console import Console
from fast_rich.syntax import Syntax

console = Console()

code = '''
def hello(name):
    return f"Hello, {name}!"
'''

syntax = Syntax(code, "python")
console.print(syntax)
```

## Constructor

```python
Syntax(
    code,                      # Source code string
    lexer,                     # Language lexer name
    theme="monokai",           # Color theme
    dedent=False,              # Remove leading whitespace
    line_numbers=False,        # Show line numbers
    start_line=1,              # Starting line number
    line_range=None,           # Line range to display
    highlight_lines=None,      # Lines to highlight
    code_width=None,           # Code width
    tab_size=4,                # Tab size
    word_wrap=False,           # Wrap long lines
    background_color=None,     # Background color
    indent_guides=False,       # Show indent guides
    padding=0,                 # Padding
)
```

## Class Methods

### from_path()

Load code from file.

```python
syntax = Syntax.from_path("main.py")
console.print(syntax)
```

## Examples

### With Line Numbers

```python
from fast_rich.console import Console
from fast_rich.syntax import Syntax

console = Console()

code = '''
def fibonacci(n):
    if n <= 1:
        return n
    return fibonacci(n-1) + fibonacci(n-2)

print(fibonacci(10))
'''

syntax = Syntax(
    code,
    "python",
    line_numbers=True,
    theme="dracula",
)
console.print(syntax)
```

### Highlighted Lines

```python
from fast_rich.console import Console
from fast_rich.syntax import Syntax

console = Console()

code = '''
def process_data(data):
    result = []
    for item in data:
        if item > 0:
            result.append(item * 2)  # Important line
    return result
'''

syntax = Syntax(
    code,
    "python",
    line_numbers=True,
    highlight_lines={5},  # Highlight line 5
)
console.print(syntax)
```

### Different Languages

```python
from fast_rich.console import Console
from fast_rich.syntax import Syntax

console = Console()

# JavaScript
console.print(Syntax('''
function hello() {
    console.log("Hello, World!");
}
''', "javascript", line_numbers=True))

# Rust
console.print(Syntax('''
fn main() {
    println!("Hello, World!");
}
''', "rust", line_numbers=True))

# SQL
console.print(Syntax('''
SELECT name, age
FROM users
WHERE age > 18
ORDER BY name;
''', "sql", line_numbers=True))
```
