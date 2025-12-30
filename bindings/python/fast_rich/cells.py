"""Cell width utilities - matches rich.cells API."""

from __future__ import annotations

import unicodedata
from functools import lru_cache


@lru_cache(maxsize=4096)
def cell_len(text: str) -> int:
    """Get the cell width of text.
    
    Args:
        text: Text to measure.
        
    Returns:
        Cell width (accounting for wide characters).
    """
    width = 0
    for char in text:
        if char in ("\r", "\n"):
            continue
        char_width = _char_width(char)
        width += char_width
    return width


def _char_width(char: str) -> int:
    """Get cell width of a single character."""
    # Control characters
    if ord(char) < 32 or ord(char) == 127:
        return 0
    
    # Get East Asian Width
    ea_width = unicodedata.east_asian_width(char)
    
    # Wide characters (W, F)
    if ea_width in ("W", "F"):
        return 2
    
    # Narrow, Half-width, Neutral, Ambiguous
    return 1


def set_cell_size(text: str, total: int) -> str:
    """Set the cell width of text by padding or truncating.
    
    Args:
        text: Text to resize.
        total: Target cell width.
        
    Returns:
        Resized text.
    """
    current_len = cell_len(text)
    
    if current_len == total:
        return text
    elif current_len < total:
        # Pad with spaces
        return text + " " * (total - current_len)
    else:
        # Truncate
        result = []
        width = 0
        for char in text:
            char_width = _char_width(char)
            if width + char_width > total:
                break
            result.append(char)
            width += char_width
        # Pad if we truncated a wide character
        if width < total:
            result.append(" " * (total - width))
        return "".join(result)


def chop_cells(text: str, width: int) -> list:
    """Chop text into lines of given cell width.
    
    Args:
        text: Text to chop.
        width: Maximum cell width per line.
        
    Returns:
        List of lines.
    """
    lines = []
    current_line = []
    current_width = 0
    
    for char in text:
        char_width = _char_width(char)
        if current_width + char_width > width:
            lines.append("".join(current_line))
            current_line = []
            current_width = 0
        current_line.append(char)
        current_width += char_width
    
    if current_line:
        lines.append("".join(current_line))
    
    return lines


__all__ = ["cell_len", "set_cell_size", "chop_cells"]
