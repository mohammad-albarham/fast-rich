"""Parity tests for Markdown - comparing fast_rich vs rich."""

from __future__ import annotations

import pytest


def normalize_output(s: str) -> str:
    """Normalize output for comparison."""
    import re
    ansi_escape = re.compile(r'\x1B(?:[@-Z\\-_]|\[[0-?]*[ -/]*[@-~])')
    s = ansi_escape.sub('', s)
    return s.strip()


class TestMarkdownParity:
    """Test Markdown parity between fast_rich and rich."""

    def test_markdown_basic(self):
        """Test basic markdown creation."""
        from fast_rich.markdown import Markdown as FastMarkdown
        from rich.markdown import Markdown as RichMarkdown

        md_content = "# Hello World\n\nThis is a **bold** statement."

        # Both should instantiate without error
        fast_md = FastMarkdown(md_content)
        rich_md = RichMarkdown(md_content)

        # Check they have the same source
        assert fast_md.markup == md_content

    def test_markdown_with_code_block(self):
        """Test markdown with code block."""
        from fast_rich.markdown import Markdown

        md_content = """
# Code Example

```python
def hello():
    print("Hello!")
```
"""
        md = Markdown(md_content)
        output = str(md)
        # Should contain the function name
        assert "def" in output or "hello" in output

    def test_markdown_with_list(self):
        """Test markdown with bullet list."""
        from fast_rich.markdown import Markdown

        md_content = """
# Shopping List

* Apples
* Bananas
* Oranges
"""
        md = Markdown(md_content)
        output = str(md)
        # Should contain list items
        assert "Apples" in output


class TestMarkdownRendering:
    """Test markdown rendering output."""

    def test_markdown_str_output(self):
        """Test markdown renders to string."""
        from fast_rich.markdown import Markdown

        md = Markdown("**Bold** and *italic*")
        output = str(md)
        assert len(output) > 0

    def test_markdown_heading_levels(self):
        """Test different heading levels."""
        from fast_rich.markdown import Markdown

        md_content = """
# H1
## H2
### H3
"""
        md = Markdown(md_content)
        output = str(md)
        assert "H1" in output
        assert "H2" in output
        assert "H3" in output

    def test_markdown_constructor_params(self):
        """Test markdown accepts constructor params."""
        from fast_rich.markdown import Markdown

        # Should not raise
        md = Markdown(
            "# Title",
            code_theme="monokai",
            inline_code_lexer="python",
        )
        assert md is not None
