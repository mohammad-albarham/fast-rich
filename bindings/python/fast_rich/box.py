"""Box styles for tables and panels - matches rich.box API."""

from __future__ import annotations

from dataclasses import dataclass
from typing import Optional


@dataclass
class Box:
    """Defines a box style for borders.
    
    This matches the rich.box.Box interface.
    """
    
    # Top border
    top_left: str = "┌"
    top: str = "─"
    top_right: str = "┐"
    top_divider: str = "┬"
    
    # Header row
    head_left: str = "│"
    head_vertical: str = "│"
    head_right: str = "│"
    
    # Header/body divider
    head_row_left: str = "├"
    head_row_horizontal: str = "─"
    head_row_cross: str = "┼"
    head_row_right: str = "┤"
    
    # Body rows
    mid_left: str = "│"
    mid_vertical: str = "│"
    mid_right: str = "│"
    
    # Row dividers
    row_left: str = "├"
    row_horizontal: str = "─"
    row_cross: str = "┼"
    row_right: str = "┤"
    
    # Footer divider
    foot_row_left: str = "├"
    foot_row_horizontal: str = "─"
    foot_row_cross: str = "┼"
    foot_row_right: str = "┤"
    
    # Footer row
    foot_left: str = "│"
    foot_vertical: str = "│"
    foot_right: str = "│"
    
    # Bottom border
    bottom_left: str = "└"
    bottom: str = "─"
    bottom_right: str = "┘"
    bottom_divider: str = "┴"

    def get_top(self, widths: list[int]) -> str:
        """Get the top border string."""
        parts = [self.top * w for w in widths]
        return self.top_left + self.top_divider.join(parts) + self.top_right

    def get_row(self, widths: list[int]) -> str:
        """Get a row divider string."""
        parts = [self.row_horizontal * w for w in widths]
        return self.row_left + self.row_cross.join(parts) + self.row_right

    def get_bottom(self, widths: list[int]) -> str:
        """Get the bottom border string."""
        parts = [self.bottom * w for w in widths]
        return self.bottom_left + self.bottom_divider.join(parts) + self.bottom_right


# Standard box styles matching rich.box
ROUNDED = Box(
    top_left="╭", top="─", top_right="╮", top_divider="┬",
    head_left="│", head_vertical="│", head_right="│",
    head_row_left="├", head_row_horizontal="─", head_row_cross="┼", head_row_right="┤",
    mid_left="│", mid_vertical="│", mid_right="│",
    row_left="├", row_horizontal="─", row_cross="┼", row_right="┤",
    foot_row_left="├", foot_row_horizontal="─", foot_row_cross="┼", foot_row_right="┤",
    foot_left="│", foot_vertical="│", foot_right="│",
    bottom_left="╰", bottom="─", bottom_right="╯", bottom_divider="┴",
)

SQUARE = Box(
    top_left="┌", top="─", top_right="┐", top_divider="┬",
    head_left="│", head_vertical="│", head_right="│",
    head_row_left="├", head_row_horizontal="─", head_row_cross="┼", head_row_right="┤",
    mid_left="│", mid_vertical="│", mid_right="│",
    row_left="├", row_horizontal="─", row_cross="┼", row_right="┤",
    foot_row_left="├", foot_row_horizontal="─", foot_row_cross="┼", foot_row_right="┤",
    foot_left="│", foot_vertical="│", foot_right="│",
    bottom_left="└", bottom="─", bottom_right="┘", bottom_divider="┴",
)

MINIMAL = Box(
    top_left=" ", top=" ", top_right=" ", top_divider=" ",
    head_left=" ", head_vertical=" ", head_right=" ",
    head_row_left=" ", head_row_horizontal="─", head_row_cross=" ", head_row_right=" ",
    mid_left=" ", mid_vertical=" ", mid_right=" ",
    row_left=" ", row_horizontal=" ", row_cross=" ", row_right=" ",
    foot_row_left=" ", foot_row_horizontal="─", foot_row_cross=" ", foot_row_right=" ",
    foot_left=" ", foot_vertical=" ", foot_right=" ",
    bottom_left=" ", bottom=" ", bottom_right=" ", bottom_divider=" ",
)

