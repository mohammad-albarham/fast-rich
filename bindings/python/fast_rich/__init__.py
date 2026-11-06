"""fast_rich - a drop-in replacement for Rich, rendered by a Rust core.

Usage: replace rich with fast_rich in imports::

    from fast_rich.console import Console
    from fast_rich.table import Table

The top-level functions below (get_console, print, inspect, ...) are adapted
from Rich (https://github.com/Textualize/rich), Copyright (c) 2020 Will
McGugan, MIT License (see LICENSE.rich).
"""

import os
from typing import IO, TYPE_CHECKING, Any, Callable, Optional, Union

from ._extension import load_ipython_extension  # noqa: F401

__all__ = ["get_console", "reconfigure", "print", "inspect", "print_json"]

if TYPE_CHECKING:
    from .console import Console

# Global console used by alternative print
_console: Optional["Console"] = None

try:
    _IMPORT_CWD = os.path.abspath(os.getcwd())
except FileNotFoundError:
    # Can happen if the cwd has been deleted
    _IMPORT_CWD = ""


def get_console() -> "Console":
    """Get a global :class:`~rich.console.Console` instance. This function is used when Rich requires a Console,
    and hasn't been explicitly given one.

    Returns:
        Console: A console instance.
    """
    global _console
    if _console is None:
        from .console import Console

        _console = Console()

    return _console


def reconfigure(*args: Any, **kwargs: Any) -> None:
    """Reconfigures the global console by replacing it with another.

    Args:
        *args (Any): Positional arguments for the replacement :class:`~rich.console.Console`.
        **kwargs (Any): Keyword arguments for the replacement :class:`~rich.console.Console`.
    """
    from fast_rich.console import Console

    new_console = Console(*args, **kwargs)
    _console = get_console()
    _console.__dict__ = new_console.__dict__


def print(
    *objects: Any,
    sep: str = " ",
    end: str = "\n",
    file: Optional[IO[str]] = None,
    flush: bool = False,
) -> None:
    r"""Print object(s) supplied via positional arguments.
    This function has an identical signature to the built-in print.
    For more advanced features, see the :class:`~rich.console.Console` class.

    Args:
        sep (str, optional): Separator between printed objects. Defaults to " ".
        end (str, optional): Character to write at end of output. Defaults to "\\n".
        file (IO[str], optional): File to write to, or None for stdout. Defaults to None.
        flush (bool, optional): Has no effect as Rich always flushes output. Defaults to False.

    """
    from .console import Console

    write_console = get_console() if file is None else Console(file=file)
    return write_console.print(*objects, sep=sep, end=end)


def print_json(
    json: Optional[str] = None,
    *,
    data: Any = None,
    indent: Union[None, int, str] = 2,
    highlight: bool = True,
    skip_keys: bool = False,
    ensure_ascii: bool = False,
    check_circular: bool = True,
    allow_nan: bool = True,
    default: Optional[Callable[[Any], Any]] = None,
    sort_keys: bool = False,
) -> None:
    """Pretty prints JSON. Output will be valid JSON.

    Args:
        json (str): A string containing JSON.
        data (Any): If json is not supplied, then encode this data.
        indent (int, optional): Number of spaces to indent. Defaults to 2.
        highlight (bool, optional): Enable highlighting of output: Defaults to True.
        skip_keys (bool, optional): Skip keys not of a basic type. Defaults to False.
        ensure_ascii (bool, optional): Escape all non-ascii characters. Defaults to False.
        check_circular (bool, optional): Check for circular references. Defaults to True.
        allow_nan (bool, optional): Allow NaN and Infinity values. Defaults to True.
        default (Callable, optional): A callable that converts values that can not be encoded
            in to something that can be JSON encoded. Defaults to None.
        sort_keys (bool, optional): Sort dictionary keys. Defaults to False.
    """

    get_console().print_json(
        json,
        data=data,
        indent=indent,
        highlight=highlight,
        skip_keys=skip_keys,
        ensure_ascii=ensure_ascii,
        check_circular=check_circular,
        allow_nan=allow_nan,
        default=default,
        sort_keys=sort_keys,
    )


