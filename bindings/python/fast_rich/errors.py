"""Error classes - matches rich.errors API."""

from __future__ import annotations


class ConsoleError(Exception):
    """Base class for console errors."""
    pass


class StyleError(Exception):
    """Error raised for invalid styles."""
    pass


class StyleSyntaxError(StyleError):
    """Error raised for style syntax errors."""
    pass


class MissingStyle(StyleError):
    """Error raised for missing styles."""
    pass


class MarkupError(Exception):
    """Error raised for markup errors."""
    pass


class LiveError(Exception):
    """Error raised for live display errors."""
    pass


class NoAltScreen(Exception):
    """Error raised when alt screen is not available."""
    pass


class NotRenderableError(Exception):
    """Error raised when object is not renderable."""
    pass


__all__ = [
    "ConsoleError",
    "StyleError",
    "StyleSyntaxError",
    "MissingStyle",
    "MarkupError",
    "LiveError",
    "NoAltScreen",
    "NotRenderableError",
]
