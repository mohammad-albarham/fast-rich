#!/usr/bin/env python3
"""Generate tests/fixtures/segment.golden from Python Rich.

Runs Rich's `Segment` helpers (`split_lines`, `split_and_crop_lines`,
`adjust_line_length`, `set_shape`, `align_*`, `simplify`, `divide`,
`split_cells`, ...) on fixed segment lists, and records every `Control`
escape string. `tests/segment_parity.rs` runs the same calls on fast-rich
and compares.

Usage (needs `rich` importable, e.g. `pip install rich==15.0.0`, or
`PYTHONPATH` pointed at an unzipped rich wheel):

    python3 scripts/gen_segment_golden.py > tests/fixtures/segment.golden

Format (one record per line, fields separated by ` ; `):

    <op> ; <args> ; <input> ; <output>

    seg   = <text hex>.<style hex>.<control 0|1>
    segs  = [seg,seg,...]
    lines = {segs segs ...}  (segs written back to back)

A style is described as in `scripts/gen_ansi_golden.py` (attributes,
`fg=`, `bg=`, `link=`); a style argument is its description in hex, or
`-` for None. `split_lines_terminator` outputs `lines|flags` (`t`/`f` per
line); `get_shape` outputs `w,h`; `control` records the escape in hex.

Inputs avoid one case where fast-rich differs: Rich's `simplify` merges a
plain unstyled segment into a control segment before it and drops the
control flag; fast-rich decides control-ness from the text, so the merged
span stays a control span.
"""

from importlib.metadata import version

from rich.color import ColorType
from rich.control import Control, escape_control_codes, strip_control_codes
from rich.segment import ControlType, Segment
from rich.style import Style

ATTRS = [
    "bold", "dim", "italic", "underline", "blink", "blink2", "reverse",
    "conceal", "strike", "underline2", "frame", "encircle", "overline",
]


def color_desc(color) -> str:
    if color.type == ColorType.DEFAULT:
        return "default"
    if color.type == ColorType.TRUECOLOR:
        return "#%02x%02x%02x" % tuple(color.triplet)
    return str(color.number)


def style_desc(style) -> str:
    if style is None:
        return ""
    words = [name for name in ATTRS if getattr(style, name) is True]
    if style.color is not None:
        words.append("fg=" + color_desc(style.color))
    if style.bgcolor is not None:
        words.append("bg=" + color_desc(style.bgcolor))
    if style.link:
        words.append("link=" + style.link)
    return " ".join(words)


def hx(s: str) -> str:
    return s.encode().hex()


def enc_style(style) -> str:
    return "-" if style is None else hx(style_desc(style))


def enc_seg(seg: Segment) -> str:
    return f"{hx(seg.text)}.{hx(style_desc(seg.style))}.{1 if seg.control else 0}"


def enc_segs(segs) -> str:
    return "[" + ",".join(enc_seg(s) for s in segs) + "]"


def enc_lines(lines) -> str:
    return "{" + "".join(enc_segs(line) for line in lines) + "}"


def out(op, args, inp, result):
    print(f"{op} ; {args} ; {inp} ; {result}")


LINK = Style(link="https://example.com/a;b")
S1 = [
    Segment("Hello ", Style.parse("bold red")),
    Segment("World\n", Style.parse("italic")),
    Segment("foo"),
    Segment("\nbar\n\n", LINK),
    Segment("日本語", Style.parse("blue on white")),
    Segment("x"),
]
S2 = [
    Segment("ab"),
    Control.move_to(1, 2).segment,
    Segment("cd\n", Style.parse("bold")),
    Control.bell().segment,
    Segment("e😀f", Style.parse("underline #ff8000")),
]
S3 = [Segment("a日b本"), Segment("éx", Style.parse("bold")), Segment("😀😀")]
S5 = [
    Segment(""),
    Segment("abc", Style.parse("bold")),
    Segment("", Style.parse("italic")),
    Segment("abc", Style.parse("bold")),
    Segment("abc", Style.parse("bold") + LINK),
    Segment("d", Style.parse("color(200) on default")),
]
S7 = [
    Segment("The quick brown fox ", Style.parse("green")),
    Segment("jumps over", Style.parse("bold")),
    Segment(" the lazy dog"),
]
SEGMENT_LISTS = [S1, S2, S3, [], S5, S7]

STYLE_ARGS = [None, Style.parse("on red"), Style.parse("bold #102030")]


def lines_inputs():
    lines = [list(Segment.split_lines(s)) for s in (S1, S2, S3, S7)]
    lines.append([])
    lines.append([[], [Segment("x")]])
    return lines


