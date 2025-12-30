"""Parity tests for Text, Panel, and other core classes."""

from __future__ import annotations

import pytest


class TestTextParity:
    """Test Text parity between fast_rich and rich."""

    def test_basic_text(self):
        """Test basic Text creation."""
        from fast_rich.text import Text as FastText
        from rich.text import Text as RichText

        ft = FastText("Hello, World!")
        rt = RichText("Hello, World!")

        assert str(ft) == str(rt)

    def test_text_plain_property(self):
        """Test plain property."""
        from fast_rich.text import Text

        text = Text("Hello")
        assert text.plain == "Hello"

    def test_text_append(self):
        """Test append method."""
        from fast_rich.text import Text

        text = Text("Hello")
        text.append(", World!")
        assert str(text) == "Hello, World!"

    def test_text_concatenation(self):
        """Test text concatenation."""
        from fast_rich.text import Text

        t1 = Text("Hello")
        t2 = t1 + " World"
        assert str(t2) == "Hello World"

    def test_text_from_markup(self):
        """Test from_markup class method."""
        from fast_rich.text import Text

        text = Text.from_markup("[bold]Hello[/bold]")
        assert "Hello" in str(text)

    def test_text_assemble(self):
        """Test assemble class method."""
        from fast_rich.text import Text

        text = Text.assemble("Hello, ", ("World", "bold"))
        assert str(text) == "Hello, World"


class TestPanelParity:
    """Test Panel parity."""

    def test_basic_panel(self):
        """Test basic Panel creation."""
        from fast_rich.panel import Panel

        panel = Panel("Content")
        output = str(panel)
        assert "Content" in output

    def test_panel_with_title(self):
        """Test Panel with title."""
        from fast_rich.panel import Panel

        panel = Panel("Content", title="My Panel")
        output = str(panel)
        assert "Content" in output
        assert "My Panel" in output

    def test_panel_fit(self):
        """Test Panel.fit class method."""
        from fast_rich.panel import Panel

        panel = Panel.fit("Content", title="Fitted")
        assert panel.expand is False


class TestRuleParity:
    """Test Rule parity."""

    def test_basic_rule(self):
        """Test basic Rule creation."""
        from fast_rich.rule import Rule

        rule = Rule()
        output = str(rule)
        assert "─" in output

    def test_rule_with_title(self):
        """Test Rule with title."""
        from fast_rich.rule import Rule

        rule = Rule("Title")
        output = str(rule)
        assert "Title" in output


class TestStyleParity:
    """Test Style parity."""

    def test_style_parse(self):
        """Test Style.parse method."""
        from fast_rich.style import Style

        style = Style.parse("bold red on blue")
        assert style.bold is True
        assert style.color == "red"
        assert style.bgcolor == "blue"

    def test_style_combination(self):
        """Test style combination with +."""
        from fast_rich.style import Style

        s1 = Style(bold=True)
        s2 = Style(color="red")
        combined = s1 + s2

        assert combined.bold is True
        assert combined.color == "red"

    def test_style_str(self):
        """Test style string representation."""
        from fast_rich.style import Style

        style = Style(bold=True, color="red")
        assert "bold" in str(style)
        assert "red" in str(style)
