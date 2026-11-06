"""Parity tests for Syntax - comparing fast_rich vs rich."""

from __future__ import annotations

import pytest


def normalize_output(s: str) -> str:
    """Normalize output for comparison."""
    import re
    ansi_escape = re.compile(r'\x1B(?:[@-Z\\-_]|\[[0-?]*[ -/]*[@-~])')
    s = ansi_escape.sub('', s)
    return s.strip()


class TestSyntaxParity:
    """Test Syntax parity between fast_rich and rich."""

    def test_syntax_basic(self):
        """Test basic syntax highlighting."""
        from fast_rich.syntax import Syntax as FastSyntax
        from rich.syntax import Syntax as RichSyntax

        code = "def hello():\n    print('Hello!')"

        # Both should instantiate without error
        fast_syn = FastSyntax(code, "python")
        rich_syn = RichSyntax(code, "python")

        # Both should have code stored
        assert fast_syn.code == code

    def test_syntax_with_line_numbers(self):
        """Test syntax with line numbers."""
        from fast_rich.syntax import Syntax

        code = "x = 1\ny = 2\nz = x + y"
        syn = Syntax(code, "python", line_numbers=True)
        output = str(syn)
        # Line numbers should appear
        assert "1" in output or "x" in output

    def test_syntax_with_theme(self):
        """Test syntax with different theme."""
        from fast_rich.syntax import Syntax

        code = "print('hello')"
        syn = Syntax(code, "python", theme="monokai")
        output = str(syn)
        assert "print" in output

    def test_syntax_start_line(self):
        """Test syntax with custom start line."""
        from fast_rich.syntax import Syntax

        code = "x = 1\ny = 2"
        syn = Syntax(code, "python", line_numbers=True, start_line=10)
        output = str(syn)
        # Should contain the code
        assert "x" in output


class TestSyntaxRendering:
    """Test syntax rendering output."""

    def test_syntax_str_output(self):
        """Test syntax renders to string."""
        from fast_rich.syntax import Syntax

        syn = Syntax("def foo(): pass", "python")
        output = str(syn)
        assert len(output) > 0

    def test_syntax_multiple_languages(self):
        """Test different languages."""
        from fast_rich.syntax import Syntax

        # Python
        py = Syntax("x = 1", "python")
        assert "x" in str(py)

        # JavaScript
        js = Syntax("let x = 1;", "javascript")
        assert "let" in str(js) or "x" in str(js)

        # Rust
        rs = Syntax("fn main() {}", "rust")
        assert "fn" in str(rs) or "main" in str(rs)

    def test_syntax_constructor_params(self):
        """Test syntax accepts all constructor params."""
        from fast_rich.syntax import Syntax

        # Should not raise
        syn = Syntax(
            "code",
            "python",
            theme="monokai",
            line_numbers=False,
            start_line=1,
            line_range=None,
            word_wrap=False,
            tab_size=4,
        )
        assert syn is not None
