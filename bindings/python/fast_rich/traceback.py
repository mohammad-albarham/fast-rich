"""Rich-style tracebacks (``rich.traceback`` API).

Adapted from Rich (https://github.com/Textualize/rich), Copyright (c) 2020
Will McGugan, MIT License (see LICENSE.rich). fast_rich change: code is
highlighted by the Rust core (syntect) instead of Pygments, and the theme's
token colours come from ``fast_rich._core.syntax_theme_styles``.
"""

from __future__ import annotations

import inspect
import linecache
import os
import sys
from dataclasses import dataclass, field
from itertools import islice
from types import ModuleType, TracebackType
from typing import (
    TYPE_CHECKING,
    Any,
    Callable,
    Dict,
    Iterable,
    List,
    Optional,
    Tuple,
    Type,
    Union,
)

from . import _core
from ._loop import loop_last
from .columns import Columns
from .constrain import Constrain
from .highlighter import RegexHighlighter, ReprHighlighter
from .panel import Panel
from .scope import render_scope
from .style import Style
from .syntax import Syntax
from .text import Text

if TYPE_CHECKING:  # pragma: no cover
    from .console import Console, ConsoleOptions, OverflowMethod, RenderResult


class PathHighlighter(RegexHighlighter):
    highlights = [r"(?P<dim>.*/)(?P<bold>.+)"]


@dataclass
class Frame:
    filename: str
    lineno: int
    name: str
    line: str = ""
    locals: Optional[Dict[str, Any]] = None
    last_instruction: Optional[Tuple[Tuple[int, int], Tuple[int, int]]] = None


@dataclass
class _SyntaxError:
    offset: int
    filename: str
    line: str
    lineno: int
    msg: str
    notes: List[str] = field(default_factory=list)


@dataclass
class Stack:
    exc_type: str
    exc_value: str
    syntax_error: Optional[_SyntaxError] = None
    is_cause: bool = False
    frames: List[Frame] = field(default_factory=list)
    notes: List[str] = field(default_factory=list)


@dataclass
class Trace:
    stacks: List[Stack]


def _safe_str(value: object) -> str:
    try:
        return str(value)
    except Exception:  # noqa: BLE001
        return "<exception str() failed>"


def _iter_syntax_lines(
    start: Tuple[int, int], end: Tuple[int, int]
) -> Iterable[Tuple[int, int, int]]:
    """(line, column1, column2) for each line of a range (Rich's helper)."""
    line1, column1 = start
    line2, column2 = end
    if line1 == line2:
        yield line1, column1, column2
    else:
        for first, last, line_no in _loop_first_last(range(line1, line2 + 1)):
            if first:
                yield line_no, column1, -1
            elif last:
                yield line_no, 0, column2
            else:
                yield line_no, 0, -1


def _loop_first_last(values: Iterable[int]) -> Iterable[Tuple[bool, bool, int]]:
    values = list(values)
    for index, value in enumerate(values):
        yield index == 0, index == len(values) - 1, value


def _last_instruction(
    tb: TracebackType,
) -> Optional[Tuple[Tuple[int, int], Tuple[int, int]]]:
    """Source range of the frame's last instruction (Python 3.11+).

    Like Rich, this uses the frame's current ``f_lasti`` (not ``tb_lasti``),
    so outer frames point at where they are now.
    """
    if sys.version_info < (3, 11):
        return None
    try:
        index = tb.tb_frame.f_lasti // 2
        positions = next(islice(tb.tb_frame.f_code.co_positions(), index, index + 1))
    except (StopIteration, ValueError):
        return None
    start_line, end_line, start_column, end_column = positions
    if None in (start_line, end_line, start_column, end_column):
        return None
    return ((start_line, start_column), (end_line, end_column))


def _suppressed_paths(suppress: Iterable[Union[str, ModuleType]]) -> List[str]:
    paths = []
    for item in suppress:
        if isinstance(item, ModuleType):
            if item.__file__ is None:
                continue
            item = os.path.dirname(item.__file__)
        paths.append(os.path.normpath(os.path.abspath(item)))
    return paths


