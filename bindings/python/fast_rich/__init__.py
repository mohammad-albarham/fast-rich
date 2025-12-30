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

__version__ = "0.3.0"

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

# Additional features (new)
from fast_rich.pretty import Pretty, pprint
from fast_rich.emoji import Emoji
from fast_rich.spinner import Spinner

# 100% coverage modules
from fast_rich.status import Status
from fast_rich.align import Align, VerticalCenter
from fast_rich.padding import Padding
from fast_rich.json import JSON
from fast_rich.highlighter import (
    Highlighter,
    NullHighlighter,
    RegexHighlighter,
    ReprHighlighter,
    JSONHighlighter,
)
from fast_rich.theme import Theme, DEFAULT_THEME
from fast_rich.filesize import decimal as filesize
from fast_rich.segment import Segment, Segments
from fast_rich.measure import Measurement
from fast_rich.scope import render_scope
from fast_rich.control import Control
from fast_rich.region import Region
from fast_rich.color import Color, ColorTriplet
from fast_rich.logging import RichHandler
from fast_rich.styled import Styled
from fast_rich.repr import auto as repr_auto
from fast_rich.terminal_theme import TerminalTheme, MONOKAI
from fast_rich.containers import Lines, Renderables, Group, group
from fast_rich.console_options import ConsoleOptions, ConsoleDimensions

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
    # Additional features
    "Pretty",
    "pprint",
    "Emoji",
    "Spinner",
    "Status",
    # 100% coverage - Alignment & Layout
    "Align",
    "VerticalCenter",
    "Padding",
    # JSON
    "JSON",
    # Highlighters
    "Highlighter",
    "NullHighlighter",
    "RegexHighlighter",
    "ReprHighlighter",
    "JSONHighlighter",
    # Theme
    "Theme",
    "DEFAULT_THEME",
    "TerminalTheme",
    "MONOKAI",
    # Utilities
    "filesize",
    "Segment",
    "Segments",
    "Measurement",
    "render_scope",
    "Control",
    "Region",
    "Color",
    "ColorTriplet",
    "Styled",
    "repr_auto",
    # Containers
    "Lines",
    "Renderables",
    "Group",
    "group",
    # Console
    "ConsoleOptions",
    "ConsoleDimensions",
    # Logging
    "RichHandler",
    # Global print
    "print",
]


