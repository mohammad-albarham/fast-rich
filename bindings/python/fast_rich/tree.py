"""Tree class - matches rich.tree API."""

from __future__ import annotations

from typing import Iterator, List, Optional, Union

from fast_rich.style import Style
from fast_rich.text import Text


class Tree:
    """A tree view.
    
    Matches rich.tree.Tree API.
    """

    GUIDE_CHARS = {
        "space": "    ",
        "continue": "│   ",
        "branch": "├── ",
        "last": "└── ",
    }

    def __init__(
        self,
        label: Union[str, Text],
        *,
        style: Optional[Union[str, Style]] = None,
        guide_style: Optional[Union[str, Style]] = None,
        expanded: bool = True,
        highlight: bool = False,
        hide_root: bool = False,
    ) -> None:
        """Create a Tree.
        
        Args:
            label: Root label.
            style: Style for labels.
            guide_style: Style for guide lines.
            expanded: Expand by default.
            highlight: Highlight labels.
            hide_root: Hide root node.
        """
        self.label = label if isinstance(label, Text) else Text(str(label))
        self.style = style
        self.guide_style = guide_style
        self.expanded = expanded
        self.highlight = highlight
        self.hide_root = hide_root
        self._children: List[Tree] = []

    def add(
        self,
        label: Union[str, Text, "Tree"],
        *,
        style: Optional[Union[str, Style]] = None,
        guide_style: Optional[Union[str, Style]] = None,
        expanded: bool = True,
        highlight: bool = False,
    ) -> "Tree":
        """Add a child node.
        
        Args:
            label: Child label or Tree.
            style: Style for label.
            guide_style: Style for guides.
            expanded: Expand node.
            highlight: Highlight label.
            
        Returns:
            The added Tree node.
        """
        if isinstance(label, Tree):
            child = label
        else:
            child = Tree(
                label,
                style=style or self.style,
                guide_style=guide_style or self.guide_style,
                expanded=expanded,
                highlight=highlight,
            )
        self._children.append(child)
        return child

    def __iter__(self) -> Iterator["Tree"]:
        """Iterate over children."""
        return iter(self._children)

    def __str__(self) -> str:
        """Render tree as string."""
        lines = []
        self._render(lines, "", True)
        return "\n".join(lines)

    def _render(
        self,
        lines: List[str],
        prefix: str,
        is_last: bool,
    ) -> None:
        """Render tree recursively."""
        if not self.hide_root:
            label_str = str(self.label)
            lines.append(f"{prefix}{label_str}")

        child_prefix = prefix
        if not self.hide_root:
            child_prefix = prefix + (
                self.GUIDE_CHARS["space"] if is_last else self.GUIDE_CHARS["continue"]
            )

        if self.expanded:
            for i, child in enumerate(self._children):
                is_last_child = i == len(self._children) - 1
                guide = self.GUIDE_CHARS["last" if is_last_child else "branch"]
                
                child_lines: List[str] = []
                child._render(
                    child_lines,
                    child_prefix + guide,
                    is_last_child,
                )
                
                for j, line in enumerate(child_lines):
                    if j == 0:
                        lines.append(f"{child_prefix}{guide}{str(child.label)}")
                    else:
                        lines.append(line)

    def __rich_console__(self, console, options):
        """Rich console protocol."""
        yield str(self)


__all__ = ["Tree"]
