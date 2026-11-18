#!/usr/bin/env python3
"""Generate tests/fixtures/text_ops.golden from Python Rich.

Runs sequences of `rich.text.Text` operations (stylize, divide, split,
truncate, join, expand_tabs, ...) and records the plain text and the style
of every character of each resulting `Text`. `tests/text_parity.rs` replays
the same operations on fast-rich's `Text` and compares.

Usage (needs `rich` importable, e.g. `pip install rich==15.0.0`):

    python3 scripts/gen_text_golden.py > tests/fixtures/text_ops.golden

Format (one record per line, strings are UTF-8 hex, `-` = empty/none,
styles are Rich style strings with spaces written as `+`, offsets and
counts are in characters):

    C                      start a case: one empty Text
    B <style>              set the base style (Text.style)
    T <hex> <style>        append a part (Text.append(str, style))
    O <op> <args...>       apply an operation to every Text in the case
    P <hex>                result: plain text of the next Text
    R <start> <end> <desc> result: style run of that Text (char indices)
    N <n>                  result: a count returned by an operation
    S <desc>               result: get_style_at_offset output

Where Rich has behavior fast-rich deliberately does not copy, the cases
avoid it: `set_length`/`fit`/`rstrip_end` count characters in Rich but
cells in fast-rich, so they only run on texts whose characters are all one
cell wide; `right_crop(0)` and `remove_suffix("")` (which empty the text in
Rich), `right_crop` past the length (which makes Rich's length negative)
and `truncate` to width 0 with an ellipsis are not generated.
"""

import random
import re
from importlib.metadata import version

from rich.cells import cell_len
from rich.console import Console
from rich.text import Text

CONSOLE = Console(color_system="truecolor", force_terminal=True)

STYLES = [
    "bold", "italic", "red", "blue", "green on yellow", "bold red",
    "underline magenta", "dim", "strike", "reverse cyan", "on blue",
]

