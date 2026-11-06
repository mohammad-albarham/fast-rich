"""Abstract base classes - matches rich.abc API."""

from __future__ import annotations

from abc import ABC, abstractmethod
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from fast_rich.console import Console
    from fast_rich.console_options import ConsoleOptions


class RichRenderable(ABC):
    """Abstract base class for Rich renderables.
    
    Matches rich.abc.RichRenderable.
    """

    @abstractmethod
    def __rich_console__(
        self,
        console: "Console",
        options: "ConsoleOptions",
    ):
        """Required method for Rich console protocol.
        
        Args:
            console: Console instance.
            options: Console options.
            
        Yields:
            Renderables or strings.
        """
        pass


__all__ = ["RichRenderable"]
