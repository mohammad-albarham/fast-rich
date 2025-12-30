"""Layout for splitting terminal - matches rich.layout API."""

from __future__ import annotations

from typing import List, Optional, Union

from fast_rich.console import Console
from fast_rich.style import Style


class Layout:
    """A layout splits the terminal into regions.
    
    Matches rich.layout.Layout API.
    """

    def __init__(
        self,
        renderable: Optional[Union[str, "Layout"]] = None,
        *,
        name: Optional[str] = None,
        size: Optional[int] = None,
        minimum_size: int = 1,
        ratio: int = 1,
        visible: bool = True,
    ) -> None:
        """Create Layout.
        
        Args:
            renderable: Content to display.
            name: Layout name.
            size: Fixed size.
            minimum_size: Minimum size.
            ratio: Size ratio.
            visible: Visibility.
        """
        self.renderable = renderable
        self.name = name
        self.size = size
        self.minimum_size = minimum_size
        self.ratio = ratio
        self.visible = visible
        self._children: List[Layout] = []
        self._split_direction: Optional[str] = None

    def split(
        self,
        *layouts: "Layout",
        direction: str = "column",
    ) -> None:
        """Split into multiple layouts.
        
        Args:
            *layouts: Child layouts.
            direction: 'column' or 'row'.
        """
        self._children = list(layouts)
        self._split_direction = direction

    def split_column(self, *layouts: "Layout") -> None:
        """Split into columns."""
        self.split(*layouts, direction="column")

    def split_row(self, *layouts: "Layout") -> None:
        """Split into rows."""
        self.split(*layouts, direction="row")

    def add_split(self, layout: "Layout") -> None:
        """Add a child layout."""
        self._children.append(layout)

    def update(self, renderable: Union[str, "Layout"]) -> None:
        """Update the content."""
        self.renderable = renderable

    def unsplit(self) -> None:
        """Remove splits."""
        self._children.clear()
        self._split_direction = None

    def __getitem__(self, name: str) -> "Layout":
        """Get child by name."""
        for child in self._children:
            if child.name == name:
                return child
            result = child[name] if child._children else None
            if result:
                return result
        raise KeyError(f"Layout {name!r} not found")

    def __str__(self) -> str:
        """Render layout as string."""
        if self._children:
            parts = [str(child) for child in self._children if child.visible]
            separator = "\n" if self._split_direction == "column" else " │ "
            return separator.join(parts)
        return str(self.renderable) if self.renderable else ""

    def __rich_console__(self, console, options):
        """Rich console protocol."""
        yield str(self)


__all__ = ["Layout"]
