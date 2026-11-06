"""Panel class - matches rich.panel API."""

from __future__ import annotations

from typing import Optional, Union

from fast_rich.box import Box, ROUNDED
from fast_rich.style import Style
from fast_rich.text import Text


class Panel:
    """A panel with a border around content.
    
    Matches rich.panel.Panel API.
    """

    def __init__(
        self,
        renderable: Union[str, Text, "Panel"],
        *,
        box: Box = ROUNDED,
        safe_box: Optional[bool] = None,
        expand: bool = True,
        style: Optional[Union[str, Style]] = None,
        border_style: Optional[Union[str, Style]] = None,
        width: Optional[int] = None,
        height: Optional[int] = None,
        padding: Union[int, tuple] = (0, 1),
        highlight: bool = False,
        title: Optional[str] = None,
        title_align: str = "center",
        subtitle: Optional[str] = None,
        subtitle_align: str = "center",
    ) -> None:
        """Create a Panel.
        
        Args:
            renderable: Content to display.
            box: Box style.
            safe_box: Use ASCII-safe box.
            expand: Expand to fill width.
            style: Panel style.
            border_style: Border style.
            width: Fixed width.
            height: Fixed height.
            padding: Content padding.
            highlight: Highlight content.
            title: Panel title.
            title_align: Title alignment.
            subtitle: Panel subtitle.
            subtitle_align: Subtitle alignment.
        """
        self.renderable = renderable
        self.box = box
        self.safe_box = safe_box
        self.expand = expand
        self.style = style
        self.border_style = border_style
        self.width = width
        self.height = height
        self.padding = padding
        self.highlight = highlight
        self.title = title
        self.title_align = title_align
        self.subtitle = subtitle
        self.subtitle_align = subtitle_align

    @classmethod
    def fit(
        cls,
        renderable: Union[str, Text],
        *,
        box: Box = ROUNDED,
        style: Optional[Union[str, Style]] = None,
        border_style: Optional[Union[str, Style]] = None,
        padding: Union[int, tuple] = (0, 1),
        title: Optional[str] = None,
        subtitle: Optional[str] = None,
    ) -> "Panel":
        """Create a panel that fits its content."""
        return cls(
            renderable,
            box=box,
            expand=False,
            style=style,
            border_style=border_style,
            padding=padding,
            title=title,
            subtitle=subtitle,
        )

    def __str__(self) -> str:
        """Render panel as string."""
        content = str(self.renderable)
        lines = content.split("\n")
        
        # Calculate width
        max_len = max(len(line) for line in lines) if lines else 0
        width = (self.width or max_len) + 4  # padding
        
        box = self.box
        result = []
        
        # Top border with optional title
        top = box.top_left + box.top * (width - 2) + box.top_right
        if self.title:
            title_text = f" {self.title} "
            pos = (width - len(title_text)) // 2
            top = top[:pos] + title_text + top[pos + len(title_text):]
        result.append(top)
        
        # Content lines
        for line in lines:
            padded = f" {line.ljust(width - 4)} "
            result.append(box.mid_left + padded + box.mid_right)
        
        # Bottom border with optional subtitle
        bottom = box.bottom_left + box.bottom * (width - 2) + box.bottom_right
        if self.subtitle:
            sub_text = f" {self.subtitle} "
            pos = (width - len(sub_text)) // 2
            bottom = bottom[:pos] + sub_text + bottom[pos + len(sub_text):]
        result.append(bottom)
        
        return "\n".join(result)

    def __rich_console__(self, console, options):
        """Rich console protocol."""
        yield str(self)


__all__ = ["Panel"]
