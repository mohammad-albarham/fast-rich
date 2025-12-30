"""Measure utilities - matches rich.measure API."""

from __future__ import annotations

from typing import Any, NamedTuple, Optional, TYPE_CHECKING

if TYPE_CHECKING:
    from fast_rich.console import Console


class Measurement(NamedTuple):
    """A measurement of renderable width.
    
    Matches rich.measure.Measurement API.
    """

    minimum: int
    maximum: int

    @classmethod
    def get(
        cls,
        console: "Console",
        options: Any,
        renderable: Any,
    ) -> "Measurement":
        """Get measurement for a renderable.
        
        Args:
            console: Console instance.
            options: Console options.
            renderable: Object to measure.
            
        Returns:
            Measurement instance.
        """
        if hasattr(renderable, "__rich_measure__"):
            measurement = renderable.__rich_measure__(console, options)
            if measurement is not None:
                return measurement
        
        text = str(renderable)
        lines = text.split("\n")
        if not lines:
            return cls(0, 0)
        
        widths = [len(line) for line in lines]
        return cls(min(widths), max(widths))

    def clamp(
        self,
        min_width: Optional[int] = None,
        max_width: Optional[int] = None,
    ) -> "Measurement":
        """Clamp measurement to given bounds.
        
        Args:
            min_width: Minimum width.
            max_width: Maximum width.
            
        Returns:
            Clamped measurement.
        """
        minimum = self.minimum
        maximum = self.maximum
        
        if min_width is not None:
            minimum = max(minimum, min_width)
        if max_width is not None:
            maximum = min(maximum, max_width)
        
        return Measurement(minimum, maximum)

    @classmethod
    def span(cls, minimum: int, maximum: int) -> "Measurement":
        """Create measurement with span."""
        return cls(minimum, maximum)


def measure_renderables(
    console: "Console",
    options: Any,
    renderables: Any,
) -> Measurement:
    """Measure multiple renderables.
    
    Args:
        console: Console instance.
        options: Console options.
        renderables: Objects to measure.
        
    Returns:
        Combined measurement.
    """
    min_width = 0
    max_width = 0
    
    for renderable in renderables:
        measurement = Measurement.get(console, options, renderable)
        min_width = max(min_width, measurement.minimum)
        max_width = max(max_width, measurement.maximum)
    
    return Measurement(min_width, max_width)


__all__ = ["Measurement", "measure_renderables"]
