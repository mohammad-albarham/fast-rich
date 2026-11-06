# Adapted from Rich (https://github.com/Textualize/rich), Copyright (c) 2020
# Will McGugan, MIT License (see LICENSE.rich). fast_rich change: rendering
# and measuring are done by the Rust core (see _native.py).
from typing import TYPE_CHECKING, Optional

from .align import AlignMethod
from .box import ROUNDED, Box
from .cells import cell_len
from .jupyter import JupyterMixin
from .measure import Measurement, measure_renderables
from .padding import Padding, PaddingDimensions
from .segment import Segment
from .style import Style, StyleType
from .text import Text, TextType

if TYPE_CHECKING:
    from .console import Console, ConsoleOptions, RenderableType, RenderResult


class Panel(JupyterMixin):
    """A console renderable that draws a border around its contents.

    Example:
        >>> console.print(Panel("Hello, World!"))

    Args:
        renderable (RenderableType): A console renderable object.
        box (Box): A Box instance that defines the look of the border (see :ref:`appendix_box`. Defaults to box.ROUNDED.
        title (Optional[TextType], optional): Optional title displayed in panel header. Defaults to None.
        title_align (AlignMethod, optional): Alignment of title. Defaults to "center".
        subtitle (Optional[TextType], optional): Optional subtitle displayed in panel footer. Defaults to None.
        subtitle_align (AlignMethod, optional): Alignment of subtitle. Defaults to "center".
        safe_box (bool, optional): Disable box characters that don't display on windows legacy terminal with *raster* fonts. Defaults to True.
        expand (bool, optional): If True the panel will stretch to fill the console width, otherwise it will be sized to fit the contents. Defaults to True.
        style (str, optional): The style of the panel (border and contents). Defaults to "none".
        border_style (str, optional): The style of the border. Defaults to "none".
        width (Optional[int], optional): Optional width of panel. Defaults to None to auto-detect.
        height (Optional[int], optional): Optional height of panel. Defaults to None to auto-detect.
        padding (Optional[PaddingDimensions]): Optional padding around renderable. Defaults to 0.
        highlight (bool, optional): Enable automatic highlighting of panel title (if str). Defaults to False.
    """

    def __init__(
        self,
        renderable: "RenderableType",
        box: Box = ROUNDED,
        *,
        title: Optional[TextType] = None,
        title_align: AlignMethod = "center",
        subtitle: Optional[TextType] = None,
        subtitle_align: AlignMethod = "center",
        safe_box: Optional[bool] = None,
        expand: bool = True,
        style: StyleType = "none",
        border_style: StyleType = "none",
        width: Optional[int] = None,
        height: Optional[int] = None,
        padding: PaddingDimensions = (0, 1),
        highlight: bool = False,
    ) -> None:
        self.renderable = renderable
        self.box = box
        self.title = title
        self.title_align: AlignMethod = title_align
        self.subtitle = subtitle
        self.subtitle_align = subtitle_align
        self.safe_box = safe_box
        self.expand = expand
        self.style = style
        self.border_style = border_style
        self.width = width
        self.height = height
        self.padding = padding
        self.highlight = highlight

    @classmethod
    def fit(
        cls,
        renderable: "RenderableType",
        box: Box = ROUNDED,
        *,
        title: Optional[TextType] = None,
        title_align: AlignMethod = "center",
        subtitle: Optional[TextType] = None,
        subtitle_align: AlignMethod = "center",
        safe_box: Optional[bool] = None,
        style: StyleType = "none",
        border_style: StyleType = "none",
        width: Optional[int] = None,
        height: Optional[int] = None,
        padding: PaddingDimensions = (0, 1),
        highlight: bool = False,
    ) -> "Panel":
        """An alternative constructor that sets expand=False."""
        return cls(
            renderable,
            box,
            title=title,
            title_align=title_align,
            subtitle=subtitle,
            subtitle_align=subtitle_align,
            safe_box=safe_box,
            style=style,
            border_style=border_style,
            width=width,
            height=height,
            padding=padding,
            highlight=highlight,
            expand=False,
        )

    @property
    def _title(self) -> Optional[Text]:
        if self.title:
            title_text = (
                Text.from_markup(self.title)
                if isinstance(self.title, str)
                else self.title.copy()
            )
            title_text.end = ""
            title_text.plain = title_text.plain.replace("\n", " ")
            title_text.no_wrap = True
            title_text.expand_tabs()
            title_text.pad(1)
            return title_text
        return None

    @property
    def _subtitle(self) -> Optional[Text]:
        if self.subtitle:
            subtitle_text = (
                Text.from_markup(self.subtitle)
                if isinstance(self.subtitle, str)
                else self.subtitle.copy()
            )
            subtitle_text.end = ""
            subtitle_text.plain = subtitle_text.plain.replace("\n", " ")
            subtitle_text.no_wrap = True
            subtitle_text.expand_tabs()
            subtitle_text.pad(1)
            return subtitle_text
        return None


from . import _native  # noqa: E402

def _native_console(self, console, options):  # type: ignore[no-untyped-def]
    yield from console.render(self, options)


def _native_measure(self, console, options):  # type: ignore[no-untyped-def]
    return _native.measure(console, options, self)


def _panel_spec(self: Panel, console: "Console", options: "ConsoleOptions") -> tuple:
    child_options = options.update(highlight=self.highlight)

    def label(title: Optional[TextType]) -> Optional[tuple]:
        if title is None or title == "":
            return None
        text = Text.from_markup(title) if isinstance(title, str) else title.copy()
        return _native.text_spec(console, child_options, text)

    style = console.get_style(self.style)
    return (
        "panel",
        _native.to_spec(console, child_options, self.renderable),
        dict(
            box=_native.box_spec(console, self.box, options, self.safe_box),
            expand=self.expand,
            width=self.width,
            height=self.height or options.height,
            padding=_native.pad4(self.padding),
            style=_native.native_style(style),
            border_style=_native.native_style(console.get_style(self.border_style)),
            title=label(self.title),
            title_align=self.title_align,
            subtitle=label(self.subtitle),
            subtitle_align=self.subtitle_align,
        ),
    )


Panel.__fast_rich__ = _panel_spec  # type: ignore[attr-defined]
Panel.__rich_console__ = _native_console  # type: ignore[attr-defined]
Panel.__rich_measure__ = _native_measure  # type: ignore[attr-defined]
