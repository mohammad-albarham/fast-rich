"""Repr utilities - matches rich.repr API."""

from __future__ import annotations

from typing import Any, Callable, Iterable, Optional, Type, TypeVar


T = TypeVar("T")


def auto(cls: Type[T]) -> Type[T]:
    """Class decorator for auto-generating __rich_repr__.
    
    Matches rich.repr.auto decorator.
    
    Args:
        cls: Class to decorate.
        
    Returns:
        Decorated class.
    """
    if not hasattr(cls, "__rich_repr__"):
        def __rich_repr__(self) -> Iterable:
            for name, value in vars(self).items():
                if not name.startswith("_"):
                    yield name, value
        cls.__rich_repr__ = __rich_repr__
    return cls


class Result:
    """A result of a repr computation."""

    def __init__(
        self,
        name: str,
        value: Any = ...,
        default: Any = ...,
    ) -> None:
        self.name = name
        self.value = value
        self.default = default


def rich_repr(cls: Type[T]) -> Type[T]:
    """Alias for auto decorator."""
    return auto(cls)


__all__ = ["auto", "rich_repr", "Result"]
