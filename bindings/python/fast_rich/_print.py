"""Global print function - matches rich.print API."""

from __future__ import annotations

import builtins
from typing import Any, IO, Literal, Optional, Union

from fast_rich.console import Console
from fast_rich.style import Style


# Default console for global print
_console: Optional[Console] = None


def get_console() -> Console:
    """Get the global console instance."""
    global _console
    if _console is None:
        _console = Console()
    return _console


def print(
    *objects: Any,
    sep: str = " ",
    end: str = "\n",
    file: Optional[IO[str]] = None,
    flush: bool = False,
    style: Optional[Union[str, Style]] = None,
    justify: Optional[Literal["left", "center", "right", "full"]] = None,
    overflow: Optional[Literal["fold", "crop", "ellipsis", "ignore"]] = None,
    no_wrap: Optional[bool] = None,
    emoji: Optional[bool] = None,
    markup: Optional[bool] = None,
    highlight: Optional[bool] = None,
    width: Optional[int] = None,
    height: Optional[int] = None,
    crop: bool = True,
    soft_wrap: bool = False,
    new_line_start: bool = False,
) -> None:
    """Print with Rich-style formatting.
    
    This is a drop-in replacement for rich.print().
    
    Args:
        *objects: Objects to print.
        sep: Separator between objects.
        end: String to append at end.
        file: File to write to.
        flush: Flush output.
        style: Style to apply.
        justify: Text justification.
        overflow: Overflow handling.
        no_wrap: Disable wrapping.
        emoji: Enable emoji.
        markup: Enable markup.
        highlight: Enable highlighting.
        width: Output width.
        height: Output height.
        crop: Crop output.
        soft_wrap: Soft wrap text.
        new_line_start: Start with newline.
    """
    if file is not None:
        # Use builtin print for file output
        builtins.print(*objects, sep=sep, end=end, file=file, flush=flush)
        return

    console = get_console()
    console.print(
        *objects,
        sep=sep,
        end=end,
        style=style,
        justify=justify,
        overflow=overflow,
        no_wrap=no_wrap,
        emoji=emoji,
        markup=markup,
        highlight=highlight,
        width=width,
        height=height,
        crop=crop,
        soft_wrap=soft_wrap,
        new_line_start=new_line_start,
    )

    if flush:
        import sys
        sys.stdout.flush()


__all__ = ["print", "get_console"]
