# Adapted from Rich (https://github.com/Textualize/rich), Copyright (c) 2020
# Will McGugan, MIT License (see LICENSE.rich). fast_rich change: rendering
# and measuring are done by the Rust core (see _native.py).
from itertools import chain
from typing import TYPE_CHECKING, Iterable, Optional, Literal

from .constrain import Constrain
from .jupyter import JupyterMixin
from .measure import Measurement
from .segment import Segment
from .style import StyleType

if TYPE_CHECKING:
    from .console import Console, ConsoleOptions, RenderableType, RenderResult

AlignMethod = Literal["left", "center", "right"]
VerticalAlignMethod = Literal["top", "middle", "bottom"]


class Align(JupyterMixin):
    """Align a renderable by adding spaces if necessary.

    Args:
        renderable (RenderableType): A console renderable.
        align (AlignMethod): One of "left", "center", or "right""
        style (StyleType, optional): An optional style to apply to the background.
        vertical (Optional[VerticalAlignMethod], optional): Optional vertical align, one of "top", "middle", or "bottom". Defaults to None.
        pad (bool, optional): Pad the right with spaces. Defaults to True.
        width (int, optional): Restrict contents to given width, or None to use default width. Defaults to None.
        height (int, optional): Set height of align renderable, or None to fit to contents. Defaults to None.

    Raises:
        ValueError: if ``align`` is not one of the expected values.

    Example:
        .. code-block:: python

            from fast_rich.console import Console
            from fast_rich.align import Align
            from fast_rich.panel import Panel

            console = Console()
            # Create a panel 20 characters wide
            p = Panel("Hello, [b]World[/b]!", style="on green", width=20)

            # Renders the panel centered in the terminal
            console.print(Align(p, align="center"))
    """

    def __init__(
        self,
        renderable: "RenderableType",
        align: AlignMethod = "left",
        style: Optional[StyleType] = None,
        *,
        vertical: Optional[VerticalAlignMethod] = None,
        pad: bool = True,
        width: Optional[int] = None,
        height: Optional[int] = None,
    ) -> None:
        if align not in ("left", "center", "right"):
            raise ValueError(
                f'invalid value for align, expected "left", "center", or "right" (not {align!r})'
            )
        if vertical is not None and vertical not in ("top", "middle", "bottom"):
            raise ValueError(
                f'invalid value for vertical, expected "top", "middle", or "bottom" (not {vertical!r})'
            )
        self.renderable = renderable
        self.align = align
        self.style = style
        self.vertical = vertical
        self.pad = pad
        self.width = width
        self.height = height

    def __repr__(self) -> str:
        return f"Align({self.renderable!r}, {self.align!r})"

    @classmethod
    def left(
        cls,
        renderable: "RenderableType",
        style: Optional[StyleType] = None,
        *,
        vertical: Optional[VerticalAlignMethod] = None,
        pad: bool = True,
        width: Optional[int] = None,
        height: Optional[int] = None,
    ) -> "Align":
        """Align a renderable to the left."""
        return cls(
            renderable,
            "left",
            style=style,
            vertical=vertical,
            pad=pad,
            width=width,
            height=height,
        )

    @classmethod
    def center(
        cls,
        renderable: "RenderableType",
        style: Optional[StyleType] = None,
        *,
        vertical: Optional[VerticalAlignMethod] = None,
        pad: bool = True,
        width: Optional[int] = None,
        height: Optional[int] = None,
    ) -> "Align":
        """Align a renderable to the center."""
        return cls(
            renderable,
            "center",
            style=style,
            vertical=vertical,
            pad=pad,
            width=width,
            height=height,
        )

    @classmethod
    def right(
        cls,
        renderable: "RenderableType",
        style: Optional[StyleType] = None,
        *,
        vertical: Optional[VerticalAlignMethod] = None,
        pad: bool = True,
        width: Optional[int] = None,
        height: Optional[int] = None,
    ) -> "Align":
        """Align a renderable to the right."""
        return cls(
            renderable,
            "right",
            style=style,
            vertical=vertical,
            pad=pad,
            width=width,
            height=height,
        )

class VerticalCenter(JupyterMixin):
    """Vertically aligns a renderable.

    Warn:
        This class is deprecated and may be removed in a future version. Use Align class with
        `vertical="middle"`.

    Args:
        renderable (RenderableType): A renderable object.
        style (StyleType, optional): An optional style to apply to the background. Defaults to None.
    """

    def __init__(
        self,
        renderable: "RenderableType",
        style: Optional[StyleType] = None,
    ) -> None:
        self.renderable = renderable
        self.style = style

    def __repr__(self) -> str:
        return f"VerticalCenter({self.renderable!r})"

    def __rich_console__(
        self, console: "Console", options: "ConsoleOptions"
    ) -> "RenderResult":
        style = console.get_style(self.style) if self.style is not None else None
        lines = console.render_lines(
            self.renderable, options.update(height=None), pad=False
        )
        width, _height = Segment.get_shape(lines)
        new_line = Segment.line()
        height = options.height or options.size.height
        top_space = (height - len(lines)) // 2
        bottom_space = height - top_space - len(lines)
        blank_line = Segment(f"{' ' * width}", style)

        def blank_lines(count: int) -> Iterable[Segment]:
            for _ in range(count):
                yield blank_line
                yield new_line

        if top_space > 0:
            yield from blank_lines(top_space)
        for line in lines:
            yield from line
            yield new_line
        if bottom_space > 0:
            yield from blank_lines(bottom_space)

    def __rich_measure__(
        self, console: "Console", options: "ConsoleOptions"
    ) -> Measurement:
        measurement = Measurement.get(console, options, self.renderable)
        return measurement


from . import _native  # noqa: E402

def _native_console(self, console, options):  # type: ignore[no-untyped-def]
    yield from console.render(self, options)


def _native_measure(self, console, options):  # type: ignore[no-untyped-def]
    return _native.measure(console, options, self)


def _align_spec(self: Align, console: "Console", options: "ConsoleOptions") -> tuple:
    child = _native.to_spec(console, options.update(height=None), self.renderable)
    if self.width is not None:
        child = ("constrain", child, self.width)
    height = self.height if self.height is not None else options.height
    return (
        "align",
        child,
        dict(
            align=self.align,
            pad=self.pad,
            vertical=self.vertical or "top",
            height=height if self.vertical else None,
        ),
    )


Align.__fast_rich__ = _align_spec  # type: ignore[attr-defined]
Align.__rich_console__ = _native_console  # type: ignore[attr-defined]
Align.__rich_measure__ = _native_measure  # type: ignore[attr-defined]
