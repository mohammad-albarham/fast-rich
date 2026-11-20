#!/usr/bin/env python3
"""Generate tests/fixtures/ansi_decode.golden from Python Rich.

Feeds strings with ANSI escape sequences through Rich's `Text.from_ansi`
and records the plain text and the style of every character.
`tests/ansi_parity.rs` decodes the same strings with fast-rich's
`Text::from_ansi` and compares.

The inputs are samples shaped like real tool output (`ls --color`,
`cargo`, `git diff --color`, `grep --color`, progress lines with `\\r`),
hand-written edge cases, and seeded random mixes of SGR codes, OSC 8 links,
other escapes and text.

Usage (needs `rich` importable, e.g. `pip install rich==15.0.0`, or
`PYTHONPATH` pointed at an unzipped rich wheel):

    python3 scripts/gen_ansi_golden.py > tests/fixtures/ansi_decode.golden

Format (one record per line):

    <input hex> <plain hex> <runs>

`runs` is `-` for empty text, otherwise comma-separated `<chars>:<style
hex>` runs of equal style. A style is written as space-separated words:
the attributes that are on (Rich's names and order), `fg=<c>`, `bg=<c>`
and `link=<url>`, where a color is `default`, a palette number or
`#rrggbb`. Attributes that Rich holds as "off" (`not bold`) are left out:
fast-rich styles have no "off" state, and an "off" attribute renders the
same as an unset one in a decoded text.

OSC strings ended by BEL are not generated: Rich does not recognize that
terminator (fast-rich does, see `src/ansi.rs`).
"""

import random
from importlib.metadata import version

from rich.color import ColorType
from rich.style import Style
from rich.text import Text

ATTRS = [
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
]

E = "\x1b"
ST = "\x1b\\"


def color_desc(color) -> str:
    if color.type == ColorType.DEFAULT:
        return "default"
    if color.type == ColorType.TRUECOLOR:
        return "#%02x%02x%02x" % tuple(color.triplet)
    return str(color.number)


def style_desc(style: Style) -> str:
    words = [name for name in ATTRS if getattr(style, name) is True]
    if style.color is not None:
        words.append("fg=" + color_desc(style.color))
    if style.bgcolor is not None:
        words.append("bg=" + color_desc(style.bgcolor))
    if style.link:
        words.append("link=" + style.link)
    return " ".join(words)


def record(source: str) -> str:
    text = Text.from_ansi(source)
    styles = [Style.null()] * len(text.plain)
    for span in text.spans:
        style = span.style if isinstance(span.style, Style) else Style.parse(span.style)
        for i in range(span.start, min(span.end, len(styles))):
            styles[i] = styles[i] + style
    runs = []
    for desc in map(style_desc, styles):
        if runs and runs[-1][1] == desc:
            runs[-1][0] += 1
        else:
            runs.append([1, desc])
    encoded = ",".join(f"{n}:{d.encode().hex()}" for n, d in runs) or "-"
    return f"{source.encode().hex()} {text.plain.encode().hex()} {encoded}"


def sgr(*codes) -> str:
    return f"{E}[{';'.join(str(c) for c in codes)}m"


def link(url: str, params: str = "") -> str:
    return f"{E}]8;{params};{url}{ST}"


