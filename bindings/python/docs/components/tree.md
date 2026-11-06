# Tree

Display hierarchical data as trees.

## Basic Usage

```python
from fast_rich.console import Console
from fast_rich.tree import Tree

console = Console()

tree = Tree("Root")
tree.add("Child 1")
tree.add("Child 2")

console.print(tree)
```

## Constructor

```python
Tree(
    label,                    # Root label
    style=None,               # Label style
    guide_style="tree.line",  # Guide line style
    expanded=True,            # Expand children
    highlight=False,          # Auto-highlight
)
```

## Methods

### add()

Add a child node.

```python
child = tree.add(
    label,              # Node label
    style=None,         # Label style
    guide_style=None,   # Guide style
    expanded=True,      # Expand this node
    highlight=False,    # Auto-highlight
)
```

Returns the new child Tree for nesting.

## Examples

### File Tree

```python
from fast_rich.console import Console
from fast_rich.tree import Tree

console = Console()

tree = Tree("📁 [bold]project[/]")

# Source directory
src = tree.add("📁 [bold blue]src[/]")
src.add("🐍 main.py")
src.add("🐍 utils.py")
src.add("🐍 config.py")

# Docs directory
docs = tree.add("📁 [bold green]docs[/]")
docs.add("📄 index.md")
docs.add("📄 guide.md")
docs.add("📄 api.md")

# Tests
tests = tree.add("📁 [bold yellow]tests[/]")
tests.add("🧪 test_main.py")
tests.add("🧪 test_utils.py")

# Root files
tree.add("📄 README.md")
tree.add("📄 pyproject.toml")

console.print(tree)
```

### Deep Nesting

```python
from fast_rich.console import Console
from fast_rich.tree import Tree

console = Console()

tree = Tree("Root")

level1 = tree.add("Level 1")
level2 = level1.add("Level 2")
level3 = level2.add("Level 3")
level4 = level3.add("Level 4")
level5 = level4.add("Level 5 - Deep!")

console.print(tree)
```

### Styled Tree

```python
from fast_rich.console import Console
from fast_rich.tree import Tree

console = Console()

tree = Tree(
    "[bold cyan]🏠 Home[/]",
    guide_style="bright_blue",
)

apps = tree.add("[bold green]📱 Apps[/]")
apps.add("[dim]🎮 Games[/]")
apps.add("[dim]📧 Mail[/]")
apps.add("[dim]🗓 Calendar[/]")

docs = tree.add("[bold yellow]📁 Documents[/]")
docs.add("[dim]📄 Report.docx[/]")
docs.add("[dim]📊 Data.xlsx[/]")

console.print(tree)
```

### JSON as Tree

```python
from fast_rich.console import Console
from fast_rich.tree import Tree

def json_to_tree(data, tree=None, key=None):
    if tree is None:
        tree = Tree(f"[bold]{key or 'root'}[/]")
    
    if isinstance(data, dict):
        for k, v in data.items():
            if isinstance(v, (dict, list)):
                branch = tree.add(f"[cyan]{k}[/]")
                json_to_tree(v, branch)
            else:
                tree.add(f"[cyan]{k}[/]: [yellow]{v}[/]")
    elif isinstance(data, list):
        for i, item in enumerate(data):
            if isinstance(item, (dict, list)):
                branch = tree.add(f"[magenta][{i}][/]")
                json_to_tree(item, branch)
            else:
                tree.add(f"[magenta][{i}][/]: [yellow]{item}[/]")
    
    return tree

console = Console()

data = {
    "name": "Alice",
    "age": 30,
    "hobbies": ["reading", "coding"],
    "address": {
        "city": "NYC",
        "zip": "10001"
    }
}

tree = json_to_tree(data, key="user")
console.print(tree)
```
