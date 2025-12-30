"""Pretty printing - matches rich.pretty API."""

from __future__ import annotations

import pprint
from typing import Any, Optional, IO

from fast_rich.console import Console
from fast_rich.text import Text
from fast_rich.panel import Panel


def pretty_repr(
    obj: Any,
    *,
    max_width: int = 80,
    indent_size: int = 4,
    max_length: Optional[int] = None,
    max_string: Optional[int] = None,
    max_depth: Optional[int] = None,
    expand_all: bool = False,
) -> str:
    """Generate a pretty representation of an object.
    
    Args:
        obj: Object to represent.
        max_width: Maximum width.
        indent_size: Indentation size.
        max_length: Max items in collections.
        max_string: Max string length.
        max_depth: Max nesting depth.
        expand_all: Expand all structures.
        
    Returns:
        Pretty string representation.
    """
    return pprint.pformat(obj, indent=indent_size, width=max_width, depth=max_depth)


def pprint(
    obj: Any,
    *,
    console: Optional[Console] = None,
    indent_guides: bool = True,
    max_length: Optional[int] = None,
    max_string: Optional[int] = None,
    max_depth: Optional[int] = None,
    expand_all: bool = False,
) -> None:
    """Pretty print an object to the console.
    
    Args:
        obj: Object to print.
        console: Console to use.
        indent_guides: Show indent guides.
        max_length: Max collection items.
        max_string: Max string length.
        max_depth: Max nesting depth.
        expand_all: Expand structures.
    """
    _console = console or Console()
    output = pretty_repr(
        obj,
        max_length=max_length,
        max_string=max_string,
        max_depth=max_depth,
        expand_all=expand_all,
    )
    _console.print(output)


class Pretty:
    """A renderable that pretty prints an object.
    
    Matches rich.pretty.Pretty API.
    """

    def __init__(
        self,
        obj: Any,
        *,
        highlighter: Optional[Any] = None,
        indent_size: int = 4,
        justify: Optional[str] = None,
        overflow: Optional[str] = None,
        no_wrap: bool = False,
        indent_guides: bool = False,
        max_length: Optional[int] = None,
        max_string: Optional[int] = None,
        max_depth: Optional[int] = None,
        expand_all: bool = False,
        margin: int = 0,
        insert_line: bool = False,
    ) -> None:
        self.obj = obj
        self.highlighter = highlighter
        self.indent_size = indent_size
        self.justify = justify
        self.overflow = overflow
        self.no_wrap = no_wrap
        self.indent_guides = indent_guides
        self.max_length = max_length
        self.max_string = max_string
        self.max_depth = max_depth
        self.expand_all = expand_all
        self.margin = margin
        self.insert_line = insert_line

    def __str__(self) -> str:
        """Render as string."""
        return pretty_repr(
            self.obj,
            indent_size=self.indent_size,
            max_length=self.max_length,
            max_string=self.max_string,
            max_depth=self.max_depth,
            expand_all=self.expand_all,
        )

    def __rich_console__(self, console, options):
        """Rich console protocol."""
        yield str(self)


def install(
    console: Optional[Console] = None,
    overflow: str = "ignore",
    crop: bool = False,
    indent_guides: bool = False,
    max_width: Optional[int] = None,
    max_length: Optional[int] = None,
    max_string: Optional[int] = None,
    max_depth: Optional[int] = None,
    expand_all: bool = False,
) -> None:
    """Install automatic pretty printing in the REPL.
    
    Args:
        console: Console to use.
        overflow: Overflow handling.
        crop: Crop output.
        indent_guides: Show indent guides.
        max_width: Maximum width.
        max_length: Max collection items.
        max_string: Max string length.
        max_depth: Max nesting depth.
        expand_all: Expand structures.
    """
    import sys

    _console = console or Console()

    def _displayhook(value: Any) -> None:
        if value is not None:
            sys.stdout.write("\n")
            pprint(
                value,
                console=_console,
                max_length=max_length,
                max_string=max_string,
                max_depth=max_depth,
                expand_all=expand_all,
            )
            sys.stdout.write("\n")

    sys.displayhook = _displayhook


__all__ = ["Pretty", "pprint", "pretty_repr", "install"]
