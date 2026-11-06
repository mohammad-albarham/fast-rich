"""Container renderables - matches rich.containers API."""

from __future__ import annotations

from typing import Any, Iterable, Iterator, List, Optional, Union

from fast_rich.style import Style


class Lines:
    """A list of lines that can be rendered.
    
    Matches rich.containers.Lines API.
    """

    def __init__(self, lines: Optional[Iterable[Any]] = None) -> None:
        """Create Lines.
        
        Args:
            lines: Initial lines.
        """
        self._lines: List[Any] = list(lines) if lines else []

    def __iter__(self) -> Iterator[Any]:
        return iter(self._lines)

    def __len__(self) -> int:
        return len(self._lines)

    def __getitem__(self, index: int) -> Any:
        return self._lines[index]

    def append(self, line: Any) -> None:
        """Append a line."""
        self._lines.append(line)

    def extend(self, lines: Iterable[Any]) -> None:
        """Extend with lines."""
        self._lines.extend(lines)

    def pop(self, index: int = -1) -> Any:
        """Pop a line."""
        return self._lines.pop(index)

    def justify(
        self,
        console: Any,
        width: int,
        justify: str = "left",
        overflow: str = "fold",
    ) -> None:
        """Justify lines to width."""
        pass  # Simplified

    def __rich_console__(self, console, options):
        for line in self._lines:
            yield str(line)


class Renderables:
    """A collection of renderables.
    
    Matches rich.containers.Renderables API.
    """

    def __init__(self, renderables: Optional[Iterable[Any]] = None) -> None:
        """Create Renderables.
        
        Args:
            renderables: Initial renderables.
        """
        self._renderables: List[Any] = list(renderables) if renderables else []

    def __iter__(self) -> Iterator[Any]:
        return iter(self._renderables)

    def __len__(self) -> int:
        return len(self._renderables)

    def append(self, renderable: Any) -> None:
        """Append a renderable."""
        self._renderables.append(renderable)

    def __rich_console__(self, console, options):
        for renderable in self._renderables:
            yield str(renderable)


class Group:
    """A group of renderables.
    
    Matches rich.console.Group API.
    """

    def __init__(
        self,
        *renderables: Any,
        fit: bool = True,
    ) -> None:
        """Create Group.
        
        Args:
            *renderables: Renderables to group.
            fit: Fit to content.
        """
        self._renderables = list(renderables)
        self.fit = fit

    def __iter__(self) -> Iterator[Any]:
        return iter(self._renderables)

    def __rich_console__(self, console, options):
        for renderable in self._renderables:
            yield str(renderable)


def group(fit: bool = True):
    """Decorator to create a Group from a generator.
    
    Args:
        fit: Fit to content.
        
    Returns:
        Decorator.
    """
    def decorator(func):
        def wrapper(*args, **kwargs):
            return Group(*func(*args, **kwargs), fit=fit)
        return wrapper
    return decorator


__all__ = ["Lines", "Renderables", "Group", "group"]
