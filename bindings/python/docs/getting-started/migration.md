# Migration from Rich

Migrating from Python Rich to fast_rich is simple - just change your imports!

## Automatic Migration

You can use a simple find-and-replace:

```bash
# Replace all imports
find . -name "*.py" -exec sed -i 's/from rich\./from fast_rich./g' {} \;
find . -name "*.py" -exec sed -i 's/import rich/import fast_rich/g' {} \;
```

## Manual Migration

### Before (Rich)

```python
from rich.console import Console
from rich.table import Table
from rich.panel import Panel
from rich.progress import Progress, track
from rich.tree import Tree
from rich.markdown import Markdown
from rich.syntax import Syntax
from rich.logging import RichHandler

console = Console()
console.print("[bold green]Hello, World![/]")
```

### After (fast_rich)

```python
from fast_rich.console import Console
from fast_rich.table import Table
from fast_rich.panel import Panel
from fast_rich.progress import Progress, track
from fast_rich.tree import Tree
from fast_rich.markdown import Markdown
from fast_rich.syntax import Syntax
from fast_rich.logging import RichHandler

console = Console()
console.print("[bold green]Hello, World![/]")
```

## Import Mapping

| Rich Import | fast_rich Import |
| :--- | :--- |
| `from rich.console import Console` | `from fast_rich.console import Console` |
| `from rich.table import Table` | `from fast_rich.table import Table` |
| `from rich.text import Text` | `from fast_rich.text import Text` |
| `from rich.panel import Panel` | `from fast_rich.panel import Panel` |
| `from rich.progress import Progress` | `from fast_rich.progress import Progress` |
| `from rich.tree import Tree` | `from fast_rich.tree import Tree` |
| `from rich.markdown import Markdown` | `from fast_rich.markdown import Markdown` |
| `from rich.syntax import Syntax` | `from fast_rich.syntax import Syntax` |
| `from rich.prompt import Prompt` | `from fast_rich.prompt import Prompt` |
| `from rich.logging import RichHandler` | `from fast_rich.logging import RichHandler` |
| `from rich import print` | `from fast_rich import print` |

## Full Module List

All 60 modules are supported:

```python
# Core
from fast_rich.console import Console
from fast_rich.table import Table
from fast_rich.text import Text
from fast_rich.style import Style
from fast_rich.panel import Panel
from fast_rich.rule import Rule
from fast_rich.box import ROUNDED, SIMPLE

# Extended
from fast_rich.progress import Progress, track
from fast_rich.tree import Tree
from fast_rich.markdown import Markdown
from fast_rich.syntax import Syntax
from fast_rich.columns import Columns
from fast_rich.traceback import Traceback, install
from fast_rich.layout import Layout
from fast_rich.live import Live
from fast_rich.prompt import Prompt, Confirm
from fast_rich.inspect import inspect

# Additional
from fast_rich.pretty import Pretty, pprint
from fast_rich.emoji import Emoji
from fast_rich.spinner import Spinner
from fast_rich.status import Status
from fast_rich.align import Align
from fast_rich.padding import Padding
from fast_rich.json import JSON
from fast_rich.highlighter import Highlighter
from fast_rich.theme import Theme
from fast_rich.logging import RichHandler

# And 30+ more modules...
```

## Performance Comparison

After migration, you'll see significant speedups:

| Feature | Rich | fast_rich | Speedup |
| :--- | ---: | ---: | ---: |
| JSON | 7.67ms | 0.07ms | **112.5x** |
| Markdown | 9.35ms | 0.10ms | **92.6x** |
| Table (1000) | 66.85ms | 0.79ms | **84.9x** |
| Average | - | - | **39.1x** |

## Compatibility Notes

### Fully Compatible

- All public APIs work identically
- All styles and markup work the same
- Progress bars, tables, trees, panels all compatible
- Console output is identical

### Minor Differences

1. **Internal APIs**: Private `_*` methods may differ
2. **Error Messages**: Some error text may vary slightly
3. **Performance**: fast_rich is faster (not a problem!)

## Troubleshooting

### Import Error

If you see `ModuleNotFoundError: No module named 'fast_rich'`:

```bash
pip install fast-rich
```

### Attribute Error

If you get `AttributeError` for a method:

1. Check if you're using a private (`_`) method
2. Ensure you have the latest version: `pip install --upgrade fast-rich`
3. Check the [API Reference](../api/index.md)

## Need Help?

- [GitHub Issues](https://github.com/mohammad-albarham/fast-rich/issues)
- [Full Documentation](../index.md)
