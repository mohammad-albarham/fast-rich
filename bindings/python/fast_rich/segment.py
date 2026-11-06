"""Segment class - matches rich.segment API."""

from __future__ import annotations

from typing import Any, Iterable, List, NamedTuple, Optional, Tuple, Union

from fast_rich.style import Style


class Segment(NamedTuple):
    """A segment of text with optional style and control code.
    
    Matches rich.segment.Segment API.
    """

    text: str
    style: Optional[Style] = None
    control: bool = False

    @classmethod
    def line(cls) -> "Segment":
        """Create a newline segment."""
        return cls("\n")

    @classmethod
    def apply_style(
        cls,
        segments: Iterable["Segment"],
        style: Optional[Style] = None,
    ) -> Iterable["Segment"]:
        """Apply a style to segments.
        
        Args:
            segments: Segments to style.
            style: Style to apply.
            
        Yields:
            Styled segments.
        """
        if style is None:
            yield from segments
            return
        for segment in segments:
            yield cls(segment.text, style + segment.style if segment.style else style, segment.control)

    @classmethod
    def filter_control(
        cls,
        segments: Iterable["Segment"],
        filter_control: bool = True,
    ) -> Iterable["Segment"]:
        """Filter control segments.
        
        Args:
            segments: Segments to filter.
            filter_control: If True, remove control segments.
            
        Yields:
            Filtered segments.
        """
        for segment in segments:
            if not filter_control or not segment.control:
                yield segment

    @classmethod
    def split_lines(
        cls,
        segments: Iterable["Segment"],
    ) -> Iterable[List["Segment"]]:
        """Split segments into lines.
        
        Args:
            segments: Segments to split.
            
        Yields:
            Lines of segments.
        """
        line: List[Segment] = []
        for segment in segments:
            text = segment.text
            while True:
                newline_idx = text.find("\n")
                if newline_idx == -1:
                    if text:
                        line.append(cls(text, segment.style, segment.control))
                    break
                if newline_idx:
                    line.append(cls(text[:newline_idx], segment.style, segment.control))
                yield line
                line = []
                text = text[newline_idx + 1:]
        if line:
            yield line

    @classmethod
    def strip_links(cls, segments: Iterable["Segment"]) -> Iterable["Segment"]:
        """Strip links from segments."""
        yield from segments

    @classmethod
    def strip_styles(cls, segments: Iterable["Segment"]) -> Iterable["Segment"]:
        """Strip styles from segments."""
        for segment in segments:
            yield cls(segment.text, None, segment.control)

    @classmethod
    def get_line_length(cls, segments: Iterable["Segment"]) -> int:
        """Get length of segments."""
        return sum(len(seg.text) for seg in segments if not seg.control)

    @classmethod
    def get_shape(
        cls,
        segments: List[List["Segment"]],
    ) -> Tuple[int, int]:
        """Get shape (width, height) of segment lines."""
        height = len(segments)
        width = max((cls.get_line_length(line) for line in segments), default=0)
        return (width, height)


class ControlType:
    """Control type constants."""
    BELL = "bell"
    CARRIAGE_RETURN = "carriage_return"
    HOME = "home"
    CLEAR = "clear"
    SHOW_CURSOR = "show_cursor"
    HIDE_CURSOR = "hide_cursor"
    ENABLE_ALT_SCREEN = "enable_alt_screen"
    DISABLE_ALT_SCREEN = "disable_alt_screen"
    CURSOR_UP = "cursor_up"
    CURSOR_DOWN = "cursor_down"
    CURSOR_FORWARD = "cursor_forward"
    CURSOR_BACKWARD = "cursor_backward"
    CURSOR_MOVE_TO_COLUMN = "cursor_move_to_column"
    CURSOR_MOVE_TO = "cursor_move_to"
    ERASE_IN_LINE = "erase_in_line"
    SET_WINDOW_TITLE = "set_window_title"


class Segments:
    """A collection of segments."""

    def __init__(self, segments: Iterable[Segment], new_lines: bool = False) -> None:
        self._segments = list(segments)
        self.new_lines = new_lines

    def __iter__(self):
        return iter(self._segments)

    def __len__(self):
        return len(self._segments)


__all__ = ["Segment", "Segments", "ControlType"]