# Hand-picked cases: (base_style, [(text, style)], [op, ...]).
CASES = [
    ("", [("hello world", "")], [("stylize", "bold", 2, 7)]),
    ("", [("hello", "red"), (" world", "")], [("stylize", "blue", 3, 8)]),
    ("", [("hello", "red"), (" world", "")], [("stylize_before", "blue bold", 3, 8)]),
    ("", [("abc", "")], [("stylize", "bold", 5, 9), ("stylize", "red", 2, 1)]),
    ("", [("abc", "")], [("stylize", "bold", 1, 99)]),
    ("italic", [("é日本🎉", "")], [("stylize", "red", 1, 3), ("style_at", 2)]),
    ("", [("", "")], [("stylize", "bold", 0, 3)]),
    ("", [("Error error ERROR", "")], [("highlight_words", "bold", 1, ["error"])]),
    ("", [("Error error ERROR", "")], [("highlight_words", "bold", 0, ["error"])]),
    ("", [("abcabc aaa", "")], [("highlight_words", "red", 1, ["ab", "abc", "aa"])]),
    ("", [("Été été ÉTÉ 日本", "")], [("highlight_words", "red", 0, ["été", "日本"])]),
    ("", [("port 8080 and 443", "")], [("highlight_regex", "bold", r"[0-9]+")]),
    ("", [("a-b c--d", "dim")], [("highlight_regex", "red", r"\w+")]),
    ("", [("key", "bold")], [("append_tokens", "=", ""), ("append_tokens", "1", "blue")]),
    ("", [("a ", "")], [("append_text", "base", "red"), ("stylize", "bold", 1, 4)]),
    ("", [("abcdef", "")], [("stylize", "bold", 1, 5), ("divide", [2, 4])]),
    ("red", [("abcdef", "")], [("divide", [])]),
    ("", [("abc", "")], [("divide", [0, 3])]),
    ("", [("é日本🎉", "bold")], [("divide", [1, 3])]),
    ("", [("a\n\nb\n", "")], [("split", "\n", 0)]),
    ("", [("a\n\nb\n", "red")], [("split", "\n", 1)]),
    ("", [("one, two", "bold"), (", three", "")], [("split", ", ", 0)]),
    ("", [("no separator", "")], [("split", ",", 0)]),
    ("", [("a b c", "")], [("split", " ", 0), ("join", ", ", "red")]),
    ("", [("a b", "bold")], [("split", " ", 0), ("join", "", "")]),
    ("dim", [("x y", "")], [("split", " ", 0), ("join", "-", "")]),
    ("", [("hello", "")], [("right_crop", 2)]),
    ("", [("é日本🎉", "red")], [("right_crop", 1)]),
    ("", [("abc", "")], [("right_crop", 10)]),
    ("", [("hello world", "red")], [("stylize", "bold", 0, 5), ("truncate", 5, "ellipsis", 0)]),
    ("", [("hello", "")], [("truncate", 3, "crop", 0)]),
    ("", [("hello", "")], [("truncate", 3, "ignore", 1)]),
    ("", [("hello", "")], [("truncate", 3, "fold", 0)]),
    ("", [("日本語", "bold"), ("x", "red")], [("truncate", 4, "ellipsis", 0)]),
    ("", [("日本語", ""), ("x", "red")], [("truncate", 3, "crop", 0)]),
    ("", [("hi", "bold")], [("truncate", 5, "crop", 1)]),
    ("", [("x", "bold")], [("pad", 2, ".")]),
    ("", [("7", "red")], [("pad_left", 2, "0"), ("pad_right", 1, "日")]),
    ("", [("abc", "bold")], [("set_length", 5), ("set_length", 2)]),
    ("", [("hello\nhi\n", "red")], [("fit", 3)]),
    ("", [("a\tbc\t日\tx", "")], [("stylize", "red", 1, 2), ("expand_tabs", 4)]),
    ("", [("ab\n", ""), ("\tc\t", "bold")], [("expand_tabs", 4)]),
    ("", [("12345678\tx", "")], [("expand_tabs", 8)]),
    ("", [("done", ""), ("  \t\n", "bold")], [("rstrip",)]),
    ("", [("ab    ", "red")], [("rstrip_end", 3)]),
    ("", [("ab    ", "red")], [("rstrip_end", 0)]),
    ("", [("report.txt", "bold")], [("remove_suffix", ".txt")]),
    ("", [("report.txt", "")], [("remove_suffix", ".py")]),
    ("", [("abcdef", "")], [("copy_styles", "bold", 1, 3), ("copy_styles", "red", 4, 99)]),
    ("italic", [("ab", ""), ("cd", "red")], [("style_at", 0), ("style_at", 3), ("style_at", 9)]),
    ("", [("fn main() {\n    if x {\n        y();\n    }\n}", "")], [("detect_indentation",)]),
    ("", [("a\n   b\n      c", "")], [("detect_indentation",)]),
    ("", [("", "")], [("detect_indentation",)]),
    ("", [("if x:\n    y()\n        z()", "")], [("indent_guides", 4, "│", "dim green")]),
    ("", [("a:\n    b\n\n    c\n\n", "red")], [("indent_guides", 4, "│", "bold")]),
    ("", [("a:\n  b\n    c\n", "")], [("indent_guides", None, "|", "blue")]),
    ("bold", [("x\n\ty", "")], [("stylize", "red", 3, 4), ("indent_guides", 4, "|", "dim")]),
]

TOKENS = [
    "hello", "world", " ", "  ", "\t", "\n", "é", "日本", "🎉", "abc", "x",
    "ab\tc", "12", "    ind", "a.b", ",", ", ", "É", "-", "\n\n", "ÉTÉ",
]
SEPARATORS = ["\n", " ", ",", ", ", "a", "ab", "日", "🎉"]
REGEXES = [r"[0-9]+", r"\w+", r"[a-c]", r" +", r"é|日本", r"\s+", r"h.l"]
WORDS = ["hello", "é", "ab", "abc", "日本", "x", "É", " ", "l", "lo"]


def single_cell(texts):
    return all(cell_len(ch) == 1 for t in texts for ch in t.plain)