REAL = [
    # ls --color (GNU dircolors)
    f"{E}[0m{E}[01;34mbenches{E}[0m  {E}[01;32mbuild.sh{E}[0m  Cargo.toml"
    f"  {E}[01;36mlink{E}[0m  {E}[40;33;01mtty0{E}[0m\n"
    f"{E}[01;31marchive.tar.gz{E}[0m  {E}[30;42mshared{E}[0m  {E}[37;41msetuid{E}[0m\n",
    f"total 8\ndrwxr-xr-x 2 u u 4096 Jan  1 00:00 {E}[01;34msrc{E}[0m\n"
    f"-rw-r--r-- 1 u u  120 Jan  1 00:00 {E}[00mREADME.md{E}[0m\n",
    f"{E}[38;5;33mdir{E}[39m {E}[38;5;208mimage.png{E}[39m {E}[1;38;5;196mcore{E}[0m",
    # cargo build errors / warnings
    f"{E}[0m{E}[1m{E}[38;5;9merror[E0425]{E}[0m{E}[0m{E}[1m: cannot find value `x` in this scope{E}[0m\n"
    f"{E}[0m {E}[0m{E}[0m{E}[1m{E}[38;5;12m--> {E}[0m{E}[0msrc/main.rs:2:5{E}[0m\n"
    f"{E}[0m  {E}[0m{E}[0m{E}[1m{E}[38;5;12m|{E}[0m\n"
    f"{E}[0m{E}[1m{E}[38;5;12m2{E}[0m{E}[0m {E}[0m{E}[0m{E}[1m{E}[38;5;12m|{E}[0m{E}[0m     x{E}[0m\n"
    f"{E}[0m  {E}[0m{E}[0m{E}[1m{E}[38;5;12m|{E}[0m{E}[0m     {E}[0m{E}[0m{E}[1m{E}[38;5;9m^{E}[0m{E}[0m "
    f"{E}[0m{E}[0m{E}[1m{E}[38;5;9mnot found in this scope{E}[0m\n",
    f"{E}[0m{E}[1m{E}[33mwarning{E}[0m{E}[0m{E}[1m: unused variable: `y`{E}[0m\n"
    f"{E}[1m{E}[92m   Compiling{E}[0m demo v0.1.0 (/work/demo)\n"
    f"{E}[1m{E}[92m    Finished{E}[0m `dev` profile [unoptimized + debuginfo] target(s) in 0.42s\n",
    f"{E}[1m{E}[96m    Building{E}[0m [=======>                 ] 12/40: syn\r"
    f"{E}[1m{E}[96m    Building{E}[0m [==============>          ] 24/40: serde\r{E}[K",
    # git diff --color
    f"{E}[1mdiff --git a/src/lib.rs b/src/lib.rs{E}[m\n"
    f"{E}[1mindex 3b18e51..a1b2c3d 100644{E}[m\n"
    f"{E}[1m--- a/src/lib.rs{E}[m\n{E}[1m+++ b/src/lib.rs{E}[m\n"
    f"{E}[36m@@ -1,3 +1,3 @@{E}[m fn main() {{\n"
    f"{E}[31m-    let x = 1;{E}[m\n{E}[32m+{E}[m{E}[32m    let x = 2;{E}[m\n"
    f"{E}[32m+{E}[m{E}[41m   {E}[m\n",
    f"{E}[33mcommit 2be0d17{E}[m{E}[33m ({E}[m{E}[1;36mHEAD -> {E}[m{E}[1;32mmain{E}[m{E}[33m){E}[m\n"
    "Author: A <a@example.com>\n",
    # grep --color
    f"{E}[35m{E}[Ksrc/ansi.rs{E}[m{E}[K{E}[36m{E}[K:{E}[m{E}[K{E}[32m{E}[K12{E}[m{E}[K{E}[36m{E}[K:"
    f"{E}[m{E}[Kfn {E}[01;31m{E}[Ktokenize{E}[m{E}[K(text: &str)\n",
    # pytest / npm style
    f"{E}[1m{E}[31mFAILED{E}[0m tests/test_x.py::test_y - {E}[31mAssertionError{E}[0m\n"
    f"{E}[32m=== {E}[32m{E}[1m3 passed{E}[0m{E}[32m in 0.12s ==={E}[0m",
    f"{E}[2K{E}[1G{E}[90m[{E}[39m{E}[32m##########{E}[39m{E}[90m.....{E}[39m] 66%",
    # Rich's own output
    f"{E}[1;35mHello{E}[0m, {E}[3;4;38;2;255;128;0mWorld{E}[0m!\n",
    f"{E}]8;id=1;https://rich.readthedocs.io{ST}{E}[4;34mdocs{E}[0m{E}]8;;{ST}\n",
    f"{E}[38;2;0;0;0;48;2;255;255;255m inverted {E}[0m {E}[7mreverse{E}[27m done",
    # tmux / vim style screen updates
    f"{E}[?1049h{E}[H{E}[2J{E}[1;1H{E}[1mtitle{E}[22m{E}[2;1Hline two{E}[?25l{E}[?1049l",
    f"{E}7{E}[5;10H{E}[7m status {E}[27m{E}8after",
    f"{E}]0;my title{ST}{E}(B{E}[mprompt$ ",
]