def inspect(
    obj: Any,
    *,
    console: Optional["Console"] = None,
    title: Optional[str] = None,
    help: bool = False,
    methods: bool = False,
    docs: bool = True,
    private: bool = False,
    dunder: bool = False,
    sort: bool = True,
    all: bool = False,
    value: bool = True,
) -> None:
    """Inspect any Python object.

    * inspect(<OBJECT>) to see summarized info.
    * inspect(<OBJECT>, methods=True) to see methods.
    * inspect(<OBJECT>, help=True) to see full (non-abbreviated) help.
    * inspect(<OBJECT>, private=True) to see private attributes (single underscore).
    * inspect(<OBJECT>, dunder=True) to see attributes beginning with double underscore.
    * inspect(<OBJECT>, all=True) to see all attributes.

    Args:
        obj (Any): An object to inspect.
        title (str, optional): Title to display over inspect result, or None use type. Defaults to None.
        help (bool, optional): Show full help text rather than just first paragraph. Defaults to False.
        methods (bool, optional): Enable inspection of callables. Defaults to False.
        docs (bool, optional): Also render doc strings. Defaults to True.
        private (bool, optional): Show private attributes (beginning with underscore). Defaults to False.
        dunder (bool, optional): Show attributes starting with double underscore. Defaults to False.
        sort (bool, optional):  Sort attributes alphabetically, callables at the top, leading and trailing underscores ignored. Defaults to True.
        all (bool, optional): Show all attributes. Defaults to False.
        value (bool, optional): Pretty print value. Defaults to True.
    """
    _console = console or get_console()
    from fast_rich._inspect import Inspect

    # Special case for inspect(inspect)
    is_inspect = obj is inspect

    _inspect = Inspect(
        obj,
        title=title,
        help=is_inspect or help,
        methods=is_inspect or methods,
        docs=is_inspect or docs,
        private=private,
        dunder=dunder,
        sort=sort,
        all=all,
        value=value,
    )
    _console.print(_inspect)


if __name__ == "__main__":  # pragma: no cover
    print("Hello, **World**")


__version__ = "0.4.0"

# Convenience re-exports (fast_rich extension; Rich keeps these in submodules).
from .align import Align, VerticalCenter  # noqa: E402
from .box import ASCII, DOUBLE, HEAVY, HORIZONTALS, MINIMAL, ROUNDED, SIMPLE, SQUARE, Box  # noqa: E402
from .color import Color, ColorTriplet  # noqa: E402
from .columns import Columns  # noqa: E402
from .console import Console, ConsoleDimensions, ConsoleOptions, Group, group  # noqa: E402
from .containers import Lines, Renderables  # noqa: E402
from .control import Control  # noqa: E402
from .emoji import Emoji  # noqa: E402
from .highlighter import (  # noqa: E402
    Highlighter,
    JSONHighlighter,
    NullHighlighter,
    RegexHighlighter,
    ReprHighlighter,
)
from .json import JSON  # noqa: E402
from .layout import Layout  # noqa: E402
from .live import Live  # noqa: E402
from .logging import RichHandler  # noqa: E402
from .markdown import Markdown  # noqa: E402
from .measure import Measurement  # noqa: E402
from .padding import Padding  # noqa: E402
from .panel import Panel  # noqa: E402
from .pretty import Pretty, pprint  # noqa: E402
from .progress import Progress, track  # noqa: E402
from .prompt import Confirm, Prompt  # noqa: E402
from .region import Region  # noqa: E402
from .repr import auto as repr_auto  # noqa: E402
from .rule import Rule  # noqa: E402
from .scope import render_scope  # noqa: E402
from .segment import Segment, Segments  # noqa: E402
from .spinner import Spinner  # noqa: E402
from .status import Status  # noqa: E402
from .style import Style  # noqa: E402
from .styled import Styled  # noqa: E402
from .syntax import Syntax  # noqa: E402
from .table import Table  # noqa: E402
from .terminal_theme import MONOKAI, TerminalTheme  # noqa: E402
from .text import Text  # noqa: E402
from .theme import Theme  # noqa: E402
from .themes import DEFAULT as DEFAULT_THEME  # noqa: E402
from .traceback import Traceback  # noqa: E402
from .traceback import install as install_traceback  # noqa: E402
from .tree import Tree  # noqa: E402

__all__ += [
    "Align", "ASCII", "Box", "Color", "ColorTriplet", "Columns", "Confirm",
    "Console", "ConsoleDimensions", "ConsoleOptions", "Control", "DEFAULT_THEME",
    "DOUBLE", "Emoji", "Group", "HEAVY", "HORIZONTALS", "Highlighter", "JSON",
    "JSONHighlighter", "Layout", "Lines", "Live", "MINIMAL", "MONOKAI",
    "Markdown", "Measurement", "NullHighlighter", "Padding", "Panel", "Pretty",
    "Progress", "Prompt", "ROUNDED", "Region", "RegexHighlighter",
    "Renderables", "ReprHighlighter", "RichHandler", "Rule", "SIMPLE", "SQUARE",
    "Segment", "Segments", "Spinner", "Status", "Style", "Styled", "Syntax",
    "Table", "TerminalTheme", "Text", "Theme", "Traceback", "Tree",
    "VerticalCenter", "group", "install_traceback", "pprint", "render_scope",
    "repr_auto", "track",
]
