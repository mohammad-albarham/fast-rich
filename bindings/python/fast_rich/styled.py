"""Styled text - matches rich.styled API."""

from __future__ import annotations

from typing import Any, Optional, Union

from fast_rich.style import Style


class Styled:
    """Apply a style to a renderable.
    
    Matches rich.styled.Styled API.
    """

    def __init__(
        self,
        renderable: Any,
        style: Optional[Union[str, Style]] = None,
    ) -> None:
        """Create Styled.
        
        Args:
            renderable: Content to style.
            style: Style to apply.
        """
        self.renderable = renderable
        self.style = style if isinstance(style, Style) else (Style.parse(style) if style else None)

    def __str__(self) -> str:
        """Render as string."""
        return str(self.renderable)

    def __rich_console__(self, console, options):
        """Rich console protocol."""
        from fast_rich.segment import Segment
        text = str(self.renderable)
        yield Segment(text, self.style)


__all__ = ["Styled"]