def main() -> None:
    print(f"# rich {version('rich')}: <op> ; <args> ; <input> ; <output>")

    out("line", "-", "[]", enc_segs([Segment.line()]))

    for segs in SEGMENT_LISTS:
        inp = enc_segs(segs)
        out("split_lines", "-", inp, enc_lines(Segment.split_lines(segs)))
        result = list(Segment.split_lines_terminator(segs))
        flags = "".join("t" if nl else "f" for _, nl in result) or "-"
        out("split_lines_terminator", "-", inp,
            enc_lines([line for line, _ in result]) + "|" + flags)
        for length in (0, 1, 3, 4, 5, 7, 10, 40):
            for pad in (True, False):
                for include in (True, False):
                    for style in STYLE_ARGS[:2]:
                        res = Segment.split_and_crop_lines(
                            segs, length, style=style, pad=pad,
                            include_new_lines=include)
                        args = f"{length} {int(pad)} {int(include)} {enc_style(style)}"
                        out("split_and_crop_lines", args, inp, enc_lines(res))
        out("get_line_length", "-", inp, str(Segment.get_line_length(segs)))
        out("simplify", "-", inp, enc_segs(Segment.simplify(segs)))
        out("strip_links", "-", inp, enc_segs(Segment.strip_links(segs)))
        out("strip_styles", "-", inp, enc_segs(Segment.strip_styles(segs)))
        out("remove_color", "-", inp, enc_segs(Segment.remove_color(segs)))
        for is_control in (False, True):
            out("filter_control", str(int(is_control)), inp,
                enc_segs(Segment.filter_control(segs, is_control)))
        for style in STYLE_ARGS:
            for post in STYLE_ARGS:
                out("apply_style", f"{enc_style(style)} {enc_style(post)}", inp,
                    enc_segs(Segment.apply_style(segs, style, post)))
        for cuts in ([], [0], [0, 0], [3], [3, 6], list(range(1, 21)), [5, 100],
                     [2, 2, 4], [100], [0, 7, 7, 9]):
            args = ",".join(map(str, cuts)) or "-"
            out("divide", args, inp, enc_lines(Segment.divide(segs, cuts)))
        # Rich's split_cells raises on control segments (its cache cannot
        # hash their code list), so only text segments are split.
        for seg in (s for s in segs if not s.control):
            for cut in range(0, seg.cell_length + 2):
                out("split_cells", str(cut), enc_segs([seg]),
                    enc_segs(seg.split_cells(cut)))

    for lines in lines_inputs():
        inp = enc_lines(lines)
        w, h = Segment.get_shape(lines)
        out("get_shape", "-", inp, f"{w},{h}")
        for line in lines:
            for length in (0, 1, 2, 3, 5, 8, 12):
                for pad in (True, False):
                    out("adjust_line_length", f"{length} {int(pad)} {enc_style(STYLE_ARGS[1])}",
                        enc_segs(line),
                        enc_segs(Segment.adjust_line_length(line, length, STYLE_ARGS[1], pad)))
        for width in (0, 3, 5, 8):
            for height in (None, 0, 1, 3, 6):
                for new_lines in (False, True):
                    style = STYLE_ARGS[1]
                    hs = "-" if height is None else str(height)
                    args = f"{width} {hs} {enc_style(style)} {int(new_lines)}"
                    out("set_shape", args, inp,
                        enc_lines(Segment.set_shape(lines, width, height, style, new_lines)))
                    if height is None:
                        continue
                    for name in ("align_top", "align_middle", "align_bottom"):
                        res = getattr(Segment, name)(lines, width, height, style, new_lines)
                        out(name, args, inp, enc_lines(res))

    controls = [
        ("bell", Control.bell()),
        ("home", Control.home()),
        ("clear", Control.clear()),
        ("move 0 0", Control.move(0, 0)),
        ("move 3 0", Control.move(3, 0)),
        ("move -3 0", Control.move(-3, 0)),
        ("move 0 2", Control.move(0, 2)),
        ("move 0 -2", Control.move(0, -2)),
        ("move 5 -7", Control.move(5, -7)),
        ("move_to_column 0 0", Control.move_to_column(0)),
        ("move_to_column 4 1", Control.move_to_column(4, 1)),
        ("move_to_column 4 -1", Control.move_to_column(4, -1)),
        ("move_to 0 0", Control.move_to(0, 0)),
        ("move_to 10 3", Control.move_to(10, 3)),
        ("show_cursor 1", Control.show_cursor(True)),
        ("show_cursor 0", Control.show_cursor(False)),
        ("alt_screen 1", Control.alt_screen(True)),
        ("alt_screen 0", Control.alt_screen(False)),
        ("title hello", Control.title("hello")),
        ("title " + hx("🚀 Loading..."), Control.title("🚀 Loading...")),
        ("codes", Control()),
        ("codes carriage_return", Control(ControlType.CARRIAGE_RETURN)),
        ("codes erase_in_line:2", Control((ControlType.ERASE_IN_LINE, 2))),
        ("codes erase_in_line:0 cursor_up:1 cursor_down:0",
         Control((ControlType.ERASE_IN_LINE, 0), (ControlType.CURSOR_UP, 1),
                 (ControlType.CURSOR_DOWN, 0))),
        ("codes enable_alt_screen disable_alt_screen show_cursor hide_cursor",
         Control(ControlType.ENABLE_ALT_SCREEN, ControlType.DISABLE_ALT_SCREEN,
                 ControlType.SHOW_CURSOR, ControlType.HIDE_CURSOR)),
        ("codes cursor_forward:12 cursor_backward:1 cursor_move_to_column:9",
         Control((ControlType.CURSOR_FORWARD, 12), (ControlType.CURSOR_BACKWARD, 1),
                 (ControlType.CURSOR_MOVE_TO_COLUMN, 9))),
    ]
    for name, control in controls:
        out("control", name, "-", hx(str(control)))
    sample = "a\x07b\x08c\x0bd\x0ce\rf\ng\th\x1b"
    out("strip_control_codes", "-", hx(sample), hx(strip_control_codes(sample)))
    out("escape_control_codes", "-", hx(sample), hx(escape_control_codes(sample)))


if __name__ == "__main__":
    main()
