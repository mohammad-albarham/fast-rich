"""Parity tests for Console - comparing fast_rich vs rich."""

from __future__ import annotations

import io
import pytest


def normalize_output(s: str) -> str:
    """Normalize output for comparison by stripping ANSI codes and whitespace."""
    import re
    # Remove ANSI escape codes
    ansi_escape = re.compile(r'\x1B(?:[@-Z\\-_]|\[[0-?]*[ -/]*[@-~])')
    s = ansi_escape.sub('', s)
    # Normalize whitespace
    return s.strip()


class TestConsoleParity:
    """Test Console parity between fast_rich and rich."""

    def test_basic_print(self):
        """Test basic print output matches."""
        # fast_rich
        from fast_rich.console import Console as FastConsole
        fast_out = io.StringIO()
        fc = FastConsole(file=fast_out)
        fc.print("Hello, World!")
        fast_result = fast_out.getvalue()

        # rich
        from rich.console import Console as RichConsole
        rich_out = io.StringIO()
        rc = RichConsole(file=rich_out, force_terminal=True)
        rc.print("Hello, World!")
        rich_result = rich_out.getvalue()

        assert normalize_output(fast_result) == normalize_output(rich_result)

    def test_styled_print(self):
        """Test styled print output."""
        from fast_rich.console import Console as FastConsole
        fast_out = io.StringIO()
        fc = FastConsole(file=fast_out)
        fc.print("Styled text", style="bold")
        fast_result = fast_out.getvalue()

        # Just verify it doesn't error and produces output
        assert "Styled text" in fast_result

    def test_console_width(self):
        """Test console width property."""
        from fast_rich.console import Console as FastConsole
        from rich.console import Console as RichConsole

        fc = FastConsole(width=80)
        rc = RichConsole(width=80)

        assert fc.width == rc.width

    def test_log_method(self):
        """Test log method exists and works."""
        from fast_rich.console import Console as FastConsole
        fast_out = io.StringIO()
        fc = FastConsole(file=fast_out)
        fc.log("Log message")
        
        assert "Log message" in fast_out.getvalue()

    def test_rule(self):
        """Test rule rendering."""
        from fast_rich.console import Console as FastConsole
        fast_out = io.StringIO()
        fc = FastConsole(file=fast_out, width=40)
        fc.rule("Title")
        
        result = fast_out.getvalue()
        assert "Title" in result


class TestConsoleSignature:
    """Test that Console has the same constructor signature as rich."""

    def test_constructor_params(self):
        """Verify constructor accepts all rich.Console params."""
        from fast_rich.console import Console
        
        # Should not raise
        console = Console(
            color_system="auto",
            force_terminal=True,
            soft_wrap=False,
            width=80,
            height=25,
            tab_size=8,
            record=True,
            markup=True,
            emoji=True,
            highlight=True,
        )
        
        assert console.width == 80

    def test_print_signature(self):
        """Verify print() accepts all rich params."""
        from fast_rich.console import Console
        import io
        
        out = io.StringIO()
        console = Console(file=out)
        
        # Should not raise
        console.print(
            "Hello",
            sep=" ",
            end="\n",
            style=None,
            justify="left",
            overflow="fold",
            no_wrap=False,
            emoji=True,
            markup=True,
            highlight=False,
            width=None,
            crop=True,
            soft_wrap=False,
            new_line_start=False,
        )
        
        assert "Hello" in out.getvalue()
