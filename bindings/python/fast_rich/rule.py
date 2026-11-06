# Adapted from Rich (https://github.com/Textualize/rich), Copyright (c) 2020
# Will McGugan, MIT License (see LICENSE.rich). fast_rich change: rendering
# and measuring are done by the Rust core (see _native.py).
from typing import Union

from .align import AlignMethod
from .cells import cell_len, set_cell_size
from .console import Console, ConsoleOptions, RenderResult
from .jupyter import JupyterMixin
from .measure import Measurement
from .style import Style
from .text import Text


class Rule(JupyterMixin):
    """A console renderable to draw a horizontal rule (line).

    Args:
        title (Union[str, Text], optional): Text to render in the rule. Defaults to "".
        characters (str, optional): Character(s) used to draw the line. Defaults to "─".
        style (StyleType, optional): Style of Rule. Defaults to "rule.line".
        end (str, optional): Character at end of Rule. defaults to "\\\\n"
        align (str, optional): How to align the title, one of "left", "center", or "right". Defaults to "center".
    """

    def __init__(
        self,
        title: Union[str, Text] = "",
        *,
        characters: str = "─",
        style: Union[str, Style] = "rule.line",
        end: str = "\n",
        align: AlignMethod = "center",
    ) -> None:
        if cell_len(characters) < 1:
            raise ValueError(
                "'characters' argument must have a cell width of at least 1"
            )
        if align not in ("left", "center", "right"):
            raise ValueError(
                f'invalid value for align, expected "left", "center", "right" (not {align!r})'
            )
        self.title = title
        self.characters = characters
        self.style = style
        self.end = end
        self.align = align

    def __repr__(self) -> str:
        return f"Rule({self.title!r}, {self.characters!r})"


from . import _native  # noqa: E402

def _native_console(self, console, options):  # type: ignore[no-untyped-def]
    yield from console.render(self, options)


def _native_measure(self, console, options):  # type: ignore[no-untyped-def]
    return _native.measure(console, options, self)


def _rule_spec(self: Rule, console: "Console", options: "ConsoleOptions") -> tuple:
    characters = (
        "-"
        if (options.ascii_only and not self.characters.isascii())
        else self.characters
    )
    title = None
    if self.title:
        title_text = (
            self.title
            if isinstance(self.title, Text)
            else console.render_str(self.title, style="rule.text")
        )
        title = _native.text_spec(console, options, title_text)
    return (
        "rule",
        dict(
            title=title,
            characters=characters,
            style=_native.native_style(console.get_style(self.style)),
            align=self.align,
        ),
    )


def _rule_measure(self, console, options):  # type: ignore[no-untyped-def]
    return Measurement(1, 1)


Rule.__fast_rich__ = _rule_spec  # type: ignore[attr-defined]
Rule.__rich_console__ = _native_console  # type: ignore[attr-defined]
Rule.__rich_measure__ = _rule_measure  # type: ignore[attr-defined]
