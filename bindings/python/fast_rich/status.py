"""Status context manager - matches rich.status API."""

from __future__ import annotations

from typing import Any, Optional, Union

from fast_rich.console import Console
from fast_rich.spinner import Spinner
from fast_rich.text import Text


class Status:
    """Display a status message with a spinner.
    
    Matches rich.status.Status API.
    """

    def __init__(
        self,
        status: Union[str, Text],
        *,
        console: Optional[Console] = None,
        spinner: str = "dots",
        spinner_style: Optional[str] = "status.spinner",
        speed: float = 1.0,
        refresh_per_second: float = 12.5,
    ) -> None:
        """Create Status.
        
        Args:
            status: Status message.
            console: Console to use.
            spinner: Spinner name.
            spinner_style: Spinner style.
            speed: Animation speed.
            refresh_per_second: Refresh rate.
        """
        self.status = status
        self.console = console or Console()
        self.spinner_name = spinner
        self.spinner_style = spinner_style
        self.speed = speed
        self.refresh_per_second = refresh_per_second
        self._spinner = Spinner(spinner, speed=speed)
        self._started = False

    def start(self) -> None:
        """Start the status display."""
        self._started = True
        self._spinner = Spinner(self.spinner_name, text=self.status, speed=self.speed)
        self._render()

    def stop(self) -> None:
        """Stop the status display."""
        if self._started:
            self.console._file.write("\r\x1b[2K")  # Clear line
            self.console._file.flush()
            self._started = False

    def update(
        self,
        status: Optional[Union[str, Text]] = None,
        *,
        spinner: Optional[str] = None,
        spinner_style: Optional[str] = None,
        speed: Optional[float] = None,
    ) -> None:
        """Update the status.
        
        Args:
            status: New status message.
            spinner: New spinner name.
            spinner_style: New spinner style.
            speed: New animation speed.
        """
        if status is not None:
            self.status = status
        if spinner is not None:
            self.spinner_name = spinner
        if spinner_style is not None:
            self.spinner_style = spinner_style
        if speed is not None:
            self.speed = speed
        
        if self._started:
            self._spinner = Spinner(self.spinner_name, text=self.status, speed=self.speed)
            self._render()

    def _render(self) -> None:
        """Render the status line."""
        self._spinner.update()
        line = str(self._spinner)
        self.console._file.write(f"\r\x1b[2K{line}")
        self.console._file.flush()

    def __enter__(self) -> "Status":
        """Enter context."""
        self.start()
        return self

    def __exit__(self, exc_type: Any, exc_val: Any, exc_tb: Any) -> None:
        """Exit context."""
        self.stop()


__all__ = ["Status"]
