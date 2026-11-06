"""Markdown rendering - matches rich.markdown API."""

from __future__ import annotations

from typing import Optional, Union

from fast_rich.style import Style


class Markdown:
    """Render Markdown to the terminal.
    
    Matches rich.markdown.Markdown API.
    """

    def __init__(
        self,
        markup: str,
        *,
        code_theme: str = "monokai",
        justify: Optional[str] = None,
        style: Optional[Union[str, Style]] = "none",
        hyperlinks: bool = True,
        inline_code_lexer: Optional[str] = None,
        inline_code_theme: Optional[str] = None,
    ) -> None:
        """Create Markdown renderer.
        
        Args:
            markup: Markdown text.
            code_theme: Theme for code blocks.
            justify: Text justification.
            style: Base style.
            hyperlinks: Render hyperlinks.
            inline_code_lexer: Lexer for inline code.
            inline_code_theme: Theme for inline code.
        """
        self.markup = markup
        self.code_theme = code_theme
        self.justify = justify
        self.style = style
        self.hyperlinks = hyperlinks
        self.inline_code_lexer = inline_code_lexer
        self.inline_code_theme = inline_code_theme

    def __str__(self) -> str:
        """Render markdown as styled text."""
        # Simple markdown rendering
        lines = self.markup.split("\n")
        result = []
        
        for line in lines:
            # Headers
            if line.startswith("# "):
                result.append(f"\033[1m{line[2:]}\033[0m")
            elif line.startswith("## "):
                result.append(f"\033[1m{line[3:]}\033[0m")
            elif line.startswith("### "):
                result.append(f"\033[1m{line[4:]}\033[0m")
            # Lists
            elif line.startswith("- ") or line.startswith("* "):
                result.append(f"  • {line[2:]}")
            elif line.startswith("  - ") or line.startswith("  * "):
                result.append(f"    ◦ {line[4:]}")
            # Code blocks
            elif line.startswith("```"):
                result.append("─" * 40)
            # Bold
            elif "**" in line:
                import re
                line = re.sub(r'\*\*(.+?)\*\*', r'\033[1m\1\033[0m', line)
                result.append(line)
            # Italic
            elif "*" in line or "_" in line:
                import re
                line = re.sub(r'\*(.+?)\*', r'\033[3m\1\033[0m', line)
                line = re.sub(r'_(.+?)_', r'\033[3m\1\033[0m', line)
                result.append(line)
            else:
                result.append(line)
        
        return "\n".join(result)

    def __rich_console__(self, console, options):
        """Rich console protocol."""
        yield str(self)


__all__ = ["Markdown"]
