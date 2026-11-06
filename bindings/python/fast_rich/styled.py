# Adapted from Rich (https://github.com/Textualize/rich), Copyright (c) 2020
# Will McGugan, MIT License (see LICENSE.rich). fast_rich change: rendering
# and measuring are done by the Rust core (see _native.py).
from typing import TYPE_CHECKING

from .measure import Measurement
from .segment import Segment
from .style import StyleType

if TYPE_CHECKING:
    from .console import Console, ConsoleOptions, RenderResult, RenderableType


class Styled:
    """Apply a style to a renderable.

    Args:
        renderable (RenderableType): Any renderable.
        style (StyleType): A style to apply across the entire renderable.
    """

    def __init__(self, renderable: "RenderableType", style: "StyleType") -> None:
        self.renderable = renderable
        self.style = style


from . import _native  # noqa: E402

def _native_console(self, console, options):  # type: ignore[no-untyped-def]
    yield from console.render(self, options)


def _native_measure(self, console, options):  # type: ignore[no-untyped-def]
    return _native.measure(console, options, self)


def _styled_spec(self: Styled, console: "Console", options: "ConsoleOptions") -> tuple:
    return (
        "styled",
        _native.to_spec(console, options, self.renderable),
        _native.native_style(console.get_style(self.style)),
    )


Styled.__fast_rich__ = _styled_spec  # type: ignore[attr-defined]
Styled.__rich_console__ = _native_console  # type: ignore[attr-defined]
Styled.__rich_measure__ = _native_measure  # type: ignore[attr-defined]
