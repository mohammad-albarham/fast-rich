"""Default styles - matches rich.default_styles API."""

from __future__ import annotations

from typing import Dict

from fast_rich.style import Style


# Default style definitions matching rich.default_styles
DEFAULT_STYLES: Dict[str, str] = {
    # General
    "none": "",
    "reset": "",
    "dim": "dim",
    "bright": "bright",
    "bold": "bold",
    "italic": "italic",
    "underline": "underline",
    "blink": "blink",
    "blink2": "blink",
    "reverse": "reverse",
    "conceal": "conceal",
    "strike": "strike",
    
    # Console
    "repr.indent": "dim green",
    "repr.str": "green",
    "repr.brace": "bold",
    "repr.comma": "bold",
    "repr.ipv4": "bright green",
    "repr.ipv6": "bright green",
    "repr.eui48": "bright green",
    "repr.eui64": "bright green",
    "repr.tag_start": "bold",
    "repr.tag_name": "bright magenta",
    "repr.tag_contents": "",
    "repr.tag_end": "bold",
    "repr.attrib_name": "yellow",
    "repr.attrib_equal": "bold",
    "repr.attrib_value": "magenta",
    "repr.number": "cyan bold",
    "repr.number_complex": "cyan bold",
    "repr.bool_true": "bright green italic",
    "repr.bool_false": "bright red italic",
    "repr.none": "bright magenta italic",
    "repr.url": "underline bright blue",
    "repr.uuid": "bright yellow",
    "repr.call": "cyan bold",
    "repr.path": "magenta",
    "repr.filename": "bright magenta",
    "repr.ellipsis": "yellow",
    
    # Rule
    "rule.line": "bright green",
    "rule.text": "bright green",
    
    # JSON
    "json.brace": "bold",
    "json.bracket": "bold",
    "json.colon": "bold",
    "json.comma": "bold",
    "json.key": "cyan bold",
    "json.null": "bright magenta italic",
    "json.bool_true": "bright green italic",
    "json.bool_false": "bright red italic",
    "json.str": "green",
    "json.number": "cyan bold",
    
    # Markdown
    "markdown.h1": "bold underline",
    "markdown.h2": "bold",
    "markdown.h3": "bold dim",
    "markdown.h4": "italic",
    "markdown.h5": "italic dim",
    "markdown.h6": "italic dim",
    "markdown.bold": "bold",
    "markdown.italic": "italic",
    "markdown.bold_italic": "bold italic",
    "markdown.code": "cyan",
    "markdown.code_block": "",
    "markdown.block_quote": "magenta",
    "markdown.list": "cyan",
    "markdown.link": "bright blue underline",
    "markdown.link_url": "blue underline",
    "markdown.hr": "yellow",
    
    # Logging
    "logging.level.notset": "dim",
    "logging.level.debug": "bright blue",
    "logging.level.info": "bright green",
    "logging.level.warning": "bright yellow",
    "logging.level.error": "bright red",
    "logging.level.critical": "bold bright red",
    
    # Progress
    "bar.back": "grey23",
    "bar.complete": "bright_green",
    "bar.finished": "bright_green",
    "bar.pulse": "bright_yellow",
    "progress.description": "none",
    "progress.filesize": "green",
    "progress.filesize.total": "green",
    "progress.download": "green",
    "progress.elapsed": "yellow",
    "progress.percentage": "magenta",
    "progress.remaining": "cyan",
    "progress.data.speed": "red",
    "progress.spinner": "green",
    
    # Table
    "table.header": "bold",
    "table.footer": "bold",
    "table.cell": "",
    "table.row_odd": "",
    "table.row_even": "",
    "table.title": "italic",
    "table.caption": "dim italic",
    
    # Tree
    "tree": "",
    "tree.line": "bright_blue",
    
    # Traceback
    "traceback.border": "red",
    "traceback.border.syntax_error": "bright_red",
    "traceback.text": "",
    "traceback.title": "bold red",
    "traceback.error": "bright_red",
    "traceback.exc_type": "bright_red",
    
    # Inspect
    "inspect.attr": "yellow",
    "inspect.attr.dunder": "yellow dim",
    "inspect.callable": "bold cyan",
    "inspect.async_def": "cyan italic",
    "inspect.def": "cyan",
    "inspect.class": "bold cyan",
    "inspect.error": "bold red",
    "inspect.equals": "",
    "inspect.doc": "",
    "inspect.value.border": "green",
    
    # Status
    "status.spinner": "green",
    
    # Prompt
    "prompt": "bold",
    "prompt.choices": "dim",
    "prompt.default": "dim italic",
    "prompt.invalid": "prompt.invalid",
    
    # Scope
    "scope.border": "cyan",
    "scope.key": "yellow italic",
    "scope.key.special": "yellow italic dim",
    "scope.equals": "cyan",
}


def get_default_styles() -> Dict[str, Style]:
    """Get dictionary of default styles.
    
    Returns:
        Dictionary of style name to Style.
    """
    return {name: Style.parse(definition) for name, definition in DEFAULT_STYLES.items()}


__all__ = ["DEFAULT_STYLES", "get_default_styles"]
