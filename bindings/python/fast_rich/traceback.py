"""Traceback rendering - matches rich.traceback API."""

from __future__ import annotations

import sys
import traceback
from types import TracebackType
from typing import Any, Callable, Optional, Type, Union

from fast_rich.console import Console
from fast_rich.style import Style


class Traceback:
    """A traceback rendered with syntax highlighting.
    
    Matches rich.traceback.Traceback API.
    """

    def __init__(
        self,
        trace: Optional[TracebackType] = None,
        *,
        width: Optional[int] = None,
        extra_lines: int = 3,
        theme: Optional[str] = None,
        word_wrap: bool = False,
        show_locals: bool = False,
        locals_max_length: int = 10,
        locals_max_string: int = 80,
        locals_hide_dunder: bool = True,
        locals_hide_sunder: bool = False,
        indent_guides: bool = True,
        suppress: tuple = (),
        max_frames: int = 100,
    ) -> None:
        """Create Traceback.
        
        Args:
            trace: Traceback object.
            width: Display width.
            extra_lines: Extra context lines.
            theme: Syntax theme.
            word_wrap: Wrap long lines.
            show_locals: Show local variables.
            locals_max_length: Max collection length.
            locals_max_string: Max string length.
            locals_hide_dunder: Hide __vars__.
            locals_hide_sunder: Hide _vars.
            indent_guides: Show indent guides.
            suppress: Modules to suppress.
            max_frames: Max frames to show.
        """
        self.trace = trace
        self.width = width
        self.extra_lines = extra_lines
        self.theme = theme
        self.word_wrap = word_wrap
        self.show_locals = show_locals
        self.locals_max_length = locals_max_length
        self.locals_max_string = locals_max_string
        self.locals_hide_dunder = locals_hide_dunder
        self.locals_hide_sunder = locals_hide_sunder
        self.indent_guides = indent_guides
        self.suppress = suppress
        self.max_frames = max_frames

    @classmethod
    def from_exception(
        cls,
        exc_type: Type[BaseException],
        exc_value: BaseException,
        exc_tb: Optional[TracebackType],
        *,
        width: Optional[int] = None,
        extra_lines: int = 3,
        theme: Optional[str] = None,
        word_wrap: bool = False,
        show_locals: bool = False,
        locals_max_length: int = 10,
        locals_max_string: int = 80,
        locals_hide_dunder: bool = True,
        locals_hide_sunder: bool = False,
        indent_guides: bool = True,
        suppress: tuple = (),
        max_frames: int = 100,
    ) -> "Traceback":
        """Create from exception info."""
        return cls(
            trace=exc_tb,
            width=width,
            extra_lines=extra_lines,
            theme=theme,
            word_wrap=word_wrap,
            show_locals=show_locals,
            locals_max_length=locals_max_length,
            locals_max_string=locals_max_string,
            locals_hide_dunder=locals_hide_dunder,
            locals_hide_sunder=locals_hide_sunder,
            indent_guides=indent_guides,
            suppress=suppress,
            max_frames=max_frames,
        )

    def __str__(self) -> str:
        """Render traceback as string."""
        if self.trace:
            lines = traceback.format_tb(self.trace)
            return "".join(lines)
        return ""

    def __rich_console__(self, console, options):
        """Rich console protocol."""
        yield str(self)


_installed = False
_original_excepthook: Optional[Callable] = None


def install(
    *,
    console: Optional[Console] = None,
    width: Optional[int] = None,
    extra_lines: int = 3,
    theme: Optional[str] = None,
    word_wrap: bool = False,
    show_locals: bool = False,
    locals_max_length: int = 10,
    locals_max_string: int = 80,
    locals_hide_dunder: bool = True,
    locals_hide_sunder: bool = False,
    indent_guides: bool = True,
    suppress: tuple = (),
    max_frames: int = 100,
) -> Callable:
    """Install a Rich traceback handler.
    
    Returns:
        The original exception hook.
    """
    global _installed, _original_excepthook
    
    if _installed:
        return _original_excepthook or sys.excepthook
    
    _original_excepthook = sys.excepthook
    _installed = True
    
    _console = console or Console(stderr=True)
    
    def excepthook(
        exc_type: Type[BaseException],
        exc_value: BaseException,
        exc_tb: Optional[TracebackType],
    ) -> None:
        tb = Traceback.from_exception(
            exc_type,
            exc_value,
            exc_tb,
            width=width,
            extra_lines=extra_lines,
            theme=theme,
            word_wrap=word_wrap,
            show_locals=show_locals,
            locals_max_length=locals_max_length,
            locals_max_string=locals_max_string,
            locals_hide_dunder=locals_hide_dunder,
            locals_hide_sunder=locals_hide_sunder,
            indent_guides=indent_guides,
            suppress=suppress,
            max_frames=max_frames,
        )
        _console.print(tb)
    
    sys.excepthook = excepthook
    return _original_excepthook


__all__ = ["Traceback", "install"]
