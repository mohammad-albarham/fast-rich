"""Text class - matches rich.text API."""

from __future__ import annotations

from typing import Optional, Union, Iterable, List, Tuple

from fast_rich.style import Style


class Text:
    """A piece of text with optional styling.
    
    Matches the rich.text.Text API.
    """

    def __init__(
        self,
        text: str = "",
        style: Optional[Union[str, Style]] = None,
        *,
        justify: Optional[str] = None,
        overflow: Optional[str] = None,
        no_wrap: Optional[bool] = None,
        end: str = "\n",
        tab_size: Optional[int] = 8,
        spans: Optional[List[Tuple[int, int, Style]]] = None,
    ) -> None:
        self._text = text
        self._style = style if isinstance(style, Style) else (Style.parse(style) if style else None)
        self.justify = justify
        self.overflow = overflow
        self.no_wrap = no_wrap
        self.end = end
        self.tab_size = tab_size
        self._spans: List[Tuple[int, int, Style]] = spans or []

    @property
    def plain(self) -> str:
        """Get the text without any formatting."""
        return self._text

    @plain.setter
    def plain(self, value: str) -> None:
        """Set the plain text."""
        self._text = value

    @property
    def style(self) -> Optional[Style]:
        """Get the base style."""
        return self._style

    @style.setter
    def style(self, value: Optional[Union[str, Style]]) -> None:
        """Set the base style."""
        self._style = value if isinstance(value, Style) else (Style.parse(value) if value else None)

    def __len__(self) -> int:
        """Return the length of the text."""
        return len(self._text)

    def __str__(self) -> str:
        """Return the plain text."""
        return self._text

    def __repr__(self) -> str:
        """Return repr."""
        return f"Text({self._text!r}, style={self._style!r})"

    def __add__(self, other: Union["Text", str]) -> "Text":
        """Concatenate text."""
        if isinstance(other, str):
            return Text(self._text + other, self._style)
        return Text(self._text + other._text)

    def __iadd__(self, other: Union["Text", str]) -> "Text":
        """In-place concatenate."""
        if isinstance(other, str):
            self._text += other
        else:
            self._text += other._text
        return self

    @classmethod
    def from_markup(cls, text: str, style: Optional[Union[str, Style]] = None) -> "Text":
        """Create a Text object from Rich markup.
        
        Args:
            text: Text with Rich markup like [bold]Hello[/bold]
            style: Optional base style.
            
        Returns:
            A Text instance.
        """
        # For now, delegate to Rust for actual markup parsing
        # This is a simplified implementation
        return cls(text, style)

    @classmethod
    def assemble(
        cls,
        *parts: Union[str, Tuple[str, Optional[Union[str, Style]]], "Text"],
        style: Optional[Union[str, Style]] = None,
    ) -> "Text":
        """Construct a Text object from multiple parts.
        
        Args:
            *parts: Strings, (string, style) tuples, or Text objects.
            style: Optional base style.
            
        Returns:
            A Text instance.
        """
        result = cls("", style)
        for part in parts:
            if isinstance(part, str):
                result._text += part
            elif isinstance(part, tuple):
                text_part, part_style = part
                # TODO: Add span tracking
                result._text += str(text_part)
            elif isinstance(part, Text):
                result._text += part._text
        return result

    def append(
        self,
        text: Union[str, "Text"],
        style: Optional[Union[str, Style]] = None,
    ) -> "Text":
        """Append text.
        
        Args:
            text: Text to append.
            style: Optional style for the appended text.
            
        Returns:
            Self for chaining.
        """
        if isinstance(text, str):
            start = len(self._text)
            self._text += text
            if style:
                end = len(self._text)
                parsed_style = style if isinstance(style, Style) else Style.parse(style)
                self._spans.append((start, end, parsed_style))
        else:
            self._text += text._text
        return self

    def stylize(
        self,
        style: Union[str, Style],
        start: int = 0,
        end: Optional[int] = None,
    ) -> None:
        """Apply a style to a range of text.
        
        Args:
            style: Style to apply.
            start: Start index.
            end: End index (defaults to end of text).
        """
        if end is None:
            end = len(self._text)
        parsed_style = style if isinstance(style, Style) else Style.parse(style)
        self._spans.append((start, end, parsed_style))

    def split(
        self,
        separator: str = "\n",
        include_separator: bool = False,
        allow_blank: bool = False,
    ) -> List["Text"]:
        """Split text on a separator.
        
        Args:
            separator: String to split on.
            include_separator: Include separator in results.
            allow_blank: Allow empty strings.
            
        Returns:
            List of Text objects.
        """
        parts = self._text.split(separator)
        if not allow_blank:
            parts = [p for p in parts if p]
        return [Text(p, self._style) for p in parts]

    def copy(self) -> "Text":
        """Return a copy of the Text."""
        return Text(
            self._text,
            self._style,
            justify=self.justify,
            overflow=self.overflow,
            no_wrap=self.no_wrap,
            end=self.end,
            tab_size=self.tab_size,
            spans=self._spans.copy(),
        )


__all__ = ["Text"]