EDGE = [
    "",
    "plain",
    "\n",
    "a\nb\n",
    "a\r\nb\r\n",
    "50%\r100%",
    f"{sgr(1)}50%\r100%",
    f"{sgr(1)}a\nb{sgr(0)}\nc",
    "x\x07y\x08z\x0bw\x0cv",
    "tab\there",
    "日本語 and 😀 emoji",
    f"{sgr(31)}é{sgr(0)}",
    f"{E}[m",
    f"{sgr(1)}b{E}[mx",
    f"{sgr(1, '', 3)}x",
    f"{E}[1;;3mx",
    f"{E}[4:3mcurly",
    f"{E}[999mx{E}[300;1my",
    f"{E}[38;5mx",
    f"{E}[38;5;300mx",
    f"{E}[38;2;1;2mx",
    f"{E}[48;2;1;2;3mx{E}[49my",
    f"{E}[38;9;1mx",
    f"{E}[38mx",
    f"{E}[1;2mA{E}[22mB",
    f"{E}[51;52mA{E}[54mB",
    f"{E}[21;4mA{E}[24mB",
    f"{E}[1;31mhi{E}[22;39mz",
    f"{E}[91;101mbright{E}[90;100mgrey",
    f"{E}[1;2",
    f"{E}",
    f"x{E}",
    f"{E}[",
    f"{E}[1",
    f"{E}(",
    f"{E}(Bxy",
    f"{E}({E}[1mz",
    f"{E}]8;;http://a{ST}one{E}]8;;{ST}two",
    f"{E}]8;id=3;a;b{ST}L{E}[0mM",
    f"{E}]8;;http://a{ST}{sgr(1)}x{sgr(0)}y",
    f"{E}]8{ST}x",
    f"{E}]8;{ST}x",
    f"{E}]unterminated text",
    f"{E}]8;;http://a\nb{ST}c",
    f"{E}[?25lhidden cursor{E}[?25h",
    f"{E}[2J{E}[Hcleared",
    f"{E}[3Aup{E}[2Bdown{E}[10Cfwd{E}[4Dback",
    f"{E}Mreverse index{E}Dindex",
    f"{E}=keypad{E}>",
    f"{E}[>4;2mmodify",
    f"{E}[ qcursor shape",
    f"{E}[38;5;16mcube{E}[38;5;231mwhite{E}[38;5;255mgrey",
    f"{E}[38;5;0;48;5;15mstd",
]

ATTR_CODES = [1, 2, 3, 4, 5, 6, 7, 8, 9, 21, 22, 23, 24, 25, 26, 27, 28, 29,
              51, 52, 53, 54, 55]
COLOR_CODES = list(range(30, 38)) + [39] + list(range(40, 48)) + [49] + \
    list(range(90, 98)) + list(range(100, 108))
UNKNOWN_CODES = [10, 11, 20, 50, 56, 60, 99, 108, 200, 256, 1000]
WORDS = ["foo", "bar baz", "x", "日本", "😀", "naïve", "  ", "a\tb", "→", "Ω",
         "é", "0", "-->", "|"]
OTHER_ESC = [f"{E}[K", f"{E}[2K", f"{E}[1G", f"{E}[?25l", f"{E}[?25h", f"{E}[3A",
             f"{E}7", f"{E}8", f"{E}(B", f"{E}M", f"{E}]0;title{ST}", f"{E}[5;5H",
             f"{E}[6n", f"{E}c"]


def random_sgr(rng: random.Random) -> str:
    params = []
    for _ in range(rng.randint(0, 4)):
        kind = rng.random()
        if kind < 0.35:
            params.append(str(rng.choice(ATTR_CODES)))
        elif kind < 0.6:
            params.append(str(rng.choice(COLOR_CODES)))
        elif kind < 0.7:
            params.append("0")
        elif kind < 0.8:
            params.append(f"{rng.choice([38, 48])};5;{rng.randint(0, 260)}")
        elif kind < 0.9:
            rgb = ";".join(str(rng.randint(0, 255)) for _ in range(3))
            params.append(f"{rng.choice([38, 48])};2;{rgb}")
        elif kind < 0.95:
            params.append(str(rng.choice(UNKNOWN_CODES)))
        else:
            params.append(rng.choice(["", "1:2", "38;5", "48;2;1"]))
    return f"{E}[{';'.join(params)}m"


def random_case(rng: random.Random) -> str:
    parts = []
    for _ in range(rng.randint(1, 14)):
        kind = rng.random()
        if kind < 0.4:
            parts.append(rng.choice(WORDS))
        elif kind < 0.75:
            parts.append(random_sgr(rng))
        elif kind < 0.82:
            parts.append(link(rng.choice(["https://a.example/x", "file:///tmp/f;1", ""]),
                              rng.choice(["", "id=7"])))
        elif kind < 0.9:
            parts.append(rng.choice(OTHER_ESC))
        elif kind < 0.97:
            parts.append("\n")
        else:
            parts.append(rng.choice(["\r", "\x07", "\r\n"]))
    return "".join(parts)


def main() -> None:
    print(f"# rich {version('rich')}: <input hex> <plain hex> <runs>")
    rng = random.Random(2026)
    cases = REAL + EDGE + [random_case(rng) for _ in range(140)]
    for case in cases:
        print(record(case))


if __name__ == "__main__":
    main()
