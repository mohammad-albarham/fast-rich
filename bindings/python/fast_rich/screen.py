"""Screen utilities - matches rich.screen API."""

from __future__ import annotations

from typing import Any, Optional

from fast_rich.control import Control


class Screen:
    """A renderable that fills the terminal screen.
    
    Matches rich.screen.Screen API.
    """

    def __init__(
        self,
        renderable: Any = "",
        *,
        style: Optional[str] = None,
        application_mode: bool = False,
    ) -> None:
        """Create Screen.
        
        Args:
            renderable: Content to display.
            style: Screen style.
            application_mode: Application mode.
        """
        self.renderable = renderable
        self.style = style
        self.application_mode = application_mode

    def __str__(self) -> str:
        """Render as string."""
        return str(self.renderable)

    def __rich_console__(self, console, options):
        """Rich console protocol."""
        yield Control.clear()
        yield Control.home()
        yield str(self.renderable)


__all__ = ["Screen"]
