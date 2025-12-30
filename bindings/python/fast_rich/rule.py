"""Rule class - matches rich.rule API."""

from __future__ import annotations

from typing import Optional, Union

from fast_rich.style import Style


class Rule:
    """A horizontal rule line.
    
    Matches rich.rule.Rule API.
    """

    def __init__(
        self,
        title: str = "",
        *,
        characters: str = "─",
        style: Optional[Union[str, Style]] = "rule.line",
        end: str = "\n",
        align: str = "center",
    ) -> None:
        """Create a Rule.
        
        Args:
            title: Optional title text.
            characters: Character(s) for the line.
            style: Style for the rule.
            end: String to append.
            align: Title alignment ('left', 'center', 'right').
        """
        self.title = title
        self.characters = characters
        self.style = style
        self.end = end
        self.align = align

    def __str__(self) -> str:
        """Render rule as string."""
        import shutil
        try:
            width = shutil.get_terminal_size().columns
        except Exception:
            width = 80

        if self.title:
            title_text = f" {self.title} "
            if self.align == "left":
                padding_left = 2
                padding_right = width - len(title_text) - padding_left
            elif self.align == "right":
                padding_right = 2
                padding_left = width - len(title_text) - padding_right
            else:  # center
                padding_left = (width - len(title_text)) // 2
                padding_right = width - len(title_text) - padding_left
            
            line = (self.characters * padding_left)[:padding_left]
            line += title_text
            line += (self.characters * padding_right)[:padding_right]
        else:
            line = (self.characters * width)[:width]

        return line

    def __rich_console__(self, console, options):
        """Rich console protocol."""
        yield str(self)


__all__ = ["Rule"]
