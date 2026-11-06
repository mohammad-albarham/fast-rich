"""Bridge between the Rich object model and the Rust renderer.

Text and the layout renderables (Panel, Table, Tree, Rule, Align, Padding,
Constrain, Columns, Styled, Markdown, Syntax) are converted to a *spec*: a
tuple whose first item names the kind. ``fast_rich._core.render`` builds the
matching Rust renderable, lays it out at the requested width and returns
lines of ``(text, style, link)``, which are turned back into Rich
``Segment`` objects here. Objects that only implement Rich's
``__rich_console__`` protocol become a ``("callback", obj)`` spec; Rust calls
``Console._render_callback`` for them at render time.
"""

from __future__ import annotations

import threading
from functools import lru_cache
from typing import TYPE_CHECKING, Any, Iterable, List, Optional, Tuple

from . import _core
from .color import ColorType
from .segment import Segment
from .style import Style

if TYPE_CHECKING:  # pragma: no cover
    from .console import Console, ConsoleOptions

Spec = Tuple[Any, ...]

_STANDARD_NAMES = (
    "black",
    "red",
    "green",
    "yellow",
    "blue",
    "magenta",
    "cyan",
    "white",
    "bright_black",
    "bright_red",
    "bright_green",
    "bright_yellow",
    "bright_blue",
    "bright_magenta",
    "bright_cyan",
    "bright_white",
)

_ATTRIBUTES = (
    ("bold", "bold"),
    ("dim", "dim"),
    ("italic", "italic"),
    ("underline", "underline"),
    ("blink", "blink"),
    ("reverse", "reverse"),
    ("conceal", "hidden"),
    ("strike", "strike"),
)

_local = threading.local()


def _color(color: Any) -> str:
    if color.type == ColorType.DEFAULT:
        return "default"
    if color.type in (ColorType.STANDARD, ColorType.WINDOWS):
        return _STANDARD_NAMES[color.number]
    if color.type == ColorType.EIGHT_BIT:
        return f"color({color.number})"
    return color.triplet.hex


@lru_cache(maxsize=4096)
def native_style(style: Optional[Style]) -> str:
    """Rich ``Style`` -> style string understood by the Rust parser."""
    if not style:
        return ""
    parts = [name for attr, name in _ATTRIBUTES if getattr(style, attr)]
    if style.color is not None:
        parts.append(_color(style.color))
    if style.bgcolor is not None:
        parts.append("on " + _color(style.bgcolor))
    return " ".join(parts)


@lru_cache(maxsize=4096)
def rich_style(style: str, link: Optional[str]) -> Optional[Style]:
    """Style string from Rust -> Rich ``Style`` (``None`` when empty)."""
    parsed = Style.parse(style) if style else None
    if link:
        linked = Style(link=link)
        parsed = linked if parsed is None else parsed + linked
    return parsed


def _style_of(console: "Console", style: Any) -> Style:
    if isinstance(style, Style):
        return style
    return console.get_style(style or "", default=Style.null())


def is_native(renderable: Any) -> bool:
    """True when the Rust renderer draws this object."""
    from .text import Text

    return isinstance(renderable, Text) or (
        hasattr(type(renderable), "__fast_rich__") and not isinstance(renderable, type)
    )


def text_spec(console: "Console", options: "ConsoleOptions", text: Any) -> Spec:
    """Spec for a ``Text`` (or a markup string)."""
    from .text import Text

    if isinstance(text, str):
        text = console.render_str(
            text, highlight=options.highlight, markup=options.markup
        )
    elif not isinstance(text, Text):
        raise TypeError(f"expected Text or str, got {text!r}")
    tab_size = console.tab_size if text.tab_size is None else text.tab_size
    plain = text.plain
    spans = text.spans
    if "\t" in plain:
        text = text.copy()
        text.expand_tabs(tab_size or 4)
        plain = text.plain
        spans = text.spans
    out_spans = []
    for span in spans:
        style = _style_of(console, span.style)
        out_spans.append(
            (span.start, span.end, native_style(style), style.link if style else None)
        )
    base = _style_of(console, text.style)
    if base and base.link:
        out_spans.insert(0, (0, len(plain), "", base.link))
    justify = text.justify or options.justify
    overflow = text.overflow or options.overflow
    no_wrap = text.no_wrap if text.no_wrap is not None else options.no_wrap
    return (
        "text",
        plain,
        out_spans,
        native_style(base),
        None if justify in (None, "default") else justify,
        overflow,
        bool(no_wrap),
    )


