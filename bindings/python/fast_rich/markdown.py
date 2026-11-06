"""Markdown rendering (Rich's ``rich.markdown`` API, rendered by the Rust core)."""

from __future__ import annotations

from typing import TYPE_CHECKING, Optional

from .jupyter import JupyterMixin
from .style import StyleType

if TYPE_CHECKING:  # pragma: no cover
    from .console import Console, ConsoleOptions, JustifyMethod

from . import _native


class Markdown(JupyterMixin):
    """A Markdown renderable.

    Args:
        markup (str): A string containing markdown.
        code_theme (str, optional): Theme for code blocks. Defaults to "monokai".
        justify (JustifyMethod, optional): Kept for Rich compatibility.
        style (StyleType, optional): Base style for the whole document.
        hyperlinks (bool, optional): Kept for Rich compatibility.
        inline_code_lexer (str, optional): Kept for Rich compatibility.
        inline_code_theme (str, optional): Kept for Rich compatibility.
    """

    def __init__(
        self,
        markup: str,
        code_theme: str = "monokai",
        justify: Optional["JustifyMethod"] = None,
        style: StyleType = "none",
        hyperlinks: bool = True,
        inline_code_lexer: Optional[str] = None,
        inline_code_theme: Optional[str] = None,
    ) -> None:
        self.markup = markup
        self.code_theme = code_theme
        self.justify = justify
        self.style = style
        self.hyperlinks = hyperlinks
        self.inline_code_lexer = inline_code_lexer
        self.inline_code_theme = inline_code_theme

    def __fast_rich__(self, console: "Console", options: "ConsoleOptions") -> tuple:
        spec = ("markdown", self.markup)
        style = console.get_style(self.style, default="none")
        if style:
            spec = ("styled", spec, _native.native_style(style))
        return spec

    def __rich_console__(self, console, options):  # type: ignore[no-untyped-def]
        yield from console.render(self, options)

    def __rich_measure__(self, console, options):  # type: ignore[no-untyped-def]
        return _native.measure(console, options, self)


__all__ = ["Markdown"]