def random_op(rng, texts):
    length = max((len(t) for t in texts), default=0)
    shortest = min((len(t) for t in texts), default=0)
    style = rng.choice(STYLES)

    def offset():
        return rng.randint(0, length + 2)

    choices = [
        lambda: ("stylize", style, offset(), offset()),
        lambda: ("stylize_before", style, offset(), offset()),
        lambda: ("highlight_words", style, rng.randint(0, 1),
                 rng.sample(WORDS, rng.randint(1, 3))),
        lambda: ("highlight_regex", style, rng.choice(REGEXES)),
        lambda: ("append_tokens", rng.choice(TOKENS), rng.choice(STYLES + [""])),
        lambda: ("append_text", rng.choice(TOKENS), rng.choice(STYLES + [""])),
        lambda: ("divide", sorted(rng.randint(0, length) for _ in range(rng.randint(0, 3)))),
        lambda: ("split", rng.choice(SEPARATORS), rng.randint(0, 1)),
        lambda: ("join", rng.choice(["", " ", ", ", "|"]), rng.choice(STYLES + [""])),
        lambda: ("truncate", rng.randint(1, length + 2),
                 rng.choice(["crop", "fold", "ellipsis", "ignore"]), rng.randint(0, 1)),
        lambda: ("pad", rng.randint(0, 3), rng.choice([" ", ".", "日"])),
        lambda: ("pad_left", rng.randint(0, 3), rng.choice([" ", "-"])),
        lambda: ("pad_right", rng.randint(0, 3), rng.choice([" ", "-"])),
        lambda: ("expand_tabs", rng.randint(1, 8)),
        lambda: ("rstrip",),
        lambda: ("remove_suffix", rng.choice(TOKENS)),
        lambda: ("copy_styles", style, offset(), offset()),
        lambda: ("style_at", offset()),
        lambda: ("detect_indentation",),
        lambda: ("indent_guides", rng.choice([None, 1, 2, 4]), rng.choice(["│", "|"]), style),
    ]
    if shortest:
        # Cropping more than the length leaves Rich with a negative length.
        choices.append(lambda: ("right_crop", rng.randint(1, shortest)))
    if single_cell(texts):
        choices += [
            lambda: ("set_length", rng.randint(0, length + 3)),
            lambda: ("fit", rng.randint(1, length + 3)),
            lambda: ("rstrip_end", rng.randint(0, length + 1)),
        ]
    return rng.choice(choices)()


def random_cases(n, seed=20261118):
    rng = random.Random(seed)
    for _ in range(n):
        base = rng.choice(["", "", "", "italic", "on blue"])
        parts = [
            ("".join(rng.choice(TOKENS) for _ in range(rng.randint(1, 4))),
             rng.choice(STYLES + ["", ""]))
            for _ in range(rng.randint(1, 4))
        ]
        texts = [build(base, parts)]
        ops = []
        for _ in range(rng.randint(1, 4)):
            op = random_op(rng, texts)
            ops.append(op)
            texts, _ = apply(texts, op)
        yield base, parts, ops


def build(base, parts):
    text = Text(style=base)
    for content, style in parts:
        text.append(content, style or None)
    return text


def apply(texts, op):
    """Apply `op` to every text; return (new texts, extra output lines)."""
    name, *args = op
    out = []
    new = []
    if name == "join":
        sep, style = args
        return [Text(sep, style=style).join(texts)], out
    for text in texts:
        if name == "stylize":
            text.stylize(args[0], args[1], args[2])
        elif name == "stylize_before":
            text.stylize_before(args[0], args[1], args[2])
        elif name == "highlight_words":
            out.append(f"N {text.highlight_words(args[2], args[0], case_sensitive=bool(args[1]))}")
        elif name == "highlight_regex":
            out.append(f"N {text.highlight_regex(re.compile(args[1]), args[0])}")
        elif name == "append_tokens":
            text.append_tokens([(args[0], args[1] or None)])
        elif name == "append_text":
            text.append_text(Text(args[0], style=args[1]))
        elif name == "divide":
            new.extend(text.divide(args[0]))
            continue
        elif name == "split":
            new.extend(text.split(args[0], include_separator=bool(args[1])))
            continue
        elif name == "right_crop":
            text.right_crop(args[0])
        elif name == "truncate":
            text.truncate(args[0], overflow=args[1], pad=bool(args[2]))
        elif name == "pad":
            text.pad(args[0], args[1])
        elif name == "pad_left":
            text.pad_left(args[0], args[1])
        elif name == "pad_right":
            text.pad_right(args[0], args[1])
        elif name == "set_length":
            text.set_length(args[0])
        elif name == "fit":
            new.extend(text.fit(args[0]))
            continue
        elif name == "expand_tabs":
            text.expand_tabs(args[0])
        elif name == "rstrip":
            text.rstrip()
        elif name == "rstrip_end":
            text.rstrip_end(args[0])
        elif name == "remove_suffix":
            text.remove_suffix(args[0])
        elif name == "copy_styles":
            other = Text(text.plain)
            other.stylize(args[0], args[1], args[2])
            text.copy_styles(other)
        elif name == "style_at":
            out.append(f"S {describe(text.get_style_at_offset(CONSOLE, args[0])) or '-'}")
        elif name == "detect_indentation":
            out.append(f"N {text.detect_indentation()}")
        elif name == "indent_guides":
            text = text.with_indent_guides(args[0], character=args[1], style=args[2])
        else:
            raise ValueError(name)
        new.append(text)
    return new, out