def to_spec(console: "Console", options: "ConsoleOptions", renderable: Any) -> Spec:
    """Convert any renderable to a spec."""
    from .console import rich_cast
    from .text import Text

    renderable = rich_cast(renderable)
    if isinstance(renderable, (str, Text)):
        return text_spec(console, options, renderable)
    if hasattr(type(renderable), "__fast_rich__") and not isinstance(renderable, type):
        return renderable.__fast_rich__(console, options)
    if hasattr(renderable, "__rich_console__") and not isinstance(renderable, type):
        # Keep the options (justify/overflow/no_wrap/highlight of e.g. a
        # table column); Rust passes them back with the real width.
        return ("callback", renderable, options)
    from . import errors

    raise errors.NotRenderableError(
        f"Unable to render {renderable!r}; "
        "A str, Segment or object with __rich_console__ method is required"
    )


def _push(options: "ConsoleOptions") -> None:
    stack = getattr(_local, "options", None)
    if stack is None:
        stack = _local.options = []
    stack.append(options)


def _pop() -> None:
    _local.options.pop()


def current_options() -> Optional["ConsoleOptions"]:
    """Options of the native render in progress (for callbacks)."""
    stack = getattr(_local, "options", None)
    return stack[-1] if stack else None


def render_lines(
    console: "Console", renderable: Any, options: "ConsoleOptions"
) -> List[List[Segment]]:
    """Render natively into lines of Rich segments (no newlines)."""
    _push(options)
    try:
        spec = to_spec(console, options, renderable)
        lines = _core.render(spec, console, max(0, options.max_width))
    finally:
        _pop()
    return [
        [Segment(text, rich_style(style, link)) for text, style, link in line]
        for line in lines
    ]


def render(
    console: "Console", renderable: Any, options: "ConsoleOptions"
) -> Iterable[Segment]:
    """Render natively, as ``Console.render`` would (with newlines)."""
    from .text import Text

    lines = render_lines(console, renderable, options)
    if isinstance(renderable, str):
        end = "\n"
    elif isinstance(renderable, Text):
        end = renderable.end
    else:
        end = "\n"
    new_line = Segment.line()
    last = len(lines) - 1
    for index, line in enumerate(lines):
        yield from line
        if index < last:
            yield new_line
        elif end:
            yield Segment(end)


def measure(console: "Console", options: "ConsoleOptions", renderable: Any):
    """Rich ``Measurement`` of a natively drawn renderable."""
    from .measure import Measurement

    _push(options)
    try:
        spec = to_spec(console, options, renderable)
        minimum, maximum = _core.measure(spec, console)
    finally:
        _pop()
    return Measurement(minimum, maximum).normalize().with_maximum(options.max_width)


def segments_spec(segments: Iterable[Segment]) -> Spec:
    """Spec for already rendered segments (control codes are dropped)."""
    return (
        "segments",
        [
            (
                seg.text,
                native_style(seg.style),
                seg.style.link if seg.style else None,
            )
            for seg in segments
            if not seg.control
        ],
    )


def box_spec(
    console: "Console",
    box: Any,
    options: "ConsoleOptions",
    safe_box: Optional[bool],
    plain_headed: bool = False,
) -> Optional[str]:
    """Rich box -> its 8-line definition, after the same substitutions Rich
    makes (ASCII-only/legacy terminals, and a plain head for headerless
    tables)."""
    if box is None:
        return None
    box = box.substitute(options, safe=console.safe_box if safe_box is None else safe_box)
    if plain_headed:
        box = box.get_plain_headed_box()
    return str(box)


def pad4(padding: Any) -> Tuple[int, int, int, int]:
    """Rich padding (int or 1/2/4-tuple) -> (top, right, bottom, left)."""
    from .padding import Padding

    return Padding.unpack(padding)
