# API Reference

## Module Overview

fast_rich provides 60 modules for 100% Rich API compatibility.

## Core Modules

### Console

```python
from fast_rich.console import Console
```

The main interface for terminal output.

### Table

```python
from fast_rich.table import Table
```

Create formatted data tables.

### Text

```python
from fast_rich.text import Text
```

Styled text with spans.

### Style

```python
from fast_rich.style import Style
```

Define and combine styles.

### Panel

```python
from fast_rich.panel import Panel
```

Bordered content panels.

### Rule

```python
from fast_rich.rule import Rule
```

Horizontal divider lines.

## Import Reference

```python
# Core
from fast_rich.console import Console
from fast_rich.table import Table
from fast_rich.text import Text
from fast_rich.style import Style
from fast_rich.panel import Panel
from fast_rich.rule import Rule
from fast_rich.box import ROUNDED, SQUARE, SIMPLE

# Extended
from fast_rich.progress import Progress, track
from fast_rich.tree import Tree
from fast_rich.markdown import Markdown
from fast_rich.syntax import Syntax
from fast_rich.columns import Columns
from fast_rich.traceback import Traceback, install
from fast_rich.layout import Layout
from fast_rich.live import Live
from fast_rich.prompt import Prompt, Confirm, IntPrompt, FloatPrompt
from fast_rich.inspect import inspect

# Additional
from fast_rich.pretty import Pretty, pprint
from fast_rich.emoji import Emoji
from fast_rich.spinner import Spinner
from fast_rich.status import Status
from fast_rich.align import Align, VerticalCenter
from fast_rich.padding import Padding
from fast_rich.json import JSON
from fast_rich.highlighter import Highlighter, RegexHighlighter
from fast_rich.theme import Theme
from fast_rich.logging import RichHandler
from fast_rich.markup import escape, render
from fast_rich.bar import Bar
from fast_rich.progress_bar import ProgressBar
from fast_rich.cells import cell_len
from fast_rich.color import Color, ColorTriplet
from fast_rich.segment import Segment
from fast_rich.control import Control
from fast_rich.errors import ConsoleError, StyleError

# Global print
from fast_rich import print
```

## Version

```python
import fast_rich
print(fast_rich.__version__)  # "0.3.0"
```
