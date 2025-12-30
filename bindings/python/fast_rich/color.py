"""Color handling - matches rich.color API."""

from __future__ import annotations

from typing import Optional, Tuple, NamedTuple


class ColorTriplet(NamedTuple):
    """RGB color triplet."""
    red: int
    green: int
    blue: int

    @property
    def hex(self) -> str:
        """Get hex representation."""
        return f"#{self.red:02x}{self.green:02x}{self.blue:02x}"

    @property
    def rgb(self) -> str:
        """Get RGB string."""
        return f"rgb({self.red},{self.green},{self.blue})"

    @property
    def normalized(self) -> Tuple[float, float, float]:
        """Get normalized RGB (0-1)."""
        return (self.red / 255, self.green / 255, self.blue / 255)


# Standard color names to ANSI codes
COLOR_TO_ANSI = {
    "black": 0,
    "red": 1,
    "green": 2,
    "yellow": 3,
    "blue": 4,
    "magenta": 5,
    "cyan": 6,
    "white": 7,
    "bright_black": 8,
    "bright_red": 9,
    "bright_green": 10,
    "bright_yellow": 11,
    "bright_blue": 12,
    "bright_magenta": 13,
    "bright_cyan": 14,
    "bright_white": 15,
    "grey0": 16,
    "gray0": 16,
    "default": -1,
}


class Color:
    """Represents a color.
    
    Matches rich.color.Color API.
    """

    def __init__(
        self,
        name: str = "",
        *,
        number: Optional[int] = None,
        triplet: Optional[ColorTriplet] = None,
    ) -> None:
        """Create Color.
        
        Args:
            name: Color name.
            number: ANSI color number.
            triplet: RGB triplet.
        """
        self.name = name
        self.number = number
        self.triplet = triplet

    @classmethod
    def from_rgb(cls, red: int, green: int, blue: int) -> "Color":
        """Create color from RGB values.
        
        Args:
            red: Red component (0-255).
            green: Green component (0-255).
            blue: Blue component (0-255).
            
        Returns:
            Color instance.
        """
        triplet = ColorTriplet(red, green, blue)
        return cls(triplet.hex, triplet=triplet)

    @classmethod
    def parse(cls, color: str) -> "Color":
        """Parse a color string.
        
        Args:
            color: Color string (name, hex, or rgb).
            
        Returns:
            Color instance.
        """
        color = color.strip().lower()
        
        # Named color
        if color in COLOR_TO_ANSI:
            return cls(color, number=COLOR_TO_ANSI[color])
        
        # Hex color
        if color.startswith("#"):
            hex_str = color[1:]
            if len(hex_str) == 3:
                hex_str = "".join(c * 2 for c in hex_str)
            if len(hex_str) == 6:
                r = int(hex_str[0:2], 16)
                g = int(hex_str[2:4], 16)
                b = int(hex_str[4:6], 16)
                return cls.from_rgb(r, g, b)
        
        # RGB color: rgb(r,g,b)
        if color.startswith("rgb(") and color.endswith(")"):
            values = color[4:-1].split(",")
            if len(values) == 3:
                r, g, b = [int(v.strip()) for v in values]
                return cls.from_rgb(r, g, b)
        
        # Default - treat as named color
        return cls(color)

    @classmethod
    def default(cls) -> "Color":
        """Get default color."""
        return cls("default", number=-1)

    def __str__(self) -> str:
        """Return color name."""
        return self.name

    def __repr__(self) -> str:
        """Return repr."""
        return f"Color({self.name!r})"

    def __eq__(self, other: object) -> bool:
        """Check equality."""
        if isinstance(other, Color):
            return self.name == other.name
        return False

    def __hash__(self) -> int:
        """Get hash."""
        return hash(self.name)

    def get_ansi_codes(self, foreground: bool = True) -> str:
        """Get ANSI escape codes for this color.
        
        Args:
            foreground: If True, get foreground code; else background.
            
        Returns:
            ANSI escape codes.
        """
        base = 30 if foreground else 40
        
        if self.triplet:
            color_type = 38 if foreground else 48
            return f"{color_type};2;{self.triplet.red};{self.triplet.green};{self.triplet.blue}"
        
        if self.number is not None and self.number >= 0:
            if self.number < 8:
                return str(base + self.number)
            elif self.number < 16:
                return str(base + 60 + (self.number - 8))
            else:
                color_type = 38 if foreground else 48
                return f"{color_type};5;{self.number}"
        
        return ""


__all__ = ["Color", "ColorTriplet", "COLOR_TO_ANSI"]
