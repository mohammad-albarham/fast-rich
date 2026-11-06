"""Color triplet - matches rich.color_triplet API."""

from __future__ import annotations

from typing import NamedTuple, Tuple


class ColorTriplet(NamedTuple):
    """An RGB color triplet.
    
    Matches rich.color_triplet.ColorTriplet API.
    """

    red: int
    green: int
    blue: int

    @property
    def hex(self) -> str:
        """Get hex representation."""
        return f"#{self.red:02x}{self.green:02x}{self.blue:02x}"

    @property
    def rgb(self) -> str:
        """Get RGB string."""
        return f"rgb({self.red},{self.green},{self.blue})"

    @property
    def normalized(self) -> Tuple[float, float, float]:
        """Get normalized RGB (0-1)."""
        return (self.red / 255, self.green / 255, self.blue / 255)

    def __rich_repr__(self):
        yield self.red
        yield self.green
        yield self.blue


__all__ = ["ColorTriplet"]
