"""Logging integration - matches rich.logging API."""

from __future__ import annotations

import logging
from datetime import datetime
from typing import Any, Callable, Iterable, List, Optional, Union

from fast_rich.console import Console
from fast_rich.text import Text


class RichHandler(logging.Handler):
    """A logging handler that uses Rich for output.
    
    Matches rich.logging.RichHandler API.
    """

    LEVEL_STYLES = {
        "DEBUG": "bright_blue",
        "INFO": "bright_green",
        "WARNING": "bright_yellow",
        "ERROR": "bright_red",
        "CRITICAL": "bold bright_red",
    }

    def __init__(
        self,
        level: int = logging.NOTSET,
        console: Optional[Console] = None,
        *,
        show_time: bool = True,
        show_level: bool = True,
        show_path: bool = True,
        enable_link_path: bool = True,
        highlighter: Optional[Any] = None,
        markup: bool = False,
        rich_tracebacks: bool = True,
        tracebacks_width: Optional[int] = None,
        tracebacks_extra_lines: int = 3,
        tracebacks_theme: Optional[str] = None,
        tracebacks_word_wrap: bool = True,
        tracebacks_show_locals: bool = False,
        tracebacks_suppress: Iterable[str] = (),
        locals_max_length: int = 10,
        locals_max_string: int = 80,
        log_time_format: Union[str, Callable[[datetime], Text]] = "[%X]",
        keywords: Optional[List[str]] = None,
    ) -> None:
        """Create RichHandler.
        
        Args:
            level: Logging level.
            console: Console to use.
            show_time: Show timestamp.
            show_level: Show log level.
            show_path: Show source path.
            enable_link_path: Make path a link.
            highlighter: Highlighter to use.
            markup: Enable Rich markup.
            rich_tracebacks: Use Rich tracebacks.
            tracebacks_width: Traceback width.
            tracebacks_extra_lines: Extra context lines.
            tracebacks_theme: Traceback theme.
            tracebacks_word_wrap: Wrap traceback.
            tracebacks_show_locals: Show local vars.
            tracebacks_suppress: Modules to suppress.
            locals_max_length: Max collection length.
            locals_max_string: Max string length.
            log_time_format: Time format.
            keywords: Keywords to highlight.
        """
        super().__init__(level)
        self.console = console or Console(stderr=True)
        self.show_time = show_time
        self.show_level = show_level
        self.show_path = show_path
        self.enable_link_path = enable_link_path
        self.highlighter = highlighter
        self.markup = markup
        self.rich_tracebacks = rich_tracebacks
        self.tracebacks_width = tracebacks_width
        self.tracebacks_extra_lines = tracebacks_extra_lines
        self.tracebacks_theme = tracebacks_theme
        self.tracebacks_word_wrap = tracebacks_word_wrap
        self.tracebacks_show_locals = tracebacks_show_locals
        self.tracebacks_suppress = tracebacks_suppress
        self.locals_max_length = locals_max_length
        self.locals_max_string = locals_max_string
        self.log_time_format = log_time_format
        self.keywords = keywords or []

    def emit(self, record: logging.LogRecord) -> None:
        """Emit a log record.
        
        Args:
            record: Log record to emit.
        """
        try:
            message = self.format(record)
            parts = []
            
            # Time
            if self.show_time:
                if callable(self.log_time_format):
                    time_text = self.log_time_format(datetime.fromtimestamp(record.created))
                else:
                    time_str = datetime.fromtimestamp(record.created).strftime(self.log_time_format)
                    time_text = Text(time_str, style="log.time")
                parts.append(str(time_text))
            
            # Level
            if self.show_level:
                level_style = self.LEVEL_STYLES.get(record.levelname, "")
                parts.append(f"[{level_style}]{record.levelname:8}[/]" if level_style else record.levelname)
            
            # Message
            parts.append(message)
            
            # Path
            if self.show_path:
                parts.append(f"[dim]{record.filename}:{record.lineno}[/dim]")
            
            output = " ".join(parts)
            self.console.print(output, markup=self.markup)
            
            # Traceback
            if record.exc_info and self.rich_tracebacks:
                from fast_rich.traceback import Traceback
                tb = Traceback.from_exception(*record.exc_info)
                self.console.print(tb)
                
        except Exception:
            self.handleError(record)


__all__ = ["RichHandler"]
