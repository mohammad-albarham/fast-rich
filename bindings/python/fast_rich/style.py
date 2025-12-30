"""Style class - matches rich.style API."""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Optional, Union


@dataclass
class Style:
    """A terminal style.
    
    Matches the rich.style.Style API.
    """
    
    color: Optional[str] = None
    bgcolor: Optional[str] = None
    bold: Optional[bool] = None
    dim: Optional[bool] = None
    italic: Optional[bool] = None
    underline: Optional[bool] = None
    blink: Optional[bool] = None
    blink2: Optional[bool] = None
    reverse: Optional[bool] = None
    conceal: Optional[bool] = None
    strike: Optional[bool] = None
    underline2: Optional[bool] = None
    frame: Optional[bool] = None
    encircle: Optional[bool] = None
    overline: Optional[bool] = None
    link: Optional[str] = None
    meta: Optional[dict] = None

    def __init__(
        self,
        color: Optional[str] = None,
        bgcolor: Optional[str] = None,
        bold: Optional[bool] = None,
        dim: Optional[bool] = None,
        italic: Optional[bool] = None,
        underline: Optional[bool] = None,
        blink: Optional[bool] = None,
        blink2: Optional[bool] = None,
        reverse: Optional[bool] = None,
        conceal: Optional[bool] = None,
        strike: Optional[bool] = None,
        underline2: Optional[bool] = None,
        frame: Optional[bool] = None,
        encircle: Optional[bool] = None,
        overline: Optional[bool] = None,
        link: Optional[str] = None,
        meta: Optional[dict] = None,
    ) -> None:
        self.color = color
        self.bgcolor = bgcolor
        self.bold = bold
        self.dim = dim
        self.italic = italic
        self.underline = underline
        self.blink = blink
        self.blink2 = blink2
        self.reverse = reverse
        self.conceal = conceal
        self.strike = strike
        self.underline2 = underline2
        self.frame = frame
        self.encircle = encircle
        self.overline = overline
        self.link = link
        self.meta = meta or {}

    @classmethod
    def parse(cls, style_definition: str) -> "Style":
        """Parse a style definition string.
        
        Args:
            style_definition: A string like "bold red on blue"
            
        Returns:
            A Style instance.
        """
        style = cls()
        parts = style_definition.lower().split()
        
        on_background = False
        for part in parts:
            if part == "on":
                on_background = True
                continue
            
            # Check for attributes
            if part == "bold":
                style.bold = True
            elif part == "dim":
                style.dim = True
            elif part == "italic":
                style.italic = True
            elif part == "underline":
                style.underline = True
            elif part == "blink":
                style.blink = True
            elif part == "reverse":
                style.reverse = True
            elif part == "strike":
                style.strike = True
            elif part == "conceal":
                style.conceal = True
            elif on_background:
                style.bgcolor = part
                on_background = False
            else:
                # Assume it's a color
                style.color = part
        
        return style

    def __add__(self, other: Optional["Style"]) -> "Style":
        """Combine styles."""
        if other is None:
            return self
        return Style(
            color=other.color or self.color,
            bgcolor=other.bgcolor or self.bgcolor,
            bold=other.bold if other.bold is not None else self.bold,
            dim=other.dim if other.dim is not None else self.dim,
            italic=other.italic if other.italic is not None else self.italic,
            underline=other.underline if other.underline is not None else self.underline,
            blink=other.blink if other.blink is not None else self.blink,
            reverse=other.reverse if other.reverse is not None else self.reverse,
            strike=other.strike if other.strike is not None else self.strike,
            conceal=other.conceal if other.conceal is not None else self.conceal,
            link=other.link or self.link,
        )

    def __bool__(self) -> bool:
        """Check if style has any attributes set."""
        return any([
            self.color, self.bgcolor, self.bold, self.dim, self.italic,
            self.underline, self.blink, self.reverse, self.strike, self.conceal,
            self.link
        ])

    def __str__(self) -> str:
        """Convert to style string."""
        parts = []
        if self.bold:
            parts.append("bold")
        if self.dim:
            parts.append("dim")
        if self.italic:
            parts.append("italic")
        if self.underline:
            parts.append("underline")
        if self.blink:
            parts.append("blink")
        if self.reverse:
            parts.append("reverse")
        if self.strike:
            parts.append("strike")
        if self.color:
            parts.append(self.color)
        if self.bgcolor:
            parts.append(f"on {self.bgcolor}")
        return " ".join(parts) if parts else "none"


# Convenience styles
NULL_STYLE = Style()


__all__ = ["Style", "NULL_STYLE"]
