"""Constrain renderable - matches rich.constrain API."""

from __future__ import annotations

from typing import Any, Optional


class Constrain:
    """Constrain the width of a renderable.
    
    Matches rich.constrain.Constrain API.
    """

    def __init__(
        self,
        renderable: Any,
        width: Optional[int] = None,
    ) -> None:
        """Create Constrain.
        
        Args:
            renderable: Content to constrain.
            width: Maximum width.
        """
        self.renderable = renderable
        self.width = width

    def __str__(self) -> str:
        """Render as string."""
        content = str(self.renderable)
        if self.width is None:
            return content
        
        lines = []
        for line in content.split("\n"):
            if len(line) > self.width:
                lines.append(line[:self.width])
            else:
                lines.append(line)
        return "\n".join(lines)

    def __rich_console__(self, console, options):
        """Rich console protocol."""
        yield str(self)


__all__ = ["Constrain"]
