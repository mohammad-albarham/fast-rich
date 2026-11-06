# Adapted from Rich (https://github.com/Textualize/rich), Copyright (c) 2020
# Will McGugan, MIT License (see LICENSE.rich). fast_rich change: rendering
# and measuring are done by the Rust core (see _native.py).
from typing import Optional, TYPE_CHECKING

from .jupyter import JupyterMixin
from .measure import Measurement

if TYPE_CHECKING:
    from .console import Console, ConsoleOptions, RenderableType, RenderResult


class Constrain(JupyterMixin):
    """Constrain the width of a renderable to a given number of characters.

    Args:
        renderable (RenderableType): A renderable object.
        width (int, optional): The maximum width (in characters) to render. Defaults to 80.
    """

    def __init__(self, renderable: "RenderableType", width: Optional[int] = 80) -> None:
        self.renderable = renderable
        self.width = width


from . import _native  # noqa: E402

def _native_console(self, console, options):  # type: ignore[no-untyped-def]
    yield from console.render(self, options)


def _native_measure(self, console, options):  # type: ignore[no-untyped-def]
    return _native.measure(console, options, self)


def _constrain_spec(self: Constrain, console: "Console", options: "ConsoleOptions") -> tuple:
    return ("constrain", _native.to_spec(console, options, self.renderable), self.width)


Constrain.__fast_rich__ = _constrain_spec  # type: ignore[attr-defined]
Constrain.__rich_console__ = _native_console  # type: ignore[attr-defined]
Constrain.__rich_measure__ = _native_measure  # type: ignore[attr-defined]
