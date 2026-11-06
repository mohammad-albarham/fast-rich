"""Syntax highlighting (Rich's ``rich.syntax`` API, highlighted by the Rust
core with syntect instead of Pygments)."""

from __future__ import annotations

import os.path
import textwrap
from typing import TYPE_CHECKING, Any, Iterable, Optional, Set, Tuple, Union

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
    names the Rust core ships ("monokai", "github", "solarized-dark",
    "solarized-light", "ocean"); other names fall back to monokai.
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

    def _code(self) -> Tuple[str, int]:
        code = self.code
        if self.dedent:
            code = textwrap.dedent(code)
        code = code.expandtabs(self.tab_size)
        start_line = self.start_line
        if self.line_range:
            start, end = self.line_range
            lines = code.splitlines(keepends=True)
            first = max(0, (start or 1) - 1)
            code = "".join(lines[first:end])
            start_line = first + 1
        return code, start_line

    def __fast_rich__(self, console: "Console", options: "ConsoleOptions") -> tuple:
        code, start_line = self._code()
        spec = (
            "syntax",
            code,
            self.lexer or "default",
            dict(
                line_numbers=self.line_numbers,
                theme=self.theme.lower(),
                start_line=start_line,
                highlight_lines=sorted(self.highlight_lines),
            ),
        )
        if self.code_width is not None:
            spec = ("constrain", spec, self.code_width + (7 if self.line_numbers else 0))
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
