"""Rich protocol - matches rich.protocol API."""

from __future__ import annotations

from typing import Any, Callable, Union


def is_renderable(obj: Any) -> bool:
    """Check if object is renderable.
    
    Args:
        obj: Object to check.
        
    Returns:
        True if renderable.
    """
    return (
        hasattr(obj, "__rich__")
        or hasattr(obj, "__rich_console__")
        or isinstance(obj, str)
    )


def rich_cast(obj: Any) -> Any:
    """Cast object to a Rich renderable.
    
    Args:
        obj: Object to cast.
        
    Returns:
        Renderable object.
    """
    if hasattr(obj, "__rich__"):
        return obj.__rich__()
    return obj


def rich_repr_result(*, angular: bool = False) -> Callable:
    """Decorator for rich repr results."""
    def decorator(func: Callable) -> Callable:
        func._rich_repr_angular = angular
        return func
    return decorator


class RichRenderable:
    """Base class for Rich renderables."""
    
    def __rich_console__(self, console: Any, options: Any) -> Any:
        """Override in subclass."""
        raise NotImplementedError


__all__ = ["is_renderable", "rich_cast", "rich_repr_result", "RichRenderable"]
