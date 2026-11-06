"""Control codes - matches rich.control API."""

from __future__ import annotations

from typing import List, Tuple, TYPE_CHECKING

if TYPE_CHECKING:
    from fast_rich.segment import Segment


class Control:
    """A renderable that generates control codes.
    
    Matches rich.control.Control API.
    """

    BELL = "\x07"
    CARRIAGE_RETURN = "\r"
    HOME = "\x1b[H"
    CLEAR = "\x1b[2J"
    CLEAR_LINE = "\x1b[2K"
    SHOW_CURSOR = "\x1b[?25h"
    HIDE_CURSOR = "\x1b[?25l"
    ENABLE_ALT_SCREEN = "\x1b[?1049h\x1b[2J"
    DISABLE_ALT_SCREEN = "\x1b[?1049l"

    def __init__(self, *controls: str) -> None:
        """Create Control.
        
        Args:
            *controls: Control sequences.
        """
        self._controls = controls

    @classmethod
    def bell(cls) -> "Control":
        """Ring the terminal bell."""
        return cls(cls.BELL)

    @classmethod
    def home(cls) -> "Control":
        """Move cursor to home position."""
        return cls(cls.HOME)

    @classmethod
    def clear(cls) -> "Control":
        """Clear the screen."""
        return cls(cls.CLEAR)

    @classmethod
    def clear_line(cls) -> "Control":
        """Clear the current line."""
        return cls(cls.CLEAR_LINE)

    @classmethod
    def show_cursor(cls, show: bool = True) -> "Control":
        """Show or hide cursor."""
        return cls(cls.SHOW_CURSOR if show else cls.HIDE_CURSOR)

    @classmethod
    def alt_screen(cls, enable: bool = True) -> "Control":
        """Enable or disable alternate screen."""
        return cls(cls.ENABLE_ALT_SCREEN if enable else cls.DISABLE_ALT_SCREEN)

    @classmethod
    def cursor_up(cls, n: int = 1) -> "Control":
        """Move cursor up."""
        return cls(f"\x1b[{n}A")

    @classmethod
    def cursor_down(cls, n: int = 1) -> "Control":
        """Move cursor down."""
        return cls(f"\x1b[{n}B")

    @classmethod
    def cursor_forward(cls, n: int = 1) -> "Control":
        """Move cursor forward."""
        return cls(f"\x1b[{n}C")

    @classmethod
    def cursor_backward(cls, n: int = 1) -> "Control":
        """Move cursor backward."""
        return cls(f"\x1b[{n}D")

    @classmethod
    def cursor_move_to(cls, x: int = 0, y: int = 0) -> "Control":
        """Move cursor to position."""
        return cls(f"\x1b[{y + 1};{x + 1}H")

    @classmethod
    def cursor_move_to_column(cls, x: int = 0) -> "Control":
        """Move cursor to column."""
        return cls(f"\x1b[{x + 1}G")

    @classmethod
    def erase_in_line(cls, mode: int = 0) -> "Control":
        """Erase in line."""
        return cls(f"\x1b[{mode}K")

    @classmethod
    def title(cls, title: str) -> "Control":
        """Set terminal title."""
        return cls(f"\x1b]0;{title}\x07")

    def __str__(self) -> str:
        """Return control sequence."""
        return "".join(self._controls)

    def __rich_console__(self, console, options):
        """Rich console protocol."""
        from fast_rich.segment import Segment
        for control in self._controls:
            yield Segment(control, control=True)


def strip_control_codes(text: str) -> str:
    """Remove control codes from text.
    
    Args:
        text: Text with control codes.
        
    Returns:
        Text without control codes.
    """
    import re
    return re.sub(r"\x1b\[[0-9;]*[A-Za-z]|\x1b\][^\x07]*\x07", "", text)


__all__ = ["Control", "strip_control_codes"]
