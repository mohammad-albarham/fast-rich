"""Theme definitions - matches rich.themes API."""

from __future__ import annotations

from typing import Dict

from fast_rich.theme import Theme


# Default theme used by Console
DEFAULT = Theme({
    # General
    "none": "",
    "reset": "",
    "dim": "dim",
    "bright": "bright",
    "bold": "bold",
    "italic": "italic",
    "underline": "underline",
    "blink": "blink",
    "reverse": "reverse",
    "strike": "strike",
    
    # Repr
    "repr.str": "green",
    "repr.number": "cyan bold",
    "repr.bool_true": "bright green italic",
    "repr.bool_false": "bright red italic",
    "repr.none": "bright magenta italic",
    "repr.url": "underline bright blue",
    
    # Rule
    "rule.line": "bright green",
    "rule.text": "bright green",
    
    # JSON
    "json.key": "cyan bold",
    "json.str": "green",
    "json.number": "cyan bold",
    "json.null": "bright magenta italic",
    
    # Markdown
    "markdown.h1": "bold underline",
    "markdown.h2": "bold",
    "markdown.code": "cyan",
    "markdown.link": "bright blue underline",
    
    # Logging
    "logging.level.debug": "bright blue",
    "logging.level.info": "bright green",
    "logging.level.warning": "bright yellow",
    "logging.level.error": "bright red",
    "logging.level.critical": "bold bright red",
    
    # Progress
    "bar.complete": "bright_green",
    "bar.finished": "bright_green",
    "progress.spinner": "green",
    
    # Table
    "table.header": "bold",
    "table.title": "italic",
    
    # Tree
    "tree.line": "bright_blue",
    
    # Traceback
    "traceback.border": "red",
    "traceback.title": "bold red",
    
    # Inspect
    "inspect.attr": "yellow",
    "inspect.callable": "bold cyan",
    
    # Status
    "status.spinner": "green",
    
    # Prompt
    "prompt": "bold",
    "prompt.default": "dim italic",
})


# SVG export theme (dark background)
SVG_EXPORT_THEME = Theme({
    "none": "",
    "repr.str": "#98c379",
    "repr.number": "#56b6c2",
    "repr.bool_true": "#56b6c2",
    "repr.bool_false": "#e06c75",
    "repr.none": "#c678dd",
})


__all__ = ["DEFAULT", "SVG_EXPORT_THEME"]
