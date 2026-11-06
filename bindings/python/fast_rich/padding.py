# Adapted from Rich (https://github.com/Textualize/rich), Copyright (c) 2020
# Will McGugan, MIT License (see LICENSE.rich). fast_rich change: rendering
# and measuring are done by the Rust core (see _native.py).
from typing import TYPE_CHECKING, List, Optional, Tuple, Union

if TYPE_CHECKING:
    from .console import (
        Console,
        ConsoleOptions,
        RenderableType,
        RenderResult,
    )

from .jupyter import JupyterMixin
from .measure import Measurement
from .segment import Segment
from .style import Style

PaddingDimensions = Union[int, Tuple[int], Tuple[int, int], Tuple[int, int, int, int]]


class Padding(JupyterMixin):
    """Draw space around content.

    Example:
        >>> print(Padding("Hello", (2, 4), style="on blue"))

    Args:
        renderable (RenderableType): String or other renderable.
        pad (Union[int, Tuple[int]]): Padding for top, right, bottom, and left borders.
            May be specified with 1, 2, or 4 integers (CSS style).
        style (Union[str, Style], optional): Style for padding characters. Defaults to "none".
        expand (bool, optional): Expand padding to fit available width. Defaults to True.
    """

    def __init__(
        self,
        renderable: "RenderableType",
        pad: "PaddingDimensions" = (0, 0, 0, 0),
        *,
        style: Union[str, Style] = "none",
        expand: bool = True,
    ):
        self.renderable = renderable
        self.top, self.right, self.bottom, self.left = self.unpack(pad)
        self.style = style
        self.expand = expand

    @classmethod
    def indent(cls, renderable: "RenderableType", level: int) -> "Padding":
        """Make padding instance to render an indent.

        Args:
            renderable (RenderableType): String or other renderable.
            level (int): Number of characters to indent.

        Returns:
            Padding: A Padding instance.
        """

        return Padding(renderable, pad=(0, 0, 0, level), expand=False)

    @staticmethod
    def unpack(pad: "PaddingDimensions") -> Tuple[int, int, int, int]:
        """Unpack padding specified in CSS style."""
        if isinstance(pad, int):
            return (pad, pad, pad, pad)
        if len(pad) == 1:
            _pad = pad[0]
            return (_pad, _pad, _pad, _pad)
        if len(pad) == 2:
            pad_top, pad_right = pad
            return (pad_top, pad_right, pad_top, pad_right)
        if len(pad) == 4:
            top, right, bottom, left = pad
            return (top, right, bottom, left)
        raise ValueError(f"1, 2 or 4 integers required for padding; {len(pad)} given")

    def __repr__(self) -> str:
        return f"Padding({self.renderable!r}, ({self.top},{self.right},{self.bottom},{self.left}))"


from . import _native  # noqa: E402

def _native_console(self, console, options):  # type: ignore[no-untyped-def]
    yield from console.render(self, options)


def _native_measure(self, console, options):  # type: ignore[no-untyped-def]
    return _native.measure(console, options, self)


def _padding_spec(self: Padding, console: "Console", options: "ConsoleOptions") -> tuple:
    return (
        "padding",
        _native.to_spec(console, options, self.renderable),
        dict(
            pad=(self.top, self.right, self.bottom, self.left),
            style=_native.native_style(console.get_style(self.style)),
            expand=self.expand,
        ),
    )


Padding.__fast_rich__ = _padding_spec  # type: ignore[attr-defined]
Padding.__rich_console__ = _native_console  # type: ignore[attr-defined]
Padding.__rich_measure__ = _native_measure  # type: ignore[attr-defined]
