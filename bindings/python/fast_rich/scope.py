"""Scope context - matches rich.scope API."""

from __future__ import annotations

from typing import Any, Dict, Mapping, Optional

from fast_rich.console import Console
from fast_rich.text import Text


def render_scope(
    scope: Mapping[str, Any],
    *,
    title: Optional[str] = None,
    sort_keys: bool = True,
    indent_guides: bool = False,
    max_length: Optional[int] = None,
    max_string: Optional[int] = None,
) -> Any:
    """Render a scope (dictionary of variables).
    
    Args:
        scope: Dictionary of names to values.
        title: Optional title.
        sort_keys: Sort keys alphabetically.
        indent_guides: Show indent guides.
        max_length: Max collection length.
        max_string: Max string length.
        
    Returns:
        Renderable scope representation.
    """
    from fast_rich.panel import Panel
    
    lines = []
    keys = sorted(scope.keys()) if sort_keys else list(scope.keys())
    
    for key in keys:
        value = scope[key]
        value_repr = repr(value)
        if max_string and len(value_repr) > max_string:
            value_repr = value_repr[:max_string] + "..."
        lines.append(f"{key} = {value_repr}")
    
    content = "\n".join(lines)
    if title:
        return Panel(content, title=title)
    return content


__all__ = ["render_scope"]
