"""Syntax highlighting (Rich's ``rich.syntax`` API, highlighted by the Rust
core with syntect instead of Pygments)."""

from __future__ import annotations

import os.path
import textwrap
from typing import TYPE_CHECKING, Any, Optional, Set, Tuple, Union

from .jupyter import JupyterMixin
from .padding import Padding, PaddingDimensions

if TYPE_CHECKING:  # pragma: no cover
    from .console import Console, ConsoleOptions

from . import _native

# File extension -> lexer name, for Syntax.guess_lexer / from_path.
_EXTENSIONS = {
    ".py": "python", ".pyw": "python", ".pyi": "python", ".rs": "rust",
    ".js": "javascript", ".mjs": "javascript", ".ts": "typescript",
    ".tsx": "tsx", ".jsx": "jsx", ".json": "json", ".toml": "toml",
    ".yaml": "yaml", ".yml": "yaml", ".md": "markdown", ".html": "html",
    ".htm": "html", ".css": "css", ".c": "c", ".h": "c", ".cpp": "cpp",
    ".cc": "cpp", ".hpp": "cpp", ".java": "java", ".go": "go", ".rb": "ruby",
    ".php": "php", ".sh": "bash", ".bash": "bash", ".zsh": "bash",
    ".sql": "sql", ".xml": "xml", ".lua": "lua", ".pl": "perl", ".r": "r",
    ".swift": "swift", ".kt": "kotlin", ".scala": "scala", ".cs": "cs",
    ".hs": "haskell", ".ini": "ini", ".diff": "diff", ".patch": "diff",
}


class Syntax(JupyterMixin):
    """Construct a Syntax object to render syntax highlighted code.

    Same constructor as ``rich.syntax.Syntax``. ``theme`` accepts the theme
    names the Rust core ships ("monokai", "ansi_dark", "ansi_light",
    "github", "solarized-dark", "solarized-light", "ocean"); other names
    fall back to monokai.
    """

    def __init__(
        self,
        code: str,
        lexer: Union[str, Any],
        *,
        theme: Union[str, Any] = "monokai",
        dedent: bool = False,
        line_numbers: bool = False,
        start_line: int = 1,
        line_range: Optional[Tuple[Optional[int], Optional[int]]] = None,
        highlight_lines: Optional[Set[int]] = None,
        code_width: Optional[int] = None,
        tab_size: int = 4,
        word_wrap: bool = False,
        background_color: Optional[str] = None,
        indent_guides: bool = False,
        padding: PaddingDimensions = 0,
    ) -> None:
        self.code = code
        self._lexer = lexer
        self.dedent = dedent
        self.line_numbers = line_numbers
        self.start_line = start_line
        self.line_range = line_range
        self.highlight_lines = highlight_lines or set()
        self.code_width = code_width
        self.tab_size = tab_size
        self.word_wrap = word_wrap
        self.background_color = background_color
        self.indent_guides = indent_guides
        self.padding = padding
        self.theme = theme if isinstance(theme, str) else "monokai"
        self._stylized_ranges: list = []

    @property
    def lexer(self) -> Optional[str]:
        """The lexer name."""
        return self._lexer if isinstance(self._lexer, str) else getattr(self._lexer, "name", None)

    @classmethod
    def from_path(
        cls,
        path: str,
        encoding: str = "utf-8",
        lexer: Optional[Union[str, Any]] = None,
        theme: Union[str, Any] = "monokai",
        dedent: bool = False,
        line_numbers: bool = False,
        line_range: Optional[Tuple[int, int]] = None,
        start_line: int = 1,
        highlight_lines: Optional[Set[int]] = None,
        code_width: Optional[int] = None,
        tab_size: int = 4,
        word_wrap: bool = False,
        background_color: Optional[str] = None,
        indent_guides: bool = False,
        padding: PaddingDimensions = 0,
    ) -> "Syntax":
        """Construct a Syntax object from a file."""
        with open(path, "rt", encoding=encoding) as code_file:
            code = code_file.read()
        if not lexer:
            lexer = cls.guess_lexer(path, code=code)
        return cls(
            code,
            lexer,
            theme=theme,
            dedent=dedent,
            line_numbers=line_numbers,
            line_range=line_range,
            start_line=start_line,
            highlight_lines=highlight_lines,
            code_width=code_width,
            tab_size=tab_size,
            word_wrap=word_wrap,
            background_color=background_color,
            indent_guides=indent_guides,
            padding=padding,
        )

    @classmethod
    def guess_lexer(cls, path: str, code: Optional[str] = None) -> str:
        """Guess the lexer name from a file name (or a ``#!`` line)."""
        ext = os.path.splitext(path)[1].lower()
        if ext in _EXTENSIONS:
            return _EXTENSIONS[ext]
        if code and code.startswith("#!"):
            first = code.splitlines()[0]
            for name in ("python", "bash", "ruby", "perl", "node"):
                if name in first:
                    return "javascript" if name == "node" else name
        return "default"

    def stylize_range(
        self,
        style: Any,
        start: Tuple[int, int],
        end: Tuple[int, int],
        style_before: bool = False,
    ) -> None:
        """Apply a style over ``start`` to ``end`` (``(line, column)``, lines
        1-based, columns 0-based), like ``rich.syntax.Syntax.stylize_range``."""
        self._stylized_ranges.append((style, start, end))

    def _code(self) -> str:
        code = self.code
        if self.dedent:
            code = textwrap.dedent(code)
        return code.expandtabs(self.tab_size)

    def __fast_rich__(self, console: "Console", options: "ConsoleOptions") -> tuple:
        code = self._code()
        ranges = []
        for style, (line1, col1), (line2, col2) in self._stylized_ranges:
            native = _native.native_style(console.get_style(style))
            for line in range(line1, line2 + 1):
                start = col1 if line == line1 else 0
                end = col2 if line == line2 else 1 << 30
                if start < end:
                    ranges.append((line, start, end, native))
        spec = (
            "syntax",
            code,
            self.lexer or "default",
            dict(
                line_numbers=self.line_numbers,
                theme=self.theme.lower(),
                start_line=self.start_line,
                line_range=(
                    (max(1, self.line_range[0] or 1), self.line_range[1] or (1 << 30))
                    if self.line_range
                    else None
                ),
                code_width=self.code_width,
                highlight_lines=sorted(self.highlight_lines),
                indent_guides=self.indent_guides and not options.ascii_only,
                tab_size=self.tab_size,
                ranges=ranges,
            ),
        )
        if self.padding:
            spec = (
                "padding",
                spec,
                dict(pad=Padding.unpack(self.padding), style="", expand=False),
            )
        return spec

    def __rich_console__(self, console, options):  # type: ignore[no-untyped-def]
        yield from console.render(self, options)

    def __rich_measure__(self, console, options):  # type: ignore[no-untyped-def]
        return _native.measure(console, options, self)


__all__ = ["Syntax"]
