"""Table class - matches rich.table API."""

from __future__ import annotations

from typing import (
    Any,
    Iterable,
    List,
    Optional,
    Union,
)

from fast_rich.box import Box, ROUNDED
from fast_rich.style import Style
from fast_rich.text import Text


class Column:
    """A column in a table."""

    def __init__(
        self,
        header: str = "",
        footer: str = "",
        *,
        header_style: Optional[Union[str, Style]] = None,
        footer_style: Optional[Union[str, Style]] = None,
        style: Optional[Union[str, Style]] = None,
        justify: str = "left",
        vertical: str = "top",
        overflow: str = "ellipsis",
        width: Optional[int] = None,
        min_width: Optional[int] = None,
        max_width: Optional[int] = None,
        ratio: Optional[int] = None,
        no_wrap: bool = False,
    ) -> None:
        self.header = header
        self.footer = footer
        self.header_style = header_style
        self.footer_style = footer_style
        self.style = style
        self.justify = justify
        self.vertical = vertical
        self.overflow = overflow
        self.width = width
        self.min_width = min_width
        self.max_width = max_width
        self.ratio = ratio
        self.no_wrap = no_wrap


class Table:
    """A table that renders data in columns and rows.
    
    This is a drop-in replacement for rich.table.Table.
    """

    def __init__(
        self,
        *headers: str,
        title: Optional[str] = None,
        caption: Optional[str] = None,
        width: Optional[int] = None,
        min_width: Optional[int] = None,
        box: Optional[Box] = ROUNDED,
        safe_box: Optional[bool] = None,
        padding: Union[int, tuple] = (0, 1),
        collapse_padding: bool = False,
        pad_edge: bool = True,
        expand: bool = False,
        show_header: bool = True,
        show_footer: bool = False,
        show_edge: bool = True,
        show_lines: bool = False,
        leading: int = 0,
        style: Optional[Union[str, Style]] = None,
        row_styles: Optional[Iterable[Union[str, Style]]] = None,
        header_style: Optional[Union[str, Style]] = None,
        footer_style: Optional[Union[str, Style]] = None,
        border_style: Optional[Union[str, Style]] = None,
        title_style: Optional[Union[str, Style]] = None,
        caption_style: Optional[Union[str, Style]] = None,
        title_justify: str = "center",
        caption_justify: str = "center",
        highlight: bool = False,
    ) -> None:
        """Create a Table.
        
        Args:
            *headers: Column headers.
            title: Table title.
            caption: Table caption.
            width: Fixed width.
            min_width: Minimum width.
            box: Box style.
            safe_box: Use ASCII-safe box.
            padding: Cell padding.
            collapse_padding: Collapse padding.
            pad_edge: Pad edges.
            expand: Expand to fill width.
            show_header: Show header row.
            show_footer: Show footer row.
            show_edge: Show table edge.
            show_lines: Show row lines.
            leading: Leading space.
            style: Table style.
            row_styles: Alternating row styles.
            header_style: Header style.
            footer_style: Footer style.
            border_style: Border style.
            title_style: Title style.
            caption_style: Caption style.
            title_justify: Title justification.
            caption_justify: Caption justification.
            highlight: Highlight cells.
        """
        self.title = title
        self.caption = caption
        self.width = width
        self.min_width = min_width
        self.box = box
        self.safe_box = safe_box
        self.padding = padding
        self.collapse_padding = collapse_padding
        self.pad_edge = pad_edge
        self.expand = expand
        self.show_header = show_header
        self.show_footer = show_footer
        self.show_edge = show_edge
        self.show_lines = show_lines
        self.leading = leading
        self.style = style
        self.row_styles = list(row_styles) if row_styles else []
        self.header_style = header_style
        self.footer_style = footer_style
        self.border_style = border_style
        self.title_style = title_style
        self.caption_style = caption_style
        self.title_justify = title_justify
        self.caption_justify = caption_justify
        self.highlight = highlight

        self._columns: List[Column] = []
        self._rows: List[List[Any]] = []
            
        # Try to get Rust table
        try:
            from fast_rich._core import Table as RustTable
            self._rust_table = RustTable()
            self._use_rust = True
        except ImportError:
            self._rust_table = None
            self._use_rust = False

        # Add headers as columns
        for header in headers:
            self.add_column(header)

    @property
    def columns(self) -> List[Column]:
        """Get columns."""
        return self._columns

    @property
    def rows(self) -> List[List[Any]]:
        """Get rows."""
        return self._rows

    @property
    def row_count(self) -> int:
        """Get number of rows."""
        return len(self._rows)

    def add_column(
        self,
        header: str = "",
        footer: str = "",
        *,
        header_style: Optional[Union[str, Style]] = None,
        footer_style: Optional[Union[str, Style]] = None,
        style: Optional[Union[str, Style]] = None,
        justify: str = "left",
        vertical: str = "top",
        overflow: str = "ellipsis",
        width: Optional[int] = None,
        min_width: Optional[int] = None,
        max_width: Optional[int] = None,
        ratio: Optional[int] = None,
        no_wrap: bool = False,
    ) -> None:
        """Add a column to the table.
        
        Args:
            header: Column header text.
            footer: Column footer text.
            header_style: Style for header.
            footer_style: Style for footer.
            style: Style for cells.
            justify: Text justification.
            vertical: Vertical alignment.
            overflow: Overflow handling.
            width: Fixed width.
            min_width: Minimum width.
            max_width: Maximum width.
            ratio: Width ratio.
            no_wrap: Disable wrapping.
        """
        column = Column(
            header=header,
            footer=footer,
            header_style=header_style,
            footer_style=footer_style,
            style=style,
            justify=justify,
            vertical=vertical,
            overflow=overflow,
            width=width,
            min_width=min_width,
            max_width=max_width,
            ratio=ratio,
            no_wrap=no_wrap,
        )
        self._columns.append(column)
        
        if self._use_rust and self._rust_table:
            try:
                self._rust_table.add_column(header)
            except Exception:
                pass

    def add_row(
        self,
        *renderables: Any,
        style: Optional[Union[str, Style]] = None,
        end_section: bool = False,
    ) -> None:
        """Add a row of renderables.
        
        Args:
            *renderables: Cell values.
            style: Row style.
            end_section: End section after this row.
        """
        row = list(renderables)
        self._rows.append(row)
        
        if self._use_rust and self._rust_table:
            try:
                self._rust_table.add_row([str(r) for r in row])
            except Exception:
                pass

    def add_section(self) -> None:
        """Add a section separator."""
        pass  # Simplified

    def __str__(self) -> str:
        """Render table as string."""
        return self._render_simple()

    def __rich_console__(self, console: Any, options: Any) -> Any:
        """Rich console protocol."""
        yield self._render_simple()

    def _render_simple(self) -> str:
        """Simple table rendering."""
        if not self._columns:
            return ""

        # Calculate column widths
        col_widths = []
        for i, col in enumerate(self._columns):
            width = len(col.header)
            for row in self._rows:
                if i < len(row):
                    width = max(width, len(str(row[i])))
            col_widths.append(width + 2)  # padding

        # Build table
        lines = []
        box = self.box or ROUNDED

        # Title
        if self.title:
            total_width = sum(col_widths) + len(col_widths) + 1
            lines.append(self.title.center(total_width))

        # Top border
        if self.show_edge:
            lines.append(box.get_top(col_widths))

        # Header
        if self.show_header:
            header_cells = []
            for i, col in enumerate(self._columns):
                header_cells.append(f" {col.header.ljust(col_widths[i] - 2)} ")
            lines.append(box.mid_left + box.mid_vertical.join(header_cells) + box.mid_right)
            
            # Header divider
            lines.append(box.get_row(col_widths))

        # Rows
        for row in self._rows:
            cells = []
            for i, col in enumerate(self._columns):
                value = str(row[i]) if i < len(row) else ""
                cells.append(f" {value.ljust(col_widths[i] - 2)} ")
            lines.append(box.mid_left + box.mid_vertical.join(cells) + box.mid_right)

        # Bottom border
        if self.show_edge:
            lines.append(box.get_bottom(col_widths))

        # Caption
        if self.caption:
            total_width = sum(col_widths) + len(col_widths) + 1
            lines.append(self.caption.center(total_width))

        return "\n".join(lines)


__all__ = ["Table", "Column"]
