"""Live render support - matches rich.live_render API."""

from __future__ import annotations

from typing import Any, Optional

from fast_rich.console import Console
from fast_rich.control import Control


class LiveRender:
    """Renders a renderable that may change.
    
    Matches rich.live_render.LiveRender API.
    """

    def __init__(
        self,
        renderable: Any = "",
        *,
        style: Optional[str] = None,
    ) -> None:
        """Create LiveRender.
        
        Args:
            renderable: Content to render.
            style: Style to apply.
        """
        self._renderable = renderable
        self.style = style
        self._shape: tuple = (0, 0)

    @property
    def renderable(self) -> Any:
        """Get current renderable."""
        return self._renderable

    @renderable.setter
    def renderable(self, new_renderable: Any) -> None:
        """Set new renderable."""
        self._renderable = new_renderable

    def set_renderable(self, renderable: Any) -> None:
        """Set the renderable.
        
        Args:
            renderable: New renderable.
        """
        self._renderable = renderable

    @property
    def position_cursor(self) -> str:
        """Get cursor positioning control."""
        height = self._shape[1]
        if height > 0:
            return f"\x1b[{height}A"
        return ""

    def __str__(self) -> str:
        """Render as string."""
        return str(self._renderable)

    def __rich_console__(self, console, options):
        """Rich console protocol."""
        # Move cursor up if needed
        if self._shape[1] > 0:
            yield Control.cursor_up(self._shape[1])
        yield str(self._renderable)


__all__ = ["LiveRender"]
