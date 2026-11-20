#!/usr/bin/env python3
"""Generate tests/fixtures/box.golden from Python Rich.

For each of Rich's 19 boxes (`rich.box`, in Rich's order) records the box's
8 definition lines and the plain text of a Table and a Panel drawn with
it. `tests/box_rich_parity.rs` checks fast-rich's `BoxStyle` constants and
renders the same Table and Panel with each `BorderStyle`.

Usage (needs `rich` importable, e.g. `pip install rich==15.0.0`, or
`PYTHONPATH` pointed at an unzipped rich wheel):

    python3 scripts/gen_box_golden.py > tests/fixtures/box.golden

Format: a record starts with `@ <BOX> <kind>`, kind `lines` (the 8 box
lines, 4 characters each), `table` (2 columns, header, 3 rows),
`table_lines` (same with `show_lines=True`), `table_noedge`
(`show_edge=False`), `table_noheader` (`show_header=False`, which makes
Rich use `Box.get_plain_headed_box`) or `panel` (two lines of text, width 20, with a
title). The record's lines follow, each prefixed with `>`, trailing spaces
trimmed. Rendering: width 40, no color, no legacy Windows, not ASCII-only
(so `Box.substitute` never changes the box).
"""

import io
from importlib.metadata import version

from rich import box
from rich.console import Console
from rich.panel import Panel
from rich.table import Table

BOXES = [
    "ASCII",
    "ASCII2",
    "ASCII_DOUBLE_HEAD",
    "SQUARE",
    "SQUARE_DOUBLE_HEAD",
    "MINIMAL",
    "MINIMAL_HEAVY_HEAD",
    "MINIMAL_DOUBLE_HEAD",
    "SIMPLE",
    "SIMPLE_HEAD",
    "SIMPLE_HEAVY",
    "HORIZONTALS",
    "ROUNDED",
    "HEAVY",
    "HEAVY_EDGE",
    "HEAVY_HEAD",
    "DOUBLE",
    "DOUBLE_EDGE",
    "MARKDOWN",
]


def render(renderable) -> list:
    out = io.StringIO()
    console = Console(
        file=out, width=40, color_system=None, legacy_windows=False
    )
    console.print(renderable)
    return [line.rstrip() for line in out.getvalue().split("\n")[:-1]]


def table(b, **kwargs):
    t = Table(box=b, **kwargs)
    t.add_column("Name")
    t.add_column("Qty", justify="right")
    t.add_row("apple", "3")
    t.add_row("kiwi", "12")
    t.add_row("fig", "7")
    return t


def emit(name, kind, lines):
    print(f"@ {name} {kind}")
    for line in lines:
        print(f">{line}")


def main() -> None:
    print(f"# rich {version('rich')}: rich.box")
    for name in BOXES:
        b = getattr(box, name)
        definition = str(b).splitlines()
        assert len(definition) == 8 and all(len(l) == 4 for l in definition)
        emit(name, "lines", definition)
        emit(name, "table", render(table(b)))
        emit(name, "table_lines", render(table(b, show_lines=True)))
        emit(name, "table_noedge", render(table(b, show_edge=False)))
        emit(name, "table_noheader", render(table(b, show_header=False)))
        emit(
            name,
            "panel",
            render(Panel("Hello\nWorld", box=b, width=20, title="Title")),
        )


if __name__ == "__main__":
    main()
