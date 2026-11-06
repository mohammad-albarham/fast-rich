"""Padding renderable - matches rich.padding API."""

from __future__ import annotations

from typing import Any, Optional, Tuple, Union

from fast_rich.style import Style


PaddingDimensions = Union[int, Tuple[int], Tuple[int, int], Tuple[int, int, int, int]]


class Padding:
    """Add padding around a renderable.
    
    Matches rich.padding.Padding API.
    """

    def __init__(
        self,
        renderable: Any,
        pad: PaddingDimensions = (0, 0, 0, 0),
        *,
        style: Optional[Union[str, Style]] = None,
        expand: bool = True,
    ) -> None:
        """Create Padding.
        
        Args:
            renderable: Content to pad.
            pad: Padding (top, right, bottom, left) or shorthand.
            style: Style to apply.
            expand: Expand to fill width.
        """
        self.renderable = renderable
        self.pad = self._normalize_padding(pad)
        self.style = style
        self.expand = expand

    @staticmethod
    def _normalize_padding(pad: PaddingDimensions) -> Tuple[int, int, int, int]:
        """Normalize padding to (top, right, bottom, left) tuple."""
        if isinstance(pad, int):
            return (pad, pad, pad, pad)
        elif len(pad) == 1:
            return (pad[0], pad[0], pad[0], pad[0])
        elif len(pad) == 2:
            return (pad[0], pad[1], pad[0], pad[1])
        elif len(pad) == 4:
            return tuple(pad)  # type: ignore
        else:
            return (0, 0, 0, 0)

    @classmethod
    def indent(
        cls,
        renderable: Any,
        level: int,
        *,
        style: Optional[Union[str, Style]] = None,
    ) -> "Padding":
        """Create padding with left indent."""
        return cls(renderable, (0, 0, 0, level * 2), style=style)

    def __str__(self) -> str:
        """Render as string."""
        top, right, bottom, left = self.pad
        
        content = str(self.renderable)
        lines = content.split("\n")
        
        # Add horizontal padding
        padded_lines = [" " * left + line + " " * right for line in lines]
        
        # Add vertical padding
        import shutil
        try:
            width = shutil.get_terminal_size().columns
        except Exception:
            width = 80
            
        blank_line = " " * width if self.expand else ""
        result_lines = []
        
        # Top padding
        for _ in range(top):
            result_lines.append(blank_line)
        
        # Content
        result_lines.extend(padded_lines)
        
        # Bottom padding
        for _ in range(bottom):
            result_lines.append(blank_line)
        
        return "\n".join(result_lines)

    def __rich_console__(self, console, options):
        """Rich console protocol."""
        yield str(self)


__all__ = ["Padding", "PaddingDimensions"]
