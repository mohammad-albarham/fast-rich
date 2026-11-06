"""Parity tests for Table - comparing fast_rich vs rich."""

from __future__ import annotations

import io
import pytest


def normalize_output(s: str) -> str:
    """Normalize output for comparison."""
    import re
    ansi_escape = re.compile(r'\x1B(?:[@-Z\\-_]|\[[0-?]*[ -/]*[@-~])')
    s = ansi_escape.sub('', s)
    return s.strip()


class TestTableParity:
    """Test Table parity between fast_rich and rich."""

    def test_basic_table(self):
        """Test basic table creation."""
        from fast_rich.table import Table as FastTable
        from rich.table import Table as RichTable

        # fast_rich
        ft = FastTable()
        ft.add_column("Name")
        ft.add_column("Age")
        ft.add_row("Alice", "30")
        ft.add_row("Bob", "25")

        # rich
        rt = RichTable()
        rt.add_column("Name")
        rt.add_column("Age")
        rt.add_row("Alice", "30")
        rt.add_row("Bob", "25")

        # Both should have same row count
        assert ft.row_count == rt.row_count

    def test_table_with_title(self):
        """Test table with title."""
        from fast_rich.table import Table as FastTable

        table = FastTable(title="My Table")
        table.add_column("Col1")
        table.add_row("Value1")

        output = str(table)
        assert "My Table" in output

    def test_table_constructor_params(self):
        """Test table accepts all constructor params."""
        from fast_rich.table import Table
        from fast_rich.box import ROUNDED

        # Should not raise
        table = Table(
            "Header1", "Header2",
            title="Title",
            caption="Caption",
            width=80,
            box=ROUNDED,
            show_header=True,
            show_footer=False,
            show_edge=True,
            show_lines=False,
            expand=False,
        )

        assert len(table.columns) == 2

    def test_add_column_params(self):
        """Test add_column accepts all params."""
        from fast_rich.table import Table

        table = Table()
        table.add_column(
            "Name",
            footer="Total",
            style="cyan",
            justify="left",
            width=20,
            min_width=10,
            max_width=30,
            no_wrap=False,
        )

        assert len(table.columns) == 1
        assert table.columns[0].header == "Name"

    def test_add_row_params(self):
        """Test add_row accepts style param."""
        from fast_rich.table import Table

        table = Table()
        table.add_column("Col")
        table.add_row("Value", style="bold")

        assert table.row_count == 1


class TestTableRendering:
    """Test table rendering output."""

    def test_table_str_output(self):
        """Test table renders to string."""
        from fast_rich.table import Table

        table = Table()
        table.add_column("Name")
        table.add_column("Value")
        table.add_row("foo", "bar")

        output = str(table)
        assert "Name" in output
        assert "Value" in output
        assert "foo" in output
        assert "bar" in output

    def test_empty_table(self):
        """Test empty table renders."""
        from fast_rich.table import Table

        table = Table()
        output = str(table)
        assert output == ""  # No columns = empty output
