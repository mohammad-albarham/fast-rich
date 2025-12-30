"""Highlighter classes - matches rich.highlighter API."""

from __future__ import annotations

import re
from abc import ABC, abstractmethod
from typing import Any, List, Optional, Pattern, Union

from fast_rich.text import Text


class Highlighter(ABC):
    """Abstract base class for highlighters.
    
    Matches rich.highlighter.Highlighter API.
    """

    @abstractmethod
    def highlight(self, text: Text) -> None:
        """Apply highlighting to text.
        
        Args:
            text: Text to highlight.
        """
        pass

    def __call__(self, text: Union[str, Text]) -> Text:
        """Highlight text.
        
        Args:
            text: Text to highlight.
            
        Returns:
            Highlighted Text.
        """
        if isinstance(text, str):
            text = Text(text)
        else:
            text = text.copy()
        self.highlight(text)
        return text


class NullHighlighter(Highlighter):
    """A highlighter that does nothing."""

    def highlight(self, text: Text) -> None:
        """No highlighting."""
        pass


class RegexHighlighter(Highlighter):
    """Apply styles based on regex patterns.
    
    Matches rich.highlighter.RegexHighlighter API.
    """

    base_style: str = ""
    highlights: List[str] = []

    def highlight(self, text: Text) -> None:
        """Apply regex-based highlighting."""
        plain = text.plain
        for pattern_str in self.highlights:
            # Extract style from pattern like r"(?P<style_name>pattern)"
            for match in re.finditer(pattern_str, plain):
                for group_name, group_value in match.groupdict().items():
                    if group_value is not None:
                        style = f"{self.base_style}.{group_name}" if self.base_style else group_name
                        text.stylize(style, match.start(group_name), match.end(group_name))


class ReprHighlighter(RegexHighlighter):
    """Highlight repr strings.
    
    Matches rich.highlighter.ReprHighlighter API.
    """

    base_style = "repr"
    highlights = [
        r"(?P<number>(?<!\w)\-?[0-9]+\.?[0-9]*(?!\w))",
        r"(?P<string>\'[^\']*\'|\"[^\"]*\")",
        r"(?P<none>(?<!\w)None(?!\w))",
        r"(?P<bool>(?<!\w)(True|False)(?!\w))",
        r"(?P<call>[\w\.]+(?=\())",
        r"(?P<uuid>[a-fA-F0-9]{8}\-[a-fA-F0-9]{4}\-[a-fA-F0-9]{4}\-[a-fA-F0-9]{4}\-[a-fA-F0-9]{12})",
    ]


class JSONHighlighter(RegexHighlighter):
    """Highlight JSON strings.
    
    Matches rich.highlighter.JSONHighlighter API.
    """

    base_style = "json"
    highlights = [
        r"(?P<number>(?<!\w)\-?[0-9]+\.?[0-9]*(?!\w))",
        r"(?P<string>\"[^\"]*\")",
        r"(?P<bool>(?<!\w)(true|false)(?!\w))",
        r"(?P<null>(?<!\w)null(?!\w))",
    ]


class ISO8601Highlighter(RegexHighlighter):
    """Highlight ISO8601 timestamps."""

    base_style = "iso8601"
    highlights = [
        r"(?P<date>\d{4}-\d{2}-\d{2})",
        r"(?P<time>\d{2}:\d{2}:\d{2})",
    ]


__all__ = [
    "Highlighter",
    "NullHighlighter",
    "RegexHighlighter",
    "ReprHighlighter",
    "JSONHighlighter",
    "ISO8601Highlighter",
]
