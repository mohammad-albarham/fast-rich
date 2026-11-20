#!/usr/bin/env python3
"""Generate tests/fixtures/style_ansi.golden from Python Rich.

For each style, records the SGR parameters Rich writes for it
(`Style._make_ansi_codes(ColorSystem.TRUECOLOR)`) and the full output of
`Console.print` for the text `x` in that style (truecolor, forced
terminal, width 20). `tests/style_ansi_parity.rs` renders the same style
with fast-rich and compares.

Usage (needs `rich` importable, e.g. `pip install rich==15.0.0`, or
`PYTHONPATH` pointed at an unzipped rich wheel):

    python3 scripts/gen_style_ansi_golden.py > tests/fixtures/style_ansi.golden

Format (one record per line, space-separated; styles use `+` for spaces,
output is UTF-8 hex):

    <style> <sgr codes or -> <console output hex>
"""

import io
from importlib.metadata import version

from rich.color import ColorSystem
from rich.console import Console
from rich.style import Style
from rich.text import Text

STYLES = [
    "bold",
    "dim",
    "italic",
    "underline",
    "blink",
    "blink2",
    "reverse",
    "conceal",
    "strike",
    "underline2",
    "frame",
    "encircle",
    "overline",
    "uu",
    "o",
    "bold overline",
    "underline2 frame encircle",
    "blink blink2",
    "#ff8000 on #0000ff overline",
    "underline underline2",
    "bold dim italic underline blink blink2 reverse conceal strike"
    " underline2 frame encircle overline",
]


def main() -> None:
    print(f"# rich {version('rich')}: <style> <sgr> <console output hex>")
    for name in STYLES:
        style = Style.parse(name)
        codes = style._make_ansi_codes(ColorSystem.TRUECOLOR) or "-"
        out = io.StringIO()
        console = Console(
            file=out, force_terminal=True, color_system="truecolor", width=20
        )
        console.print(Text("x", style=style))
        print(name.replace(" ", "+"), codes, out.getvalue().encode().hex())


if __name__ == "__main__":
    main()