HORIZONTALS = Box(
    top_left=" ", top="─", top_right=" ", top_divider=" ",
    head_left=" ", head_vertical=" ", head_right=" ",
    head_row_left=" ", head_row_horizontal="─", head_row_cross=" ", head_row_right=" ",
    mid_left=" ", mid_vertical=" ", mid_right=" ",
    row_left=" ", row_horizontal="─", row_cross=" ", row_right=" ",
    foot_row_left=" ", foot_row_horizontal="─", foot_row_cross=" ", foot_row_right=" ",
    foot_left=" ", foot_vertical=" ", foot_right=" ",
    bottom_left=" ", bottom="─", bottom_right=" ", bottom_divider=" ",
)

SIMPLE = Box(
    top_left=" ", top=" ", top_right=" ", top_divider=" ",
    head_left=" ", head_vertical=" ", head_right=" ",
    head_row_left=" ", head_row_horizontal="─", head_row_cross="─", head_row_right=" ",
    mid_left=" ", mid_vertical=" ", mid_right=" ",
    row_left=" ", row_horizontal=" ", row_cross=" ", row_right=" ",
    foot_row_left=" ", foot_row_horizontal="─", foot_row_cross="─", foot_row_right=" ",
    foot_left=" ", foot_vertical=" ", foot_right=" ",
    bottom_left=" ", bottom=" ", bottom_right=" ", bottom_divider=" ",
)

HEAVY = Box(
    top_left="┏", top="━", top_right="┓", top_divider="┳",
    head_left="┃", head_vertical="┃", head_right="┃",
    head_row_left="┣", head_row_horizontal="━", head_row_cross="╋", head_row_right="┫",
    mid_left="┃", mid_vertical="┃", mid_right="┃",
    row_left="┣", row_horizontal="━", row_cross="╋", row_right="┫",
    foot_row_left="┣", foot_row_horizontal="━", foot_row_cross="╋", foot_row_right="┫",
    foot_left="┃", foot_vertical="┃", foot_right="┃",
    bottom_left="┗", bottom="━", bottom_right="┛", bottom_divider="┻",
)

DOUBLE = Box(
    top_left="╔", top="═", top_right="╗", top_divider="╦",
    head_left="║", head_vertical="║", head_right="║",
    head_row_left="╠", head_row_horizontal="═", head_row_cross="╬", head_row_right="╣",
    mid_left="║", mid_vertical="║", mid_right="║",
    row_left="╠", row_horizontal="═", row_cross="╬", row_right="╣",
    foot_row_left="╠", foot_row_horizontal="═", foot_row_cross="╬", foot_row_right="╣",
    foot_left="║", foot_vertical="║", foot_right="║",
    bottom_left="╚", bottom="═", bottom_right="╝", bottom_divider="╩",
)

ASCII = Box(
    top_left="+", top="-", top_right="+", top_divider="+",
    head_left="|", head_vertical="|", head_right="|",
    head_row_left="+", head_row_horizontal="-", head_row_cross="+", head_row_right="+",
    mid_left="|", mid_vertical="|", mid_right="|",
    row_left="+", row_horizontal="-", row_cross="+", row_right="+",
    foot_row_left="+", foot_row_horizontal="-", foot_row_cross="+", foot_row_right="+",
    foot_left="|", foot_vertical="|", foot_right="|",
    bottom_left="+", bottom="-", bottom_right="+", bottom_divider="+",
)

__all__ = [
    "Box",
    "ROUNDED",
    "SQUARE",
    "MINIMAL",
    "HORIZONTALS",
    "SIMPLE",
    "HEAVY",
    "DOUBLE",
    "ASCII",
]
