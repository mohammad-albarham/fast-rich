"""Console options - matches rich.console.ConsoleOptions API."""

from __future__ import annotations

from typing import Any, NamedTuple, Optional

from fast_rich.style import Style
from fast_rich.theme import Theme


class ConsoleDimensions(NamedTuple):
    """Console dimensions."""
    width: int
    height: int


class ConsoleOptions:
    """Options for console rendering.
    
    Matches rich.console.ConsoleOptions API.
    """

    def __init__(
        self,
        *,
        size: Optional[ConsoleDimensions] = None,
        legacy_windows: bool = False,
        min_width: int = 1,
        max_width: int = 80,
        is_terminal: bool = False,
        encoding: str = "utf-8",
        max_height: int = 25,
        justify: Optional[str] = None,
        overflow: Optional[str] = None,
        no_wrap: bool = False,
        highlight: bool = False,
        markup: bool = False,
        height: Optional[int] = None,
    ) -> None:
        """Create ConsoleOptions.
        
        Args:
            size: Console size.
            legacy_windows: Legacy Windows mode.
            min_width: Minimum width.
            max_width: Maximum width.
            is_terminal: Is a terminal.
            encoding: Character encoding.
            max_height: Maximum height.
            justify: Text justification.
            overflow: Overflow handling.
            no_wrap: Disable wrapping.
            highlight: Enable highlighting.
            markup: Enable markup.
            height: Fixed height.
        """
        self.size = size or ConsoleDimensions(max_width, max_height)
        self.legacy_windows = legacy_windows
        self.min_width = min_width
        self.max_width = max_width
        self.is_terminal = is_terminal
        self.encoding = encoding
        self.max_height = max_height
        self.justify = justify
        self.overflow = overflow
        self.no_wrap = no_wrap
        self.highlight = highlight
        self.markup = markup
        self.height = height

    @property
    def ascii_only(self) -> bool:
        """Check if ASCII only."""
        return self.legacy_windows

    def update(
        self,
        *,
        width: Optional[int] = None,
        min_width: Optional[int] = None,
        max_width: Optional[int] = None,
        justify: Optional[str] = None,
        overflow: Optional[str] = None,
        no_wrap: Optional[bool] = None,
        highlight: Optional[bool] = None,
        markup: Optional[bool] = None,
        height: Optional[int] = None,
    ) -> "ConsoleOptions":
        """Return updated options.
        
        Args:
            width: New width.
            min_width: New min width.
            max_width: New max width.
            justify: New justification.
            overflow: New overflow.
            no_wrap: New no_wrap.
            highlight: New highlight.
            markup: New markup.
            height: New height.
            
        Returns:
            Updated options.
        """
        options = ConsoleOptions(
            size=self.size,
            legacy_windows=self.legacy_windows,
            min_width=min_width if min_width is not None else self.min_width,
            max_width=max_width if max_width is not None else (width or self.max_width),
            is_terminal=self.is_terminal,
            encoding=self.encoding,
            max_height=self.max_height,
            justify=justify if justify is not None else self.justify,
            overflow=overflow if overflow is not None else self.overflow,
            no_wrap=no_wrap if no_wrap is not None else self.no_wrap,
            highlight=highlight if highlight is not None else self.highlight,
            markup=markup if markup is not None else self.markup,
            height=height if height is not None else self.height,
        )
        return options

    def update_width(self, width: int) -> "ConsoleOptions":
        """Return options with new width."""
        return self.update(width=width)

    def update_height(self, height: int) -> "ConsoleOptions":
        """Return options with new height."""
        return self.update(height=height)


__all__ = ["ConsoleOptions", "ConsoleDimensions"]
