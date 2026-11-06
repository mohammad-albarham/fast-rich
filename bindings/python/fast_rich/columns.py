"""Columns layout - matches rich.columns API."""

from __future__ import annotations

from typing import Iterable, Optional, Union

from fast_rich.style import Style


class Columns:
    """Display renderables in columns.
    
    Matches rich.columns.Columns API.
    """

    def __init__(
        self,
        renderables: Optional[Iterable] = None,
        *,
        padding: Union[int, tuple] = (0, 1),
        expand: bool = False,
        equal: bool = False,
        column_first: bool = False,
        right_to_left: bool = False,
        align: str = "left",
        width: Optional[int] = None,
        title: Optional[str] = None,
    ) -> None:
        """Create Columns.
        
        Args:
            renderables: Items to display.
            padding: Padding between columns.
            expand: Expand to fill width.
            equal: Make columns equal width.
            column_first: Fill columns first.
            right_to_left: Right to left order.
            align: Text alignment.
            width: Fixed column width.
            title: Optional title.
        """
        self.renderables = list(renderables) if renderables else []
        self.padding = padding
        self.expand = expand
        self.equal = equal
        self.column_first = column_first
        self.right_to_left = right_to_left
        self.align = align
        self.width = width
        self.title = title

    def add_renderable(self, renderable) -> None:
        """Add an item."""
        self.renderables.append(renderable)

    def __str__(self) -> str:
        """Render columns as string."""
        if not self.renderables:
            return ""
            
        items = [str(r) for r in self.renderables]
        
        if self.right_to_left:
            items = items[::-1]
            
        # Simple column layout
        import shutil
        try:
            term_width = shutil.get_terminal_size().columns
        except Exception:
            term_width = 80
            
        col_width = self.width or max(len(item) for item in items) + 2
        num_cols = max(1, term_width // col_width)
        
        lines = []
        for i in range(0, len(items), num_cols):
            row_items = items[i:i + num_cols]
            line = ""
            for item in row_items:
                if self.align == "center":
                    line += item.center(col_width)
                elif self.align == "right":
                    line += item.rjust(col_width)
                else:
                    line += item.ljust(col_width)
            lines.append(line.rstrip())
        
        return "\n".join(lines)

    def __rich_console__(self, console, options):
        """Rich console protocol."""
        yield str(self)


__all__ = ["Columns"]
