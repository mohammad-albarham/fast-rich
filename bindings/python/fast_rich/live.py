"""Live display - matches rich.live API."""

from __future__ import annotations

import time
from typing import Any, Optional, Union

from fast_rich.console import Console


class Live:
    """Live updating display.
    
    Matches rich.live.Live API.
    """

    def __init__(
        self,
        renderable: Optional[Any] = None,
        *,
        console: Optional[Console] = None,
        screen: bool = False,
        auto_refresh: bool = True,
        refresh_per_second: float = 4,
        transient: bool = False,
        redirect_stdout: bool = True,
        redirect_stderr: bool = True,
        vertical_overflow: str = "ellipsis",
        get_renderable: Optional[Any] = None,
    ) -> None:
        """Create Live display.
        
        Args:
            renderable: Content to display.
            console: Console to use.
            screen: Use alternate screen.
            auto_refresh: Auto refresh.
            refresh_per_second: Refresh rate.
            transient: Remove on exit.
            redirect_stdout: Redirect stdout.
            redirect_stderr: Redirect stderr.
            vertical_overflow: Overflow handling.
            get_renderable: Callback for content.
        """
        self.renderable = renderable
        self.console = console or Console()
        self.screen = screen
        self.auto_refresh = auto_refresh
        self.refresh_per_second = refresh_per_second
        self.transient = transient
        self.redirect_stdout = redirect_stdout
        self.redirect_stderr = redirect_stderr
        self.vertical_overflow = vertical_overflow
        self.get_renderable = get_renderable
        self._started = False

    def __enter__(self) -> "Live":
        """Enter context."""
        self.start()
        return self

    def __exit__(self, exc_type: Any, exc_val: Any, exc_tb: Any) -> None:
        """Exit context."""
        self.stop()

    def start(self, refresh: bool = False) -> None:
        """Start live display."""
        self._started = True
        if self.screen:
            self.console._file.write("\033[?1049h")  # Enter alt screen
        if refresh:
            self.refresh()

    def stop(self) -> None:
        """Stop live display."""
        self._started = False
        if self.screen:
            self.console._file.write("\033[?1049l")  # Exit alt screen
        if not self.transient:
            self.console.print()

    def update(self, renderable: Any, *, refresh: bool = False) -> None:
        """Update content.
        
        Args:
            renderable: New content.
            refresh: Force refresh.
        """
        self.renderable = renderable
        if refresh:
            self.refresh()

    def refresh(self) -> None:
        """Refresh the display."""
        if not self._started:
            return
            
        content = self.get_renderable() if self.get_renderable else self.renderable
        if content:
            # Move cursor up and clear
            self.console._file.write("\033[2K\r")
            self.console.print(content, end="")
            self.console._file.flush()


__all__ = ["Live"]