class Traceback:
    """A Console renderable that renders a traceback.

    Same constructor as ``rich.traceback.Traceback``.
    """

    LEXERS = {"": "text", ".py": "python", ".pxd": "cython", ".pyx": "cython", ".pxi": "pyrex"}

    def __init__(
        self,
        trace: Optional[Trace] = None,
        *,
        width: Optional[int] = 100,
        code_width: Optional[int] = 88,
        extra_lines: int = 3,
        theme: Optional[str] = None,
        word_wrap: bool = False,
        show_locals: bool = False,
        locals_max_length: int = 10,
        locals_max_string: int = 80,
        locals_max_depth: Optional[int] = None,
        locals_hide_dunder: bool = True,
        locals_hide_sunder: bool = False,
        locals_overflow: Optional["OverflowMethod"] = None,
        indent_guides: bool = True,
        suppress: Iterable[Union[str, ModuleType]] = (),
        max_frames: int = 100,
    ) -> None:
        if trace is None:
            exc_type, exc_value, traceback = sys.exc_info()
            if exc_type is None or exc_value is None or traceback is None:
                raise ValueError(
                    "Value for 'trace' required if not called in except: block"
                )
            trace = self.extract(
                exc_type,
                exc_value,
                traceback,
                show_locals=show_locals,
                locals_hide_dunder=locals_hide_dunder,
                locals_hide_sunder=locals_hide_sunder,
            )
        self.trace = trace
        self.width = width
        self.code_width = code_width
        self.extra_lines = extra_lines
        # Rich's default traceback theme: the terminal's ANSI colours.
        self.theme = theme if isinstance(theme, str) else "ansi_dark"
        self.word_wrap = word_wrap
        self.show_locals = show_locals
        self.indent_guides = indent_guides
        self.locals_max_length = locals_max_length
        self.locals_max_string = locals_max_string
        self.locals_max_depth = locals_max_depth
        self.locals_overflow = locals_overflow
        self.suppress = _suppressed_paths(suppress)
        self.max_frames = max(4, max_frames) if max_frames > 0 else 0

    @classmethod
    def from_exception(
        cls,
        exc_type: Type[Any],
        exc_value: BaseException,
        traceback: Optional[TracebackType],
        *,
        width: Optional[int] = 100,
        code_width: Optional[int] = 88,
        extra_lines: int = 3,
        theme: Optional[str] = None,
        word_wrap: bool = False,
        show_locals: bool = False,
        locals_max_length: int = 10,
        locals_max_string: int = 80,
        locals_max_depth: Optional[int] = None,
        locals_hide_dunder: bool = True,
        locals_hide_sunder: bool = False,
        locals_overflow: Optional["OverflowMethod"] = None,
        indent_guides: bool = True,
        suppress: Iterable[Union[str, ModuleType]] = (),
        max_frames: int = 100,
    ) -> "Traceback":
        """Create a traceback from exception info."""
        trace = cls.extract(
            exc_type,
            exc_value,
            traceback,
            show_locals=show_locals,
            locals_hide_dunder=locals_hide_dunder,
            locals_hide_sunder=locals_hide_sunder,
        )
        return cls(
            trace,
            width=width,
            code_width=code_width,
            extra_lines=extra_lines,
            theme=theme,
            word_wrap=word_wrap,
            show_locals=show_locals,
            locals_max_length=locals_max_length,
            locals_max_string=locals_max_string,
            locals_max_depth=locals_max_depth,
            locals_overflow=locals_overflow,
            indent_guides=indent_guides,
            suppress=suppress,
            max_frames=max_frames,
        )

    @classmethod
    def extract(
        cls,
        exc_type: Type[BaseException],
        exc_value: BaseException,
        traceback: Optional[TracebackType],
        *,
        show_locals: bool = False,
        locals_max_length: int = 10,
        locals_max_string: int = 80,
        locals_hide_dunder: bool = True,
        locals_hide_sunder: bool = False,
    ) -> Trace:
        """Extract the stacks (innermost exception first) from exception info."""
        stacks: List[Stack] = []
        is_cause = False
        seen = set()
        while exc_value is not None and id(exc_value) not in seen:
            seen.add(id(exc_value))
            notes = [_safe_str(n) for n in getattr(exc_value, "__notes__", None) or []]
            stack = Stack(
                exc_type=_safe_str(exc_type.__name__),
                exc_value=_safe_str(exc_value),
                is_cause=is_cause,
                notes=notes,
            )
            if isinstance(exc_value, SyntaxError) and exc_value.filename != "<stdin>":
                stack.syntax_error = _SyntaxError(
                    offset=exc_value.offset or 0,
                    filename=exc_value.filename or "?",
                    lineno=exc_value.lineno or 0,
                    line=exc_value.text or "",
                    msg=exc_value.msg,
                    notes=notes,
                )
            tb = traceback
            while tb is not None:
                frame = tb.tb_frame
                code = frame.f_code
                frame_locals = None
                if show_locals:
                    # Like Rich: no functions or classes.
                    frame_locals = {
                        key: value
                        for key, value in frame.f_locals.items()
                        if not (locals_hide_dunder and key.startswith("__"))
                        and not (locals_hide_sunder and key.startswith("_"))
                        and not (inspect.isfunction(value) or inspect.isclass(value))
                    }
                filename = code.co_filename or "?"
                if filename and not filename.startswith("<"):
                    if not os.path.isabs(filename):
                        filename = os.path.join(os.getcwd(), filename)
                stack.frames.append(
                    Frame(
                        filename=filename,
                        lineno=tb.tb_lineno,
                        name=code.co_name,
                        locals=frame_locals,
                        last_instruction=_last_instruction(tb),
                    )
                )
                tb = tb.tb_next
            stacks.append(stack)
            cause = exc_value.__cause__
            if cause is not None:
                exc_type, exc_value, traceback = type(cause), cause, cause.__traceback__
                is_cause = True
                continue
            context = exc_value.__context__
            if context is not None and not getattr(exc_value, "__suppress_context__", False):
                exc_type, exc_value, traceback = type(context), context, context.__traceback__
                is_cause = False
                continue
            break
        return Trace(stacks=stacks)

    def _background_style(self) -> Style:
        """The syntax theme's background (none for the ANSI themes)."""
        background = _core.syntax_theme_styles(self.theme).get("background", "")
        return Style.parse(background or "none")

    def __rich_console__(
        self, console: "Console", options: "ConsoleOptions"
    ) -> "RenderResult":
        background_style = self._background_style()
        highlighter = ReprHighlighter()
        for last, stack in loop_last(reversed(self.trace.stacks)):
            if stack.frames:
                stack_renderable = Constrain(
                    Panel(
                        self._render_stack(stack),
                        title="[traceback.title]Traceback [dim](most recent call last)",
                        style=background_style,
                        border_style="traceback.border",
                        expand=True,
                        padding=(0, 1),
                    ),
                    self.width,
                )
                # Rich 15 maps pygments.* styles to the theme's token colours
                # in a console theme, but they never reach its output (frame
                # headers stay uncoloured), so no theme is pushed here.
                yield stack_renderable
            if stack.syntax_error is not None:
                yield Constrain(
                    Panel(
                        self._render_syntax_error(stack.syntax_error),
                        style=background_style,
                        border_style="traceback.border.syntax_error",
                        expand=True,
                        padding=(0, 1),
                        width=self.width,
                    ),
                    self.width,
                )
                yield Text.assemble(
                    (f"{stack.exc_type}: ", "traceback.exc_type"),
                    highlighter(stack.syntax_error.msg),
                )
            elif stack.exc_value:
                yield Text.assemble(
                    (f"{stack.exc_type}: ", "traceback.exc_type"),
                    highlighter(stack.exc_value),
                )
            else:
                yield Text.assemble((f"{stack.exc_type}", "traceback.exc_type"))
            for note in stack.notes:
                yield Text.assemble(("[NOTE] ", "traceback.note"), highlighter(note))
            if not last:
                if stack.is_cause:
                    yield Text.from_markup(
                        "\n[i]The above exception was the direct cause of the following exception:\n",
                    )
                else:
                    yield Text.from_markup(
                        "\n[i]During handling of the above exception, another exception occurred:\n",
                    )

    def _render_syntax_error(self, syntax_error: _SyntaxError) -> Any:
        from .console import Group

        highlighter = ReprHighlighter()
        path_highlighter = PathHighlighter()
        parts: List[Any] = []
        if syntax_error.filename != "<stdin>" and os.path.exists(syntax_error.filename):
            text = Text.assemble(
                (f" {syntax_error.filename}", "pygments.string"),
                (":", "pygments.text"),
                (str(syntax_error.lineno), "pygments.number"),
                style="pygments.text",
            )
            parts.append(path_highlighter(text))
        syntax_error_text = highlighter(syntax_error.line.rstrip())
        syntax_error_text.no_wrap = True
        offset = min(syntax_error.offset - 1, len(syntax_error_text))
        syntax_error_text.stylize("bold underline", offset, offset)
        syntax_error_text += Text.from_markup(
            "\n" + " " * offset + "[traceback.offset]▲[/]",
            style="pygments.text",
        )
        parts.append(syntax_error_text)
        return Group(*parts)

    @classmethod
    def _guess_lexer(cls, filename: str, code: str) -> str:
        ext = os.path.splitext(filename)[-1]
        if not ext:
            new_line_index = code.index("\n") if "\n" in code else len(code)
            first_line = code[:new_line_index]
            if first_line.startswith("#!") and "python" in first_line.lower():
                return "python"
        return cls.LEXERS.get(ext) or Syntax.guess_lexer(filename, code)

    def _render_stack(self, stack: Stack) -> Any:
        from .console import Group

        path_highlighter = PathHighlighter()

        def render_locals(frame: Frame) -> List[Any]:
            if not frame.locals:
                return []
            return [
                render_scope(
                    frame.locals,
                    title="locals",
                    indent_guides=self.indent_guides,
                    max_length=self.locals_max_length,
                    max_string=self.locals_max_string,
                    max_depth=self.locals_max_depth,
                    overflow=self.locals_overflow,
                )
            ]

        renderables: List[Any] = []
        exclude_frames: Optional[range] = None
        if self.max_frames != 0:
            exclude_frames = range(
                self.max_frames // 2,
                len(stack.frames) - self.max_frames // 2,
            )
        excluded = False
        for frame_index, frame in enumerate(stack.frames):
            if exclude_frames and frame_index in exclude_frames:
                excluded = True
                continue
            if excluded:
                assert exclude_frames is not None
                renderables.append(
                    Text(
                        f"\n... {len(exclude_frames)} frames hidden ...",
                        justify="center",
                        style="traceback.error",
                    )
                )
                excluded = False
            first = frame_index == 0
            suppressed = any(frame.filename.startswith(path) for path in self.suppress)
            if os.path.exists(frame.filename):
                text = Text.assemble(
                    path_highlighter(Text(frame.filename, style="pygments.string")),
                    (":", "pygments.text"),
                    (str(frame.lineno), "pygments.number"),
                    " in ",
                    (frame.name, "pygments.function"),
                    style="pygments.text",
                )
            else:
                text = Text.assemble(
                    "in ",
                    (frame.name, "pygments.function"),
                    (":", "pygments.text"),
                    (str(frame.lineno), "pygments.number"),
                    style="pygments.text",
                )
            if not frame.filename.startswith("<") and not first:
                renderables.append("")
            renderables.append(text)
            if frame.filename.startswith("<"):
                renderables.extend(render_locals(frame))
                continue
            if suppressed:
                continue
            code_lines = linecache.getlines(frame.filename)
            code = "".join(code_lines)
            if not code:
                continue
            syntax = Syntax(
                code,
                self._guess_lexer(frame.filename, code),
                theme=self.theme,
                line_numbers=True,
                line_range=(
                    frame.lineno - self.extra_lines,
                    frame.lineno + self.extra_lines,
                ),
                highlight_lines={frame.lineno},
                word_wrap=self.word_wrap,
                code_width=self.code_width,
                indent_guides=self.indent_guides,
                dedent=False,
            )
            renderables.append("")
            if frame.last_instruction is not None:
                start, end = frame.last_instruction
                # A line at a time, so indentation isn't underlined.
                for line1, column1, column2 in _iter_syntax_lines(start, end):
                    try:
                        if column1 == 0:
                            line = code_lines[line1 - 1]
                            column1 = len(line) - len(line.lstrip())
                        if column2 == -1:
                            column2 = len(code_lines[line1 - 1])
                    except IndexError:
                        continue
                    syntax.stylize_range(
                        style="traceback.error_range",
                        start=(line1, column1),
                        end=(line1, column2),
                    )
            locals_ = render_locals(frame)
            renderables.append(Columns([syntax, *locals_], padding=1) if locals_ else syntax)
        return Group(*renderables)


