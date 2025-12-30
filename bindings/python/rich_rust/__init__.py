"""
rich_rust - A drop-in replacement for Python Rich with Rust performance.

Usage:
    # Instead of:
    # from rich.console import Console
    # from rich.table import Table
    
    # Use:
    from rich_rust.console import Console
    from rich_rust.table import Table
    
    # Or for full compatibility:
    from rich_rust import print, Console, Table, Panel, Tree, Progress
"""

# Import the Rust bindings
try:
    from rich_rust._core import (
        PyConsole as _RustConsole,
        PyTable as _RustTable,
        PyText as _RustText,
        PyPanel as _RustPanel,
        PyTree as _RustTree,
        PyProgress as _RustProgress,
        PyStyle as _RustStyle,
        PyRule as _RustRule,
        PyMarkdown as _RustMarkdown,
        PySyntax as _RustSyntax,
        PyColumns as _RustColumns,
        PyTraceback as _RustTraceback,
        PyPrompt as _RustPrompt,
        PyLayout as _RustLayout,
        PyLive as _RustLive,
    )
    RUST_AVAILABLE = True
except ImportError:
    RUST_AVAILABLE = False

# Re-export main classes with Rich-compatible API
from rich_rust.console import Console
from rich_rust.table import Table
from rich_rust.text import Text
from rich_rust.panel import Panel
from rich_rust.tree import Tree
from rich_rust.progress import Progress, track
from rich_rust.style import Style
from rich_rust.rule import Rule
from rich_rust.markdown import Markdown
from rich_rust.syntax import Syntax
from rich_rust.columns import Columns
from rich_rust.traceback import Traceback, install as install_traceback
from rich_rust.prompt import Prompt, Confirm
from rich_rust.layout import Layout
from rich_rust.live import Live
from rich_rust._print import print

__version__ = "0.2.0"
__all__ = [
    "Console",
    "Table", 
    "Text",
    "Panel",
    "Tree",
    "Progress",
    "track",
    "Style",
    "Rule",
    "Markdown",
    "Syntax",
    "Columns",
    "Traceback",
    "install_traceback",
    "Prompt",
    "Confirm",
    "Layout",
    "Live",
    "print",
]
