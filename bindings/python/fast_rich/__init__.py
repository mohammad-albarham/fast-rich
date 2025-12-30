"""
fast_rich - A drop-in replacement for Python Rich with Rust performance.

Usage:
    # Instead of:
    from rich.console import Console
    from rich.table import Table

    # Use:
    from fast_rich.console import Console
    from fast_rich.table import Table

    # Everything works the same, just faster!
"""

from __future__ import annotations

__version__ = "0.2.0"

# Import core classes with Rich-compatible API
from fast_rich.console import Console
from fast_rich.table import Table
from fast_rich.text import Text
from fast_rich.style import Style
from fast_rich.panel import Panel
from fast_rich.rule import Rule
from fast_rich.box import (
    Box,
    ROUNDED,
    SQUARE,
    MINIMAL,
    HORIZONTALS,
    SIMPLE,
    HEAVY,
    DOUBLE,
    ASCII,
)

# Extended components
from fast_rich.progress import Progress, track
from fast_rich.tree import Tree
from fast_rich.markdown import Markdown
from fast_rich.syntax import Syntax
from fast_rich.columns import Columns
from fast_rich.traceback import Traceback, install as install_traceback
from fast_rich.layout import Layout
from fast_rich.live import Live
from fast_rich.prompt import Prompt, Confirm
from fast_rich.inspect import inspect

# Global print function
from fast_rich._print import print

__all__ = [
    # Core
    "Console",
    "Table",
    "Text",
    "Style",
    "Panel",
    "Rule",
    # Box styles
    "Box",
    "ROUNDED",
    "SQUARE",
    "MINIMAL",
    "HORIZONTALS",
    "SIMPLE",
    "HEAVY",
    "DOUBLE",
    "ASCII",
    # Extended
    "Progress",
    "track",
    "Tree",
    "Markdown",
    "Syntax",
    "Columns",
    "Traceback",
    "install_traceback",
    "Layout",
    "Live",
    "Prompt",
    "Confirm",
    "inspect",
    # Utilities
    "print",
]
