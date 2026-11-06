"""Markup parsing - matches rich.markup API."""

from __future__ import annotations

import re
from typing import List, Optional, Tuple, Union

from fast_rich.text import Text
from fast_rich.style import Style


# Regex for matching Rich markup tags
TAG_PATTERN = re.compile(r"\[(/?)([^\[\]]*?)\]")


def escape(text: str) -> str:
    """Escape markup characters in text.
    
    Args:
        text: Text with potential markup.
        
    Returns:
        Escaped text.
    """
    return text.replace("[", r"\[").replace("]", r"\]")


def _parse_tag(tag: str) -> Tuple[bool, str]:
    """Parse a markup tag.
    
    Args:
        tag: Tag content (without brackets).
        
    Returns:
        Tuple of (is_closing, style_name).
    """
    is_closing = tag.startswith("/")
    style_name = tag[1:] if is_closing else tag
    return is_closing, style_name


def render(
    markup: str,
    *,
    style: Optional[Union[str, Style]] = None,
    emoji: bool = True,
    emoji_variant: Optional[str] = None,
) -> Text:
    """Render markup string to Text.
    
    Args:
        markup: Markup string like "[bold]Hello[/]".
        style: Base style to apply.
        emoji: Enable emoji shortcodes.
        emoji_variant: Emoji variant.
        
    Returns:
        Text object with styles applied.
    """
    text = Text()
    style_stack: List[str] = []
    last_end = 0
    
    if style:
        style_stack.append(str(style) if not isinstance(style, str) else style)
    
    for match in TAG_PATTERN.finditer(markup):
        # Add text before tag
        if match.start() > last_end:
            plain_text = markup[last_end:match.start()]
            # Unescape
            plain_text = plain_text.replace(r"\[", "[").replace(r"\]", "]")
            current_style = " ".join(style_stack) if style_stack else None
            text.append(plain_text, style=current_style)
        
        is_closing, style_name = _parse_tag(match.group(0)[1:-1])
        
        if is_closing:
            # Pop style from stack
            if style_name:
                # Pop specific style
                for i in range(len(style_stack) - 1, -1, -1):
                    if style_stack[i] == style_name:
                        style_stack.pop(i)
                        break
            elif style_stack:
                # Pop last style
                style_stack.pop()
        else:
            # Push style to stack
            if style_name:
                style_stack.append(style_name)
        
        last_end = match.end()
    
    # Add remaining text
    if last_end < len(markup):
        plain_text = markup[last_end:]
        plain_text = plain_text.replace(r"\[", "[").replace(r"\]", "]")
        current_style = " ".join(style_stack) if style_stack else None
        text.append(plain_text, style=current_style)
    
    # Handle emoji if enabled
    if emoji:
        from fast_rich.emoji import replace as emoji_replace
        text._plain = emoji_replace(text._plain)
    
    return text


class MarkupError(Exception):
    """Exception for markup parsing errors."""
    pass


__all__ = ["escape", "render", "MarkupError"]
