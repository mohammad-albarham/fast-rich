# Adapted from Rich (https://github.com/Textualize/rich), Copyright (c) 2020
# Will McGugan, MIT License (see LICENSE.rich). fast_rich change: rendering
# and measuring are done by the Rust core (see _native.py).
from typing import Iterator, List, Optional, Tuple

from ._loop import loop_first, loop_last
from .console import Console, ConsoleOptions, RenderableType, RenderResult
from .jupyter import JupyterMixin
from .measure import Measurement
from .segment import Segment
from .style import Style, StyleStack, StyleType
from .styled import Styled

GuideType = Tuple[str, str, str, str]


class Tree(JupyterMixin):
    """A renderable for a tree structure.

    Attributes:
        ASCII_GUIDES (GuideType): Guide lines used when Console.ascii_only is True.
        TREE_GUIDES (List[GuideType, GuideType, GuideType]): Default guide lines.

    Args:
        label (RenderableType): The renderable or str for the tree label.
        style (StyleType, optional): Style of this tree. Defaults to "tree".
        guide_style (StyleType, optional): Style of the guide lines. Defaults to "tree.line".
        expanded (bool, optional): Also display children. Defaults to True.
        highlight (bool, optional): Highlight renderable (if str). Defaults to False.
        hide_root (bool, optional): Hide the root node. Defaults to False.
    """

    ASCII_GUIDES = ("    ", "|   ", "+-- ", "`-- ")
    TREE_GUIDES = [
        ("    ", "│   ", "├── ", "└── "),
        ("    ", "┃   ", "┣━━ ", "┗━━ "),
        ("    ", "║   ", "╠══ ", "╚══ "),
    ]

    def __init__(
        self,
        label: RenderableType,
        *,
        style: StyleType = "tree",
        guide_style: StyleType = "tree.line",
        expanded: bool = True,
        highlight: bool = False,
        hide_root: bool = False,
    ) -> None:
        self.label = label
        self.style = style
        self.guide_style = guide_style
        self.children: List[Tree] = []
        self.expanded = expanded
        self.highlight = highlight
        self.hide_root = hide_root

    def add(
        self,
        label: RenderableType,
        *,
        style: Optional[StyleType] = None,
        guide_style: Optional[StyleType] = None,
        expanded: bool = True,
        highlight: Optional[bool] = False,
    ) -> "Tree":
        """Add a child tree.

        Args:
            label (RenderableType): The renderable or str for the tree label.
            style (StyleType, optional): Style of this tree. Defaults to "tree".
            guide_style (StyleType, optional): Style of the guide lines. Defaults to "tree.line".
            expanded (bool, optional): Also display children. Defaults to True.
            highlight (Optional[bool], optional): Highlight renderable (if str). Defaults to False.

        Returns:
            Tree: A new child Tree, which may be further modified.
        """
        node = Tree(
            label,
            style=self.style if style is None else style,
            guide_style=self.guide_style if guide_style is None else guide_style,
            expanded=expanded,
            highlight=self.highlight if highlight is None else highlight,
        )
        self.children.append(node)
        return node


from . import _native  # noqa: E402

def _native_console(self, console, options):  # type: ignore[no-untyped-def]
    yield from console.render(self, options)


def _native_measure(self, console, options):  # type: ignore[no-untyped-def]
    return _native.measure(console, options, self)


def _tree_spec(self: Tree, console: "Console", options: "ConsoleOptions") -> tuple:
    get_style = console.get_style
    label_options = options.update(highlight=self.highlight, justify=None)

    def node(tree: Tree) -> tuple:
        return (
            _native.to_spec(console, label_options, tree.label),
            _native.native_style(get_style(tree.style, default="")),
            tree.expanded,
            [node(child) for child in tree.children],
        )

    guide_style = get_style(self.guide_style, default="") or Style.null()
    if options.ascii_only:
        guide = "ascii"
    elif guide_style.bold:
        guide = "bold"
    elif guide_style.underline2:
        guide = "double"
    else:
        guide = "unicode"
    return (
        "tree",
        node(self),
        dict(
            guide=guide,
            guide_style=_native.native_style(guide_style),
            hide_root=self.hide_root,
        ),
    )


Tree.__fast_rich__ = _tree_spec  # type: ignore[attr-defined]
Tree.__rich_console__ = _native_console  # type: ignore[attr-defined]
Tree.__rich_measure__ = _native_measure  # type: ignore[attr-defined]
