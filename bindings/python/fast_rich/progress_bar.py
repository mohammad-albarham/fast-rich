"""Progress bar widget - matches rich.progress_bar API."""

from __future__ import annotations

from typing import Optional, Union

from fast_rich.style import Style


class ProgressBar:
    """A progress bar widget.
    
    Matches rich.progress_bar.ProgressBar API.
    """

    def __init__(
        self,
        total: float = 100.0,
        completed: float = 0.0,
        *,
        width: Optional[int] = None,
        pulse: bool = False,
        style: Optional[Union[str, Style]] = "bar.back",
        complete_style: Optional[Union[str, Style]] = "bar.complete",
        finished_style: Optional[Union[str, Style]] = "bar.finished",
        pulse_style: Optional[Union[str, Style]] = "bar.pulse",
        animation_time: Optional[float] = None,
    ) -> None:
        """Create ProgressBar.
        
        Args:
            total: Total value.
            completed: Completed value.
            width: Bar width.
            pulse: Enable pulse animation.
            style: Background style.
            complete_style: Completed portion style.
            finished_style: Finished style.
            pulse_style: Pulse animation style.
            animation_time: Animation time.
        """
        self.total = total
        self.completed = completed
        self.width = width
        self.pulse = pulse
        self.style = style
        self.complete_style = complete_style
        self.finished_style = finished_style
        self.pulse_style = pulse_style
        self.animation_time = animation_time

    @property
    def percentage_completed(self) -> float:
        """Get percentage completed."""
        if self.total <= 0:
            return 0.0
        return min(100.0, max(0.0, (self.completed / self.total) * 100.0))

    def update(self, completed: float, total: Optional[float] = None) -> None:
        """Update progress.
        
        Args:
            completed: New completed value.
            total: New total value.
        """
        self.completed = completed
        if total is not None:
            self.total = total

    def __str__(self) -> str:
        """Render as string."""
        import shutil
        width = self.width or 40
        
        if self.total <= 0:
            percent = 0
        else:
            percent = self.completed / self.total
        
        filled = int(width * percent)
        empty = width - filled
        
        # Use block characters
        bar = "━" * filled + "╸" + "━" * max(0, empty - 1) if filled < width else "━" * width
        return bar[:width]

    def __repr__(self) -> str:
        return f"ProgressBar(completed={self.completed}, total={self.total})"

    def __rich_console__(self, console, options):
        """Rich console protocol."""
        from fast_rich.segment import Segment
        yield Segment(str(self))


__all__ = ["ProgressBar"]
