"""Align renderable - matches rich.align API."""

from __future__ import annotations

from typing import Any, Optional, Union

from fast_rich.style import Style


class Align:
    """Align a renderable within available width.
    
    Matches rich.align.Align API.
    """

    def __init__(
        self,
        renderable: Any,
        align: str = "left",
        *,
        style: Optional[Union[str, Style]] = None,
        vertical: Optional[str] = None,
        pad: bool = True,
        width: Optional[int] = None,
        height: Optional[int] = None,
    ) -> None:
        """Create Align.
        
        Args:
            renderable: Content to align.
            align: Horizontal alignment (left, center, right).
            style: Style to apply.
            vertical: Vertical alignment (top, middle, bottom).
            pad: Pad to fill width.
            width: Target width.
            height: Target height.
        """
        self.renderable = renderable
        self.align = align
        self.style = style
        self.vertical = vertical
        self.pad = pad
        self.width = width
        self.height = height

    @classmethod
    def left(
        cls,
        renderable: Any,
        *,
        style: Optional[Union[str, Style]] = None,
        pad: bool = True,
        width: Optional[int] = None,
    ) -> "Align":
        """Left align."""
        return cls(renderable, "left", style=style, pad=pad, width=width)

    @classmethod
    def center(
        cls,
        renderable: Any,
        *,
        style: Optional[Union[str, Style]] = None,
        pad: bool = True,
        width: Optional[int] = None,
    ) -> "Align":
        """Center align."""
        return cls(renderable, "center", style=style, pad=pad, width=width)

    @classmethod
    def right(
        cls,
        renderable: Any,
        *,
        style: Optional[Union[str, Style]] = None,
        pad: bool = True,
        width: Optional[int] = None,
    ) -> "Align":
        """Right align."""
        return cls(renderable, "right", style=style, pad=pad, width=width)

    def __str__(self) -> str:
        """Render as string."""
        content = str(self.renderable)
        if self.width is None:
            import shutil
            try:
                width = shutil.get_terminal_size().columns
            except Exception:
                width = 80
        else:
            width = self.width

        lines = content.split("\n")
        result = []
        for line in lines:
            if self.align == "center":
                result.append(line.center(width))
            elif self.align == "right":
                result.append(line.rjust(width))
            else:
                result.append(line.ljust(width) if self.pad else line)
        
        return "\n".join(result)

    def __rich_console__(self, console, options):
        """Rich console protocol."""
        yield str(self)


class VerticalCenter:
    """Vertically center a renderable."""

    def __init__(
        self,
        renderable: Any,
        *,
        style: Optional[Union[str, Style]] = None,
    ) -> None:
        self.renderable = renderable
        self.style = style

    def __str__(self) -> str:
        return str(self.renderable)

    def __rich_console__(self, console, options):
        yield str(self)


__all__ = ["Align", "VerticalCenter"]