def hexs(s):
    return s.encode("utf-8").hex() or "-"


def style_arg(style):
    return style.replace(" ", "+") or "-"


def encode_op(op):
    name, *args = op
    enc = {
        "stylize": lambda: [style_arg(args[0]), args[1], args[2]],
        "stylize_before": lambda: [style_arg(args[0]), args[1], args[2]],
        "highlight_words": lambda: [style_arg(args[0]), args[1],
                                    ",".join(hexs(w) for w in args[2])],
        "highlight_regex": lambda: [style_arg(args[0]), hexs(args[1])],
        "append_tokens": lambda: [hexs(args[0]), style_arg(args[1])],
        "append_text": lambda: [hexs(args[0]), style_arg(args[1])],
        "divide": lambda: [",".join(map(str, args[0])) or "-"],
        "split": lambda: [hexs(args[0]), args[1]],
        "join": lambda: [hexs(args[0]), style_arg(args[1])],
        "truncate": lambda: [args[0], args[1], args[2]],
        "pad": lambda: [args[0], hexs(args[1])],
        "pad_left": lambda: [args[0], hexs(args[1])],
        "pad_right": lambda: [args[0], hexs(args[1])],
        "remove_suffix": lambda: [hexs(args[0])],
        "copy_styles": lambda: [style_arg(args[0]), args[1], args[2]],
        "indent_guides": lambda: ["-" if args[0] is None else args[0], hexs(args[1]),
                                  style_arg(args[2])],
    }.get(name, lambda: list(args))()
    return " ".join(["O", name, *map(str, enc)])


def describe(style):
    parts = []
    if style.color is not None and not style.color.is_default:
        parts.append(style.color.name)
    if style.bgcolor is not None and not style.bgcolor.is_default:
        parts.append("on_" + style.bgcolor.name)
    flags = "".join(
        flag
        for flag, on in (
            ("b", style.bold),
            ("d", style.dim),
            ("i", style.italic),
            ("u", style.underline),
            ("s", style.strike),
            ("r", style.reverse),
        )
        if on
    )
    if flags:
        parts.append(flags)
    return ":".join(parts)


def runs(text):
    out = []
    for i in range(len(text.plain)):
        desc = describe(text.get_style_at_offset(CONSOLE, i))
        if out and out[-1][2] == desc and out[-1][1] == i:
            out[-1][1] = i + 1
        else:
            out.append([i, i + 1, desc])
    return [r for r in out if r[2]]


def main():
    print(f"# Generated by scripts/gen_text_golden.py from rich {version('rich')}")
    for base, parts, ops in CASES + list(random_cases(600)):
        print("C")
        if base:
            print(f"B {style_arg(base)}")
        for content, style in parts:
            print(f"T {hexs(content)} {style_arg(style)}")
        texts = [build(base, parts)]
        for op in ops:
            print(encode_op(op))
            texts, out = apply(texts, op)
            for line in out:
                print(line)
        for text in texts:
            print(f"P {hexs(text.plain)}")
            for start, end, desc in runs(text):
                print(f"R {start} {end} {desc}")


if __name__ == "__main__":
    main()
