"""Color palette - matches rich.palette API."""

from __future__ import annotations

from typing import List, Tuple


class Palette:
    """A palette of colors.
    
    Matches rich.palette.Palette API.
    """

    def __init__(self, colors: List[Tuple[int, int, int]]) -> None:
        """Create Palette.
        
        Args:
            colors: List of RGB tuples.
        """
        self._colors = colors

    def __getitem__(self, index: int) -> Tuple[int, int, int]:
        """Get color by index."""
        return self._colors[index]

    def __len__(self) -> int:
        """Get number of colors."""
        return len(self._colors)

    def match(self, color: Tuple[int, int, int]) -> int:
        """Find closest matching color index.
        
        Args:
            color: RGB tuple to match.
            
        Returns:
            Index of closest color.
        """
        r, g, b = color
        min_distance = float("inf")
        best_match = 0
        
        for i, (pr, pg, pb) in enumerate(self._colors):
            # Calculate color distance
            distance = (r - pr) ** 2 + (g - pg) ** 2 + (b - pb) ** 2
            if distance < min_distance:
                min_distance = distance
                best_match = i
        
        return best_match


# Standard 16-color ANSI palette
STANDARD_PALETTE = Palette([
    (0, 0, 0),        # 0: Black
    (128, 0, 0),      # 1: Red
    (0, 128, 0),      # 2: Green
    (128, 128, 0),    # 3: Yellow
    (0, 0, 128),      # 4: Blue
    (128, 0, 128),    # 5: Magenta
    (0, 128, 128),    # 6: Cyan
    (192, 192, 192),  # 7: White
    (128, 128, 128),  # 8: Bright Black
    (255, 0, 0),      # 9: Bright Red
    (0, 255, 0),      # 10: Bright Green
    (255, 255, 0),    # 11: Bright Yellow
    (0, 0, 255),      # 12: Bright Blue
    (255, 0, 255),    # 13: Bright Magenta
    (0, 255, 255),    # 14: Bright Cyan
    (255, 255, 255),  # 15: Bright White
])


__all__ = ["Palette", "STANDARD_PALETTE"]
