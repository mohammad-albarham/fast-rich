"""Syntax highlighting - matches rich.syntax API."""

from __future__ import annotations

from typing import Optional, Union

from fast_rich.style import Style


class Syntax:
    """Syntax highlighted source code.
    
    Matches rich.syntax.Syntax API.
    """

    def __init__(
        self,
        code: str,
        lexer: str,
        *,
        theme: str = "monokai",
        dedent: bool = False,
        line_numbers: bool = False,
        start_line: int = 1,
        line_range: Optional[tuple] = None,
        highlight_lines: Optional[set] = None,
        code_width: Optional[int] = None,
        tab_size: int = 4,
        word_wrap: bool = False,
        background_color: Optional[str] = None,
        indent_guides: bool = False,
        padding: Union[int, tuple] = 0,
    ) -> None:
        """Create Syntax highlighter.
        
        Args:
            code: Source code.
            lexer: Language lexer name.
            theme: Color theme.
            dedent: Remove leading whitespace.
            line_numbers: Show line numbers.
            start_line: Starting line number.
            line_range: Range of lines to show.
            highlight_lines: Lines to highlight.
            code_width: Fixed code width.
            tab_size: Tab size.
            word_wrap: Wrap long lines.
            background_color: Background color.
            indent_guides: Show indent guides.
            padding: Padding around code.
        """
        self.code = code
        self.lexer = lexer
        self.theme = theme
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

        if self.dedent:
            import textwrap
            self.code = textwrap.dedent(self.code)

    @classmethod
    def from_path(
        cls,
        path: str,
        encoding: str = "utf-8",
        lexer: Optional[str] = None,
        theme: str = "monokai",
        dedent: bool = False,
        line_numbers: bool = False,
        line_range: Optional[tuple] = None,
        start_line: int = 1,
        highlight_lines: Optional[set] = None,
        code_width: Optional[int] = None,
        tab_size: int = 4,
        word_wrap: bool = False,
        background_color: Optional[str] = None,
        indent_guides: bool = False,
        padding: Union[int, tuple] = 0,
    ) -> "Syntax":
        """Create Syntax from a file path."""
        with open(path, encoding=encoding) as f:
            code = f.read()
        
        if lexer is None:
            # Guess lexer from extension
            ext = path.rsplit(".", 1)[-1] if "." in path else ""
            lexer_map = {
                "py": "python",
                "rs": "rust",
                "js": "javascript",
                "ts": "typescript",
                "go": "go",
                "rb": "ruby",
                "java": "java",
                "c": "c",
                "cpp": "cpp",
                "h": "c",
                "hpp": "cpp",
                "json": "json",
                "yaml": "yaml",
                "yml": "yaml",
                "toml": "toml",
                "md": "markdown",
                "sh": "bash",
                "bash": "bash",
                "sql": "sql",
                "html": "html",
                "css": "css",
            }
            lexer = lexer_map.get(ext, "text")
        
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

    def __str__(self) -> str:
        """Render syntax highlighted code."""
        lines = self.code.split("\n")
        
        if self.line_range:
            start, end = self.line_range
            lines = lines[start - 1:end]
            
        result = []
        for i, line in enumerate(lines):
            line_num = self.start_line + i
            
            if self.line_numbers:
                prefix = f"{line_num:4d} │ "
            else:
                prefix = ""
                
            # Highlight line if in highlight_lines
            if line_num in self.highlight_lines:
                result.append(f"\033[43m{prefix}{line}\033[0m")
            else:
                result.append(f"{prefix}{line}")
        
        return "\n".join(result)

    def __rich_console__(self, console, options):
        """Rich console protocol."""
        yield str(self)


__all__ = ["Syntax"]