def install(
    *,
    console: Optional["Console"] = None,
    width: Optional[int] = 100,
    code_width: Optional[int] = 88,
    extra_lines: int = 3,
    theme: Optional[str] = None,
    word_wrap: bool = False,
    show_locals: bool = False,
    locals_max_length: int = 10,
    locals_max_string: int = 80,
    locals_max_depth: Optional[int] = None,
    locals_hide_dunder: bool = True,
    locals_hide_sunder: Optional[bool] = None,
    locals_overflow: Optional["OverflowMethod"] = None,
    indent_guides: bool = True,
    suppress: Iterable[Union[str, ModuleType]] = (),
    max_frames: int = 100,
) -> Callable[[Type[BaseException], BaseException, Optional[TracebackType]], Any]:
    """Install a rich traceback handler as ``sys.excepthook``.

    Returns:
        The previous exception handler.
    """
    from .console import Console

    traceback_console = Console(stderr=True) if console is None else console
    locals_hide_sunder = (
        True if (traceback_console.is_jupyter and locals_hide_sunder is None) else bool(locals_hide_sunder)
    )

    def excepthook(
        type_: Type[BaseException],
        value: BaseException,
        traceback: Optional[TracebackType],
    ) -> None:
        traceback_console.print(
            Traceback.from_exception(
                type_,
                value,
                traceback,
                width=width,
                code_width=code_width,
                extra_lines=extra_lines,
                theme=theme,
                word_wrap=word_wrap,
                show_locals=show_locals,
                locals_max_length=locals_max_length,
                locals_max_string=locals_max_string,
                locals_max_depth=locals_max_depth,
                locals_hide_dunder=locals_hide_dunder,
                locals_hide_sunder=locals_hide_sunder,
                locals_overflow=locals_overflow,
                indent_guides=indent_guides,
                suppress=suppress,
                max_frames=max_frames,
            )
        )

    old_excepthook = sys.excepthook
    sys.excepthook = excepthook
    return old_excepthook


__all__ = ["Traceback", "install", "Trace", "Stack", "Frame", "PathHighlighter"]
