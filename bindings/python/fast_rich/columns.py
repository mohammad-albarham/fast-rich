# Adapted from Rich (https://github.com/Textualize/rich), Copyright (c) 2020
# Will McGugan, MIT License (see LICENSE.rich). fast_rich change: rendering
# and measuring are done by the Rust core (see _native.py).
from collections import defaultdict
from itertools import chain
from operator import itemgetter
from typing import Dict, Iterable, List, Optional, Tuple

from .align import Align, AlignMethod
from .console import Console, ConsoleOptions, RenderableType, RenderResult
from .constrain import Constrain
from .measure import Measurement
from .padding import Padding, PaddingDimensions
from .table import Table
from .text import TextType
from .jupyter import JupyterMixin


class Columns(JupyterMixin):
    """Display renderables in neat columns.

    Args:
        renderables (Iterable[RenderableType]): Any number of Rich renderables (including str).
        width (int, optional): The desired width of the columns, or None to auto detect. Defaults to None.
        padding (PaddingDimensions, optional): Optional padding around cells. Defaults to (0, 1).
        expand (bool, optional): Expand columns to full width. Defaults to False.
        equal (bool, optional): Arrange in to equal sized columns. Defaults to False.
        column_first (bool, optional): Align items from top to bottom (rather than left to right). Defaults to False.
        right_to_left (bool, optional): Start column from right hand side. Defaults to False.
        align (str, optional): Align value ("left", "right", or "center") or None for default. Defaults to None.
        title (TextType, optional): Optional title for Columns.
    """

    def __init__(
        self,
        renderables: Optional[Iterable[RenderableType]] = None,
        padding: PaddingDimensions = (0, 1),
        *,
        width: Optional[int] = None,
        expand: bool = False,
        equal: bool = False,
        column_first: bool = False,
        right_to_left: bool = False,
        align: Optional[AlignMethod] = None,
        title: Optional[TextType] = None,
    ) -> None:
        self.renderables = list(renderables or [])
        self.width = width
        self.padding = padding
        self.expand = expand
        self.equal = equal
        self.column_first = column_first
        self.right_to_left = right_to_left
        self.align: Optional[AlignMethod] = align
        self.title = title

    def add_renderable(self, renderable: RenderableType) -> None:
        """Add a renderable to the columns.

        Args:
            renderable (RenderableType): Any renderable object.
        """
        self.renderables.append(renderable)


from . import _native  # noqa: E402

def _native_console(self, console, options):  # type: ignore[no-untyped-def]
    yield from console.render(self, options)


def _native_measure(self, console, options):  # type: ignore[no-untyped-def]
    return _native.measure(console, options, self)


def _columns_spec(self: Columns, console: "Console", options: "ConsoleOptions") -> tuple:
    _top, right, _bottom, left = Padding.unpack(self.padding)
    items = [_native.to_spec(console, options, r) for r in self.renderables]
    spec = (
        "columns",
        items,
        dict(
            gap=max(left, right),
            expand=self.expand,
            equal=self.equal,
            column_first=self.column_first,
            right_to_left=self.right_to_left,
            align=self.align,
            width=self.width,
        ),
    )
    if self.title:
        title = console.render_str(self.title, style="table.title", justify="center") if isinstance(self.title, str) else self.title
        spec = ("group", [_native.text_spec(console, options.update(justify="center"), title), spec])
    return spec


Columns.__fast_rich__ = _columns_spec  # type: ignore[attr-defined]
Columns.__rich_console__ = _native_console  # type: ignore[attr-defined]
