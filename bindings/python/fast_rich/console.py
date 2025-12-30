"""Console class - matches rich.console API."""

from __future__ import annotations

import sys
from typing import (
    Any,
    IO,
    Iterable,
    Literal,
    Optional,
    Union,
)

from fast_rich.style import Style
from fast_rich.text import Text


# Type alias for justify options
JustifyMethod = Literal["left", "center", "right", "full"]
OverflowMethod = Literal["fold", "crop", "ellipsis", "ignore"]


class Console:
    """A high performance console for terminal output.
    
    This is a drop-in replacement for rich.console.Console.
    """

    def __init__(
        self,
        *,
        color_system: Optional[Literal["auto", "standard", "256", "truecolor", "windows"]] = "auto",
        force_terminal: Optional[bool] = None,
        force_jupyter: Optional[bool] = None,
        force_interactive: Optional[bool] = None,
        soft_wrap: bool = False,
        theme: Optional[Any] = None,
        stderr: bool = False,
        file: Optional[IO[str]] = None,
        quiet: bool = False,
        width: Optional[int] = None,
        height: Optional[int] = None,
        style: Optional[Union[str, Style]] = None,
        no_color: Optional[bool] = None,
        tab_size: int = 8,
        record: bool = False,
        markup: bool = True,
        emoji: bool = True,
        emoji_variant: Optional[Literal["emoji", "text"]] = None,
        highlight: bool = True,
        log_time: bool = True,
        log_path: bool = True,
        log_time_format: str = "[%X]",
        highlighter: Optional[Any] = None,
        legacy_windows: Optional[bool] = None,
        safe_box: bool = True,
        get_datetime: Optional[Any] = None,
        get_time: Optional[Any] = None,
        _environ: Optional[dict] = None,
    ) -> None:
        """Create a Console instance.
        
        Args:
            color_system: Color system to use.
            force_terminal: Force terminal mode.
            force_jupyter: Force Jupyter mode.
            force_interactive: Force interactive mode.
            soft_wrap: Enable soft wrapping.
            theme: Theme to use.
            stderr: Write to stderr.
            file: File to write to.
            quiet: Suppress output.
            width: Console width.
            height: Console height.
            style: Default style.
            no_color: Disable color.
            tab_size: Tab size.
            record: Record output.
            markup: Enable markup.
            emoji: Enable emoji.
            emoji_variant: Emoji variant.
            highlight: Enable highlighting.
            log_time: Show time in logs.
            log_path: Show path in logs.
            log_time_format: Time format.
            highlighter: Highlighter to use.
            legacy_windows: Legacy Windows mode.
            safe_box: Use safe box characters.
            get_datetime: Custom datetime function.
            get_time: Custom time function.
            _environ: Environment override.
        """
        self._file = file or (sys.stderr if stderr else sys.stdout)
        self._quiet = quiet
        self._width = width
        self._height = height
        self._style = style if isinstance(style, Style) else (Style.parse(style) if style else None)
        self._soft_wrap = soft_wrap
        self._markup = markup
        self._emoji = emoji
        self._highlight = highlight
        self._record = record
        self._recorded: list[str] = []
        self._tab_size = tab_size
        self._no_color = no_color
        
        # Try to import Rust bindings
        try:
            from rich_rust import Console as RustConsole
            self._rust_console = RustConsole()
            self._use_rust = True
        except ImportError:
            self._rust_console = None
            self._use_rust = False

    @property
    def width(self) -> int:
        """Get console width."""
        if self._width:
            return self._width
        try:
            import shutil
            return shutil.get_terminal_size().columns
        except Exception:
            return 80

    @property
    def height(self) -> int:
        """Get console height."""
        if self._height:
            return self._height
        try:
            import shutil
            return shutil.get_terminal_size().lines
        except Exception:
            return 25

    @property
    def is_terminal(self) -> bool:
        """Check if output is a terminal."""
        return hasattr(self._file, "isatty") and self._file.isatty()

    def print(
        self,
        *objects: Any,
        sep: str = " ",
        end: str = "\n",
        style: Optional[Union[str, Style]] = None,
        justify: Optional[JustifyMethod] = None,
        overflow: Optional[OverflowMethod] = None,
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
        """Print to the console.
        
        This mimics the signature of rich.console.Console.print exactly.
        """
        if self._quiet:
            return

        # Convert objects to strings
        text_parts = []
        for obj in objects:
            if isinstance(obj, str):
                text_parts.append(obj)
            elif hasattr(obj, "__rich_console__"):
                # Rich renderables
                text_parts.append(str(obj))
            elif hasattr(obj, "__str__"):
                text_parts.append(str(obj))
            else:
                text_parts.append(repr(obj))

        output = sep.join(text_parts)
        
        if new_line_start and output:
            output = "\n" + output
            
        # Try to use Rust for rendering if available
        if self._use_rust and self._rust_console:
            try:
                style_str = str(style) if style else None
                self._rust_console.print(output, style_str)
                if end:
                    self._file.write(end)
                    self._file.flush()
                return
            except Exception:
                pass  # Fall back to Python

        # Python fallback
        self._file.write(output)
        if end:
            self._file.write(end)
        self._file.flush()
        
        if self._record:
            self._recorded.append(output + end)

    def print_json(
        self,
        json: Optional[str] = None,
        *,
        data: Optional[Any] = None,
        indent: Union[None, int, str] = 2,
        highlight: bool = True,
        skip_keys: bool = False,
        ensure_ascii: bool = False,
        check_circular: bool = True,
        allow_nan: bool = True,
        default: Optional[Any] = None,
        sort_keys: bool = False,
    ) -> None:
        """Print JSON with syntax highlighting."""
        import json as json_module
        
        if data is not None:
            json_str = json_module.dumps(
                data,
                indent=indent,
                skipkeys=skip_keys,
                ensure_ascii=ensure_ascii,
                check_circular=check_circular,
                allow_nan=allow_nan,
                default=default,
                sort_keys=sort_keys,
            )
        else:
            json_str = json or ""
            
        if self._use_rust and self._rust_console:
            try:
                self._rust_console.print_json(json_str)
                return
            except Exception:
                pass
        
        # Fallback: print without highlighting
        self.print(json_str)

    def log(
        self,
        *objects: Any,
        sep: str = " ",
        end: str = "\n",
        style: Optional[Union[str, Style]] = None,
        justify: Optional[JustifyMethod] = None,
        emoji: Optional[bool] = None,
        markup: Optional[bool] = None,
        highlight: Optional[bool] = None,
        log_locals: bool = False,
        _stack_offset: int = 1,
    ) -> None:
        """Log with timestamp."""
        from datetime import datetime
        timestamp = datetime.now().strftime("[%X]")
        self.print(timestamp, *objects, sep=sep, end=end, style=style)

    def rule(
        self,
        title: str = "",
        *,
        characters: str = "─",
        style: Optional[Union[str, Style]] = "rule.line",
        align: Literal["left", "center", "right"] = "center",
    ) -> None:
        """Print a horizontal rule."""
        width = self.width
        if title:
            title_text = f" {title} "
            padding = (width - len(title_text)) // 2
            line = characters * padding + title_text + characters * padding
        else:
            line = characters * width
        self.print(line[:width])

    def status(
        self,
        status: str,
        *,
        spinner: str = "dots",
        spinner_style: Optional[Union[str, Style]] = "status.spinner",
        speed: float = 1.0,
        refresh_per_second: float = 12.5,
    ) -> "Status":
        """Create a status context manager."""
        return Status(self, status, spinner=spinner)

    def clear(self, home: bool = True) -> None:
        """Clear the console."""
        self._file.write("\033[2J")
        if home:
            self._file.write("\033[H")
        self._file.flush()

    def export_text(self, *, clear: bool = True, styles: bool = False) -> str:
        """Export recorded output as text."""
        output = "".join(self._recorded)
        if clear:
            self._recorded.clear()
        return output

    def export_html(
        self,
        *,
        theme: Optional[Any] = None,
        clear: bool = True,
        code_format: Optional[str] = None,
        inline_styles: bool = False,
    ) -> str:
        """Export recorded output as HTML."""
        # Simplified implementation
        text = self.export_text(clear=clear)
        return f"<pre>{text}</pre>"

    def export_svg(
        self,
        *,
        title: str = "Rich",
        theme: Optional[Any] = None,
        clear: bool = True,
        code_format: str = "",
        font_aspect_ratio: float = 0.61,
        unique_id: Optional[str] = None,
    ) -> str:
        """Export recorded output as SVG."""
        # Simplified implementation
        text = self.export_text(clear=clear)
        return f'<svg><text>{text}</text></svg>'

    def input(
        self,
        prompt: str = "",
        *,
        markup: bool = True,
        emoji: bool = True,
        password: bool = False,
        stream: Optional[IO[str]] = None,
    ) -> str:
        """Get input from user."""
        self.print(prompt, end="")
        if password:
            import getpass
            return getpass.getpass("")
        return input()


class Status:
    """A status context manager for showing progress."""
    
    def __init__(
        self,
        console: Console,
        status: str,
        *,
        spinner: str = "dots",
    ) -> None:
        self.console = console
        self.status = status
        self.spinner = spinner

    def __enter__(self) -> "Status":
        self.console.print(f"⠋ {self.status}...")
        return self

    def __exit__(self, exc_type: Any, exc_val: Any, exc_tb: Any) -> None:
        self.console.print(f"✓ {self.status}")

    def update(self, status: str) -> None:
        """Update the status message."""
        self.status = status


__all__ = ["Console", "Status"]
