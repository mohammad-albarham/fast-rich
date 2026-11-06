"""Rich-style tracebacks (``rich.traceback`` API), drawn with fast_rich's
Rust-rendered Panel/Syntax/Text."""

from __future__ import annotations

import linecache
import os
import sys
from dataclasses import dataclass, field
from types import ModuleType, TracebackType
from typing import TYPE_CHECKING, Any, Callable, Dict, Iterable, List, Optional, Sequence, Type, Union

from ._loop import loop_last
from .constrain import Constrain
from .highlighter import ReprHighlighter
from .panel import Panel
from .pretty import Pretty
from .scope import render_scope
from .syntax import Syntax
from .text import Text

if TYPE_CHECKING:  # pragma: no cover
    from .console import Console, ConsoleOptions, RenderResult


@dataclass
class Frame:
    filename: str
    lineno: int
    name: str
    line: str = ""
    locals: Optional[Dict[str, Any]] = None


@dataclass
class Stack:
    exc_type: str
    exc_value: str
    is_cause: bool = False
    frames: List[Frame] = field(default_factory=list)
    notes: List[str] = field(default_factory=list)


@dataclass
class Trace:
    stacks: List[Stack]


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
        locals_hide_dunder: bool = True,
        locals_hide_sunder: bool = False,
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
        self.theme = theme or "monokai"
        self.word_wrap = word_wrap
        self.show_locals = show_locals
        self.locals_max_length = locals_max_length
        self.locals_max_string = locals_max_string
        self.indent_guides = indent_guides
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
        locals_hide_dunder: bool = True,
        locals_hide_sunder: bool = False,
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
            stack = Stack(
                exc_type=getattr(exc_type, "__name__", str(exc_type)),
                exc_value=str(exc_value),
                is_cause=is_cause,
                notes=[str(n) for n in getattr(exc_value, "__notes__", [])],
            )
            tb = traceback
            while tb is not None:
                frame = tb.tb_frame
                code = frame.f_code
                frame_locals = None
                if show_locals:
                    frame_locals = {
                        key: value
                        for key, value in frame.f_locals.items()
                        if not (locals_hide_dunder and key.startswith("__"))
                        and not (locals_hide_sunder and key.startswith("_"))
                    }
                stack.frames.append(
                    Frame(
                        filename=code.co_filename or "?",
                        lineno=tb.tb_lineno,
                        name=code.co_name,
                        locals=frame_locals,
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

    def _frame_renderables(self, frame: Frame) -> Iterable[Any]:
        yield Text.assemble(
            (frame.filename, "green"),
            ":",
            (str(frame.lineno), "cyan"),
            " in ",
            (frame.name, "bold"),
        )
        if any(frame.filename.startswith(path) for path in self.suppress):
            return
        lines = linecache.getlines(frame.filename)
        if lines:
            first = max(1, frame.lineno - self.extra_lines)
            last = min(len(lines), frame.lineno + self.extra_lines)
            code = "".join(lines[first - 1 : last])
            yield Syntax(
                code,
                Syntax.guess_lexer(frame.filename, code),
                theme=self.theme,
                line_numbers=True,
                start_line=first,
                highlight_lines={frame.lineno},
                code_width=self.code_width,
                word_wrap=self.word_wrap,
            )
        if frame.locals:
            yield render_scope(
                frame.locals,
                title="locals",
                indent_guides=self.indent_guides,
                max_length=self.locals_max_length,
                max_string=self.locals_max_string,
            )

    def _render_stack(self, stack: Stack) -> Any:
        from .console import Group

        frames = stack.frames
        hidden = 0
        if self.max_frames and len(frames) > self.max_frames:
            half = self.max_frames // 2
            hidden = len(frames) - self.max_frames
            frames = frames[:half] + frames[-half:]
        renderables: List[Any] = []
        for index, frame in enumerate(frames):
            if hidden and index == self.max_frames // 2:
                renderables.append(
                    Text(f"\n... {hidden} frames hidden ...", justify="center", style="dim")
                )
            if index:
                renderables.append("")
            renderables.extend(self._frame_renderables(frame))
        return Group(*renderables)

    def __rich_console__(
        self, console: "Console", options: "ConsoleOptions"
    ) -> "RenderResult":
        highlighter = ReprHighlighter()
        for last, stack in loop_last(reversed(self.trace.stacks)):
            if stack.frames:
                panel = Panel(
                    self._render_stack(stack),
                    title="[traceback.title]Traceback [dim](most recent call last)",
                    border_style="traceback.border",
                    expand=True,
                    padding=(0, 1),
                )
                yield Constrain(panel, self.width)
            exc_text = Text.assemble(
                (f"{stack.exc_type}: ", "traceback.exc_type"),
                highlighter(stack.exc_value),
            ) if stack.exc_value else Text(stack.exc_type, style="traceback.exc_type")
            yield exc_text
            for note in stack.notes:
                yield Text.assemble(("[NOTE] ", "traceback.note"), highlighter(note))
            if not last:
                if stack.is_cause:
                    yield Text.from_markup(
                        "\n[i]The above exception was the direct cause of the following exception:\n"
                    )
                else:
                    yield Text.from_markup(
                        "\n[i]During handling of the above exception, another exception occurred:\n"
                    )


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
    locals_hide_dunder: bool = True,
    locals_hide_sunder: Optional[bool] = None,
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
                locals_hide_dunder=locals_hide_dunder,
                locals_hide_sunder=bool(locals_hide_sunder),
                indent_guides=indent_guides,
                suppress=suppress,
                max_frames=max_frames,
            )
        )

    old_excepthook = sys.excepthook
    sys.excepthook = excepthook
    return old_excepthook


__all__ = ["Traceback", "install", "Trace", "Stack", "Frame"]
