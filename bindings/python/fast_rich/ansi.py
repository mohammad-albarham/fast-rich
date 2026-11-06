"""ANSI parser - matches rich.ansi API."""

from __future__ import annotations

import re
from typing import Iterable, Optional

from fast_rich.style import Style
from fast_rich.text import Text


# ANSI escape sequence pattern
ANSI_PATTERN = re.compile(r"\x1b\[([0-9;]*)m")


class AnsiDecoder:
    """Decode ANSI escape sequences.
    
    Matches rich.ansi.AnsiDecoder API.
    """

    def __init__(self) -> None:
        """Create AnsiDecoder."""
        self._style = Style()

    def decode(self, terminal_text: str) -> Iterable[Text]:
        """Decode ANSI text to Rich Text.
        
        Args:
            terminal_text: Text with ANSI escape codes.
            
        Yields:
            Text objects for each line.
        """
        for line in terminal_text.splitlines():
            yield self.decode_line(line)

    def decode_line(self, line: str) -> Text:
        """Decode a single line.
        
        Args:
            line: Line with ANSI codes.
            
        Returns:
            Text object.
        """
        text = Text()
        last_end = 0
        
        for match in ANSI_PATTERN.finditer(line):
            # Add text before the escape sequence
            if match.start() > last_end:
                text.append(line[last_end:match.start()])
            
            # Parse ANSI codes
            codes = match.group(1).split(";") if match.group(1) else ["0"]
            self._apply_codes(codes)
            
            last_end = match.end()
        
        # Add remaining text
        if last_end < len(line):
            text.append(line[last_end:])
        
        return text

    def _apply_codes(self, codes: list) -> None:
        """Apply ANSI codes to current style."""
        for code in codes:
            if code == "0":
                self._style = Style()
            elif code == "1":
                self._style = Style(bold=True)
            elif code == "3":
                self._style = Style(italic=True)
            elif code == "4":
                self._style = Style(underline=True)
            # Add more codes as needed


def strip_ansi(text: str) -> str:
    """Remove ANSI escape sequences from text.
    
    Args:
        text: Text with ANSI sequences.
        
    Returns:
        Plain text.
    """
    return ANSI_PATTERN.sub("", text)


__all__ = ["AnsiDecoder", "strip_ansi"]
