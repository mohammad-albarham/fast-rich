"""Bar renderable - matches rich.bar API."""

from __future__ import annotations

from typing import Optional, Union

from fast_rich.style import Style


class Bar:
    """A horizontal bar with customizable fill.
    
    Matches rich.bar.Bar API.
    """

    def __init__(
        self,
        size: float = 1.0,
        begin: float = 0.0,
        end: float = 1.0,
        *,
        width: Optional[int] = None,
        color: Optional[str] = "default",
        bgcolor: Optional[str] = None,
    ) -> None:
        """Create a Bar.
        
        Args:
            size: Size of the bar (0.0-1.0).
            begin: Start position (0.0-1.0).
            end: End position (0.0-1.0).
            width: Width of the bar.
            color: Bar color.
            bgcolor: Background color.
        """
        self.size = max(0.0, min(1.0, size))
        self.begin = max(0.0, min(1.0, begin))
        self.end = max(0.0, min(1.0, end))
        self.width = width
        self.color = color
        self.bgcolor = bgcolor

    def __str__(self) -> str:
        """Render as string."""
        import shutil
        width = self.width or shutil.get_terminal_size().columns
        
        filled_start = int(width * self.begin)
        filled_end = int(width * self.end * self.size)
        
        bar = " " * filled_start + "█" * (filled_end - filled_start) + " " * (width - filled_end)
        return bar[:width]

    def __repr__(self) -> str:
        return f"Bar(size={self.size}, begin={self.begin}, end={self.end})"

    def __rich_console__(self, console, options):
        """Rich console protocol."""
        from fast_rich.segment import Segment
        text = str(self)
        style = Style()
        if self.color:
            style = Style.parse(self.color)
        yield Segment(text, style)


__all__ = ["Bar"]
