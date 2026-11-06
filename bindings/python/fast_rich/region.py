"""Region class - matches rich.region API."""

from __future__ import annotations

from typing import NamedTuple


class Region(NamedTuple):
    """A rectangular region.
    
    Matches rich.region.Region API.
    """

    x: int
    y: int
    width: int
    height: int

    @property
    def right(self) -> int:
        """Get right edge."""
        return self.x + self.width

    @property
    def bottom(self) -> int:
        """Get bottom edge."""
        return self.y + self.height

    def contains(self, x: int, y: int) -> bool:
        """Check if point is in region.
        
        Args:
            x: X coordinate.
            y: Y coordinate.
            
        Returns:
            True if point is in region.
        """
        return self.x <= x < self.right and self.y <= y < self.bottom

    def overlaps(self, other: "Region") -> bool:
        """Check if regions overlap.
        
        Args:
            other: Other region.
            
        Returns:
            True if regions overlap.
        """
        return not (
            self.right <= other.x or
            other.right <= self.x or
            self.bottom <= other.y or
            other.bottom <= self.y
        )

    def intersection(self, other: "Region") -> "Region":
        """Get intersection of regions.
        
        Args:
            other: Other region.
            
        Returns:
            Intersection region.
        """
        x = max(self.x, other.x)
        y = max(self.y, other.y)
        right = min(self.right, other.right)
        bottom = min(self.bottom, other.bottom)
        
        if right > x and bottom > y:
            return Region(x, y, right - x, bottom - y)
        return Region(0, 0, 0, 0)


__all__ = ["Region"]
