"""Feature-by-feature test of an installed fast-rich wheel.

Every feature is a small snippet written against the *Rich* API (fast-rich
advertises itself as a drop-in replacement). Each snippet runs in its own
subprocess with stdout/stderr piped, so output from both the Python layer and
the native Rust extension is captured.

The same snippet is also run against real `rich` (if installed) as a reference:
  * if Rich itself fails a check, the test is wrong  -> reported as INVALID
  * otherwise we also report whether fast-rich output is identical to Rich's.

A second group exercises the native extension `fast_rich._core` directly.

Statuses:
  PASS     feature works (all checks satisfied)
  WRONG    runs, but output/behaviour is wrong (failed check or assertion)
  MISSING  API not implemented (ImportError / AttributeError / unsupported kwarg)
  ERROR    raises another exception
  HANG     did not finish within the timeout

Usage: python feature_test.py [--json OUT.json] [--only SUBSTR] [--timeout S]
"""

from __future__ import annotations

import argparse
import concurrent.futures as cf
import json
import os
import platform
import re
import subprocess
import sys
import tempfile
import textwrap
import unicodedata

# --------------------------------------------------------------------------- #
# Feature definitions
# --------------------------------------------------------------------------- #
# Keys:
#   id, group, name, code      snippet; "PKG" is replaced by fast_rich / rich
#   stdin                      text fed to stdin (default: empty)
#   expect / forbid            substrings that must / must not appear (ANSI-stripped)
#   regex                      regexes (re.M) that must match the plain output
#   raw_expect / raw_forbid    regexes matched against the raw output (with ANSI)
#   max_width                  every line's display width must be <= N
#   line_width                 every non-empty line's display width must == N
#   equal_width                every non-empty line has the same display width
#   exit_nonzero               the snippet is expected to exit with an error
#   allow_tb                   output may contain a Python traceback
#   native                     only run against fast_rich (no Rich reference)

PRELUDE = "import io, os, sys\nfrom PKG.console import Console\nconsole = Console()\n"

FEATURES: list[dict] = []


def F(id, group, name, code, **kw):
    FEATURES.append(dict(id=id, group=group, name=name, code=textwrap.dedent(code), **kw))


# ---- Console -------------------------------------------------------------- #
G = "Console"
F("console.print", G, "print plain text", 'console.print("hello world")', expect=["hello world"])
F("console.markup", G, "print with markup tags", 'console.print("[bold red]alert[/bold red] done")',
  expect=["alert done"], forbid=["[bold", "[/bold"])
F("console.markup_nested", G, "nested markup", 'console.print("[bold]a [italic]b[/italic] c[/bold]")',
  expect=["a b c"], forbid=["[italic", "[/"])
F("console.markup_escape", G, "escaped markup \\[ is literal", r'console.print("\\[not a tag]")',
  expect=["[not a tag]"])
F("console.multiline", G, "multi-line string keeps newlines", 'console.print("line1\\nline2\\nline3")',
  regex=[r"line1[ \t]*\n[ \t]*line2[ \t]*\n[ \t]*line3"])
F("console.style_kw", G, "print(style=...)", 'console.print("styled", style="bold red")', expect=["styled"])
F("console.ansi_terminal", G, "ANSI styling when force_terminal=True",
  'Console(force_terminal=True).print("[bold]B[/bold]")', expect=["B"], raw_expect=[r"\x1b\[[0-9;]*m"])
F("console.no_ansi_piped", G, "no ANSI codes when output is piped",
  'console.print("[bold red]piped[/]")', expect=["piped"], raw_forbid=[r"\x1b\["])
F("console.justify", G, "justify='center'", 'Console(width=40).print("hi", justify="center")',
  regex=[r"^ {17,20}hi"])
F("console.wrap", G, "wraps long text at console width", 'Console(width=40).print("word " * 30)',
  expect=["word"], max_width=40)
F("console.wrap_cjk", G, "wraps CJK text by display width", 'Console(width=10).print("日本語日本語日本語日本語")',
  expect=["日本"], max_width=10)
F("console.no_wrap_ellipsis", G, "no_wrap + overflow='ellipsis'",
  'Console(width=10).print("abcdefghijklmnop", no_wrap=True, overflow="ellipsis")',
  regex=[r"^abcdefghi…\s*$"])
F("console.emoji_code", G, "emoji shortcodes :smile:", 'console.print(":smile: hi")', expect=["😄 hi"])
F("console.emoji_time", G, "text like 12:30:45 is not mangled", 'console.print("at 12:30:45 ok")',
  expect=["at 12:30:45 ok"])
F("console.link", G, "[link=...] markup (OSC 8 hyperlink)",
  'Console(force_terminal=True).print("[link=https://example.com]click[/link]")',
  expect=["click"], forbid=["[link"], raw_expect=[r"\x1b\]8;"])
F("console.highlight", G, "auto-highlights numbers (force_terminal)",
  'Console(force_terminal=True).print("count 12345 items")', raw_expect=[r"\x1b\[[0-9;]+m12345"])
F("console.print_dict", G, "print a dict (pretty repr)", 'console.print({"a": 1, "b": [1, 2]})',
  expect=["'a': 1", "'b': [1, 2]"])
F("console.multi_objects", G, "print several objects with sep", 'console.print("x", 1, "y", sep="|")',
  expect=["x|1|y"])
F("console.rich_protocol", G, "__rich__ protocol", '''
class X:
    def __rich__(self):
        return "[bold]via rich protocol[/bold]"
console.print(X())
''', expect=["via rich protocol"], forbid=["[bold"])
F("console.rich_console_protocol", G, "__rich_console__ protocol", '''
from PKG.text import Text
class R:
    def __rich_console__(self, console, options):
        yield "line one"
        yield Text("line two")
console.print(R())
''', regex=[r"line one\s*\n\s*line two"])
F("console.log", G, "console.log", 'console.log("log message here")', expect=["log message here"])
F("console.rule", G, "console.rule(title)", 'console.rule("Section")', expect=["Section", "─"])
F("console.print_json", G, "console.print_json(str)", 'console.print_json(\'{"a": 1, "b": [true, null]}\')',
  expect=['"a": 1', "true", "null"])
F("console.print_json_data", G, "console.print_json(data=...)", 'console.print_json(data={"k": [1, 2]})',
  expect=['"k": ['])
F("console.input", G, "console.input reads stdin", 'v = console.input("Q? ")\nprint("GOT", v)',
  stdin="typed\n", expect=["GOT typed"])
F("console.file", G, "Console(file=StringIO)", '''
buf = io.StringIO()
c = Console(file=buf)
c.print("[bold]into file[/]")
s = buf.getvalue()
assert "into file" in s and "[bold]" not in s, repr(s)
print("OK")
''', expect=["OK"])
F("console.stderr", G, "Console(stderr=True) writes to stderr", '''
Console(stderr=True).print("to-stderr")
''', stderr_expect=["to-stderr"], stdout_forbid=["to-stderr"])
F("console.quiet", G, "Console(quiet=True) prints nothing", 'Console(quiet=True).print("hidden")',
  forbid=["hidden"])
F("console.size", G, "width / height / size", '''
c = Console(width=50, height=20)
assert c.width == 50 and c.height == 20, (c.width, c.height)
assert tuple(c.size) == (50, 20), c.size
print("OK")
''', expect=["OK"])
F("console.record_text", G, "record=True + export_text", '''
c = Console(record=True)
c.print("[bold]recorded[/]")
t = c.export_text()
assert "recorded" in t and "[bold]" not in t, repr(t)
print("OK")
''', expect=["OK"])
F("console.export_html", G, "export_html keeps styles", '''
c = Console(record=True)
c.print("[bold]recorded[/]")
h = c.export_html()
assert "recorded" in h, h
assert "bold" in h.lower(), "no style information in HTML: " + h[:200]
print("OK")
''', expect=["OK"])
F("console.export_svg", G, "export_svg", '''
c = Console(record=True)
c.print("[bold]recorded[/]")
s = c.export_svg()
assert "<svg" in s and "recorded" in s, s[:200]
assert "bold" in s.lower(), "no style information in SVG"
print("OK")
''', expect=["OK"])
F("console.export_svg_theme", G, "export_svg(theme=MONOKAI)", '''
from PKG.terminal_theme import MONOKAI
c = Console(record=True)
c.print("[red]themed[/]")
s = c.export_svg(theme=MONOKAI)
assert "<svg" in s, s[:200]
print("OK")
''', expect=["OK"])
F("console.status", G, "console.status(...) context manager", '''
import time
with console.status("Working..."):
    time.sleep(0.2)
print("done")
''', expect=["done"])
F("console.clear_screen", G, "console.clear() and console.screen()", '''
console.clear()
with console.screen():
    pass
print("done")
''', expect=["done"])
F("console.pager", G, "console.pager()", '''
with console.pager():
    console.print("paged text")
''', expect=["paged text"])
F("console.global_print", G, "from PKG import print", '''
from PKG import print
print("[bold]glob[/bold] ok", {"a": 1})
''', expect=["glob ok", "'a': 1"], forbid=["[bold"])
F("console.theme", G, "custom Theme style names", '''
from PKG.theme import Theme
Console(theme=Theme({"warning": "bold magenta"}), force_terminal=True).print("[warning]careful[/warning]")
''', expect=["careful"], forbid=["[warning"], raw_expect=[r"\x1b\[[0-9;]*m"])
F("console.options", G, "ConsoleOptions / console.options", '''
c = Console(width=70)
assert c.options.max_width == 70, c.options.max_width
print("OK")
''', expect=["OK"])

# ---- Text / Style / Color --------------------------------------------------- #
G = "Text & Style"
F("text.basic", G, "Text + append + plain/len", '''
from PKG.text import Text
t = Text("Hello", style="bold")
t.append(" World", style="red")
assert t.plain == "Hello World", t.plain
assert len(t) == 11, len(t)
console.print(t)
''', expect=["Hello World"])
F("text.from_markup", G, "Text.from_markup", '''
from PKG.text import Text
t = Text.from_markup("[bold]B[/bold] plain")
assert t.plain == "B plain", t.plain
assert len(t.spans) == 1, t.spans
print("OK")
''', expect=["OK"])
F("text.stylize", G, "Text.stylize(range)", '''
from PKG.text import Text
t = Text("abcdef")
t.stylize("bold", 0, 3)
assert len(t.spans) == 1 and (t.spans[0].start, t.spans[0].end) == (0, 3), t.spans
print("OK")
''', expect=["OK"])
F("text.assemble", G, "Text.assemble", '''
from PKG.text import Text
t = Text.assemble(("a", "bold"), "b", ("c", "red"))
assert t.plain == "abc", t.plain
print("OK")
''', expect=["OK"])
F("text.truncate", G, "Text.truncate(overflow='ellipsis')", '''
from PKG.text import Text
t = Text("hello world")
t.truncate(5, overflow="ellipsis")
assert t.plain == "hell…", repr(t.plain)
print("OK")
''', expect=["OK"])
F("text.highlight_regex", G, "Text.highlight_regex", '''
from PKG.text import Text
t = Text("foo 123 bar 45")
t.highlight_regex(r"\\d+", "bold")
assert len(t.spans) == 2, t.spans
print("OK")
''', expect=["OK"])
F("text.from_ansi", G, "Text.from_ansi", '''
from PKG.text import Text
t = Text.from_ansi("\\x1b[31mred\\x1b[0m text")
assert t.plain == "red text", repr(t.plain)
assert t.spans, "ANSI style not converted to a span"
print("OK")
''', expect=["OK"])
F("style.parse", G, "Style.parse('bold red on white')", '''
from PKG.style import Style
s = Style.parse("bold red on white")
assert s.bold is True, s.bold
assert s.color.name == "red", s.color
assert s.bgcolor.name == "white", s.bgcolor
print("OK")
''', expect=["OK"])
F("style.combine", G, "Style + Style", '''
from PKG.style import Style
s = Style(bold=True) + Style(italic=True)
assert s.bold and s.italic, s
print("OK")
''', expect=["OK"])
F("color.parse", G, "Color.parse hex / name / rgb", '''
from PKG.color import Color
assert tuple(Color.parse("#ff0000").triplet) == (255, 0, 0)
assert Color.parse("red").name == "red"
assert tuple(Color.parse("rgb(0,128,255)").triplet) == (0, 128, 255)
print("OK")
''', expect=["OK"])
F("color.invalid", G, "Color.parse rejects bad input cleanly", '''
from PKG.color import Color, ColorParseError
for bad in ["#日本", "#zzzzzz", "notacolor"]:
    try:
        Color.parse(bad)
    except ColorParseError:
        pass
    else:
        raise AssertionError("accepted " + bad)
print("OK")
''', expect=["OK"])
F("color.triplet", G, "ColorTriplet.hex", '''
from PKG.color_triplet import ColorTriplet
assert ColorTriplet(1, 2, 255).hex == "#0102ff"
print("OK")
''', expect=["OK"])
F("markup.escape", G, "markup.escape()", '''
from PKG.markup import escape
console.print(escape("[bold]literal[/bold]"))
''', expect=["[bold]literal[/bold]"])
F("cells.cell_len", G, "cell_len (CJK = 2 cells)", '''
from PKG.cells import cell_len
assert cell_len("日本") == 4, cell_len("日本")
assert cell_len("abc") == 3
print("OK")
''', expect=["OK"])
F("emoji.class", G, "Emoji('smile')", 'from PKG.emoji import Emoji\nconsole.print(Emoji("smile"))', expect=["😄"])
F("styled", G, "Styled(renderable, style)", 'from PKG.styled import Styled\nconsole.print(Styled("styled text", "bold"))',
  expect=["styled text"])

# ---- Table / Panel / Box --------------------------------------------------- #
G = "Table"
TABLE = '''
from PKG.table import Table
t = Table({args})
t.add_column("Name")
t.add_column("Age")
t.add_row("Alice", "30")
t.add_row("Bob", "4")
console.print(t)
'''
F("table.basic", G, "columns + rows", TABLE.format(args=""), expect=["Name", "Age", "Alice", "30", "Bob"],
  regex=[r"[┏╭┌+]"], equal_width=True)
F("table.title", G, "title + caption", TABLE.format(args='title="My Title", caption="my caption"'),
  expect=["My Title", "my caption", "Alice"])
F("table.no_header", G, "show_header=False", TABLE.format(args="show_header=False"),
  expect=["Alice"], forbid=["Name"])
F("table.box_ascii", G, "box=box.ASCII", 'from PKG import box\n' + TABLE.format(args="box=box.ASCII"),
  expect=["Alice", "+", "|"], forbid=["│", "─"])
F("table.box_styles", G, "box styles ROUNDED/SQUARE/HEAVY/DOUBLE", '''
from PKG import box
from PKG.table import Table
for b in (box.ROUNDED, box.SQUARE, box.HEAVY, box.DOUBLE, box.MINIMAL, box.SIMPLE, box.HORIZONTALS):
    t = Table(box=b)
    t.add_column("H")
    t.add_row("v")
    console.print(t)
''', expect=["╭", "┌", "┏", "╔"])
F("table.justify", G, "column justify='right'", '''
from PKG.table import Table
t = Table()
t.add_column("Number", justify="right")
t.add_row("1")
t.add_row("100")
console.print(t)
''', regex=[r" {4,}1 [│|┃]"])
F("table.renderable_cells", G, "Text / Panel inside cells", '''
from PKG.table import Table
from PKG.text import Text
from PKG.panel import Panel
t = Table()
t.add_column("A")
t.add_column("B")
t.add_row(Text("styled cell", style="bold"), Panel("inner"))
console.print(t)
''', expect=["styled cell", "inner"])
F("table.cjk", G, "CJK / emoji cells stay aligned", '''
from PKG.table import Table
t = Table()
t.add_column("Lang")
t.add_column("Word")
t.add_row("Japanese", "日本語")
t.add_row("Party", "🎉🎉")
t.add_row("ASCII", "abc")
console.print(t)
''', expect=["日本語", "abc"], equal_width=True)
F("table.expand", G, "expand=True fills console width", '''
from PKG.table import Table
t = Table(expand=True)
t.add_column("A")
t.add_row("x")
Console(width=60).print(t)
''', line_width=60)
F("table.wrap_cells", G, "long cells wrap within width", '''
from PKG.table import Table
t = Table()
t.add_column("Text")
t.add_row("lorem ipsum " * 20)
Console(width=40).print(t)
''', expect=["lorem"], max_width=40)
F("table.add_column_kwargs", G, "add_column(style, no_wrap, width)", '''
from PKG.table import Table
t = Table()
t.add_column("A", style="cyan", no_wrap=True, width=12)
t.add_row("value")
console.print(t)
''', expect=["value"])

G = "Panel"
F("panel.basic", G, "Panel(content)", 'from PKG.panel import Panel\nconsole.print(Panel("content"))',
  expect=["content", "╭", "╯"], equal_width=True)
F("panel.expand", G, "Panel expands to console width (Rich default)",
  'from PKG.panel import Panel\nconsole.print(Panel("content"))', line_width=80)
F("panel.title", G, "title + subtitle", 'from PKG.panel import Panel\nconsole.print(Panel("body", title="Title", subtitle="Sub"))',
  expect=["body", "Title", "Sub"])
F("panel.fit", G, "Panel.fit", 'from PKG.panel import Panel\nconsole.print(Panel.fit("fit me"))', line_width=10)
F("panel.box", G, "box=box.DOUBLE", 'from PKG import box\nfrom PKG.panel import Panel\nconsole.print(Panel("d", box=box.DOUBLE))',
  expect=["╔", "d"])
F("panel.multiline", G, "multi-line content", 'from PKG.panel import Panel\nconsole.print(Panel("a\\nbb\\nccc"))',
  expect=["a", "bb", "ccc"], equal_width=True)
F("panel.cjk", G, "CJK content stays aligned", 'from PKG.panel import Panel\nconsole.print(Panel.fit("日本語\\nabc"))',
  expect=["日本語"], equal_width=True)
F("panel.nested_table", G, "Table inside Panel", '''
from PKG.panel import Panel
from PKG.table import Table
t = Table()
t.add_column("Name")
t.add_row("Alice")
console.print(Panel(t, title="People"))
''', expect=["Name", "Alice", "People"])
F("panel.markup", G, "markup inside panel", 'from PKG.panel import Panel\nconsole.print(Panel("[bold]strong[/bold]"))',
  expect=["strong"], forbid=["[bold"])

# ---- Layout helpers -------------------------------------------------------- #
G = "Layout & alignment"
F("rule.basic", G, "Rule(title) spans console width", 'from PKG.rule import Rule\nconsole.print(Rule("Chapter"))',
  expect=["Chapter", "─"], line_width=80)
F("rule.chars", G, "Rule(characters='=')", 'from PKG.rule import Rule\nconsole.print(Rule("T", characters="="))',
  expect=["T", "="], forbid=["─"])
F("align.center", G, "Align.center", 'from PKG.align import Align\nConsole(width=40).print(Align.center("mid"))',
  regex=[r"^ {17,19}mid"])
F("align.right", G, "Align.right", 'from PKG.align import Align\nConsole(width=40).print(Align.right("end"))',
  regex=[r"^ {36,37}end"])
F("align.vertical", G, "VerticalCenter", 'from PKG.align import VerticalCenter\nconsole.print(VerticalCenter("vc"))',
  expect=["vc"])
F("padding", G, "Padding(renderable, (1, 4))", 'from PKG.padding import Padding\nconsole.print(Padding("pad", (1, 4)))',
  regex=[r"^\s*\n {4}pad"])
F("columns", G, "Columns lays items side by side", 'from PKG.columns import Columns\nconsole.print(Columns(["alpha", "beta", "gamma"]))',
  regex=[r"alpha\s+beta\s+gamma"])
F("columns.many", G, "Columns wraps many items in width",
  'from PKG.columns import Columns\nconsole.print(Columns([f"item{i:02d}" for i in range(40)]))',
  expect=[f"item{i:02d}" for i in range(40)], max_width=80)
F("layout.split", G, "Layout.split_row with panels", '''
from PKG.layout import Layout
from PKG.panel import Panel
layout = Layout()
layout.split_row(Layout(name="left"), Layout(name="right"))
layout["left"].update(Panel("LEFT"))
layout["right"].update(Panel("RIGHT"))
Console(height=8).print(layout)
''', regex=[r"LEFT.*RIGHT"])
F("group", G, "Group of renderables", '''
from PKG.console import Group
from PKG.panel import Panel
console.print(Panel(Group(Panel("one"), Panel("two"))))
''', expect=["one", "two"])
F("group.decorator", G, "@group decorator", '''
from PKG.console import group
@group()
def items():
    yield "first item"
    yield "second item"
console.print(items())
''', regex=[r"first item\s*\n\s*second item"])
F("constrain", G, "Constrain(width=20)", 'from PKG.constrain import Constrain\nfrom PKG.panel import Panel\nconsole.print(Constrain(Panel("x"), width=20))',
  line_width=20)
F("containers.renderables", G, "Renderables container", 'from PKG.containers import Renderables\nconsole.print(Renderables(["r-one", "r-two"]))',
  regex=[r"r-one\s*\n\s*r-two"])

# ---- Widgets ---------------------------------------------------------------- #
G = "Tree / Markdown / Syntax / JSON"
F("tree.nested", G, "Tree with nested children", '''
from PKG.tree import Tree
root = Tree("root")
child = root.add("child")
child.add("grandchild")
root.add("second")
console.print(root)
''', expect=["root", "child", "grandchild", "second", "├──", "└──"])
F("tree.renderable", G, "Tree node with Panel label", '''
from PKG.tree import Tree
from PKG.panel import Panel
root = Tree("root")
root.add(Panel("boxed node"))
console.print(root)
''', expect=["root", "boxed node"])
MD = "# Heading\\n\\nSome **bold** and *italic* text.\\n\\n- item one\\n- item two\\n\\n1. first\\n2. second\\n\\n> quoted\\n\\n`inline`\\n\\n```python\\nprint('hi')\\n```\\n\\n[a link](https://example.com)\\n"
F("markdown.basic", G, "Markdown: headings, emphasis, lists, quote, code",
  f'from PKG.markdown import Markdown\nconsole.print(Markdown("{MD}"))',
  expect=["Heading", "bold", "italic", "item one", "first", "second", "quoted", "inline", "print('hi')", "a link"],
  forbid=["**", "```"])
F("markdown.table", G, "Markdown tables",
  'from PKG.markdown import Markdown\nconsole.print(Markdown("| a | b |\\n|---|---|\\n| 1 | 2 |\\n"))',
  expect=["a", "1", "2"], forbid=["|---|"])
CODE = "def add(a, b):\\n    return a + b\\n"
F("syntax.basic", G, "Syntax(code, 'python')", f'from PKG.syntax import Syntax\nconsole.print(Syntax("{CODE}", "python"))',
  expect=["def add(a, b):", "return a + b"])
F("syntax.line_numbers", G, "Syntax(line_numbers=True)",
  f'from PKG.syntax import Syntax\nconsole.print(Syntax("{CODE}", "python", line_numbers=True))',
  regex=[r"^\s*1\s.*def add", r"^\s*2\s.*return"])
F("syntax.color", G, "Syntax highlighting emits colors (force_terminal)",
  f'from PKG.syntax import Syntax\nConsole(force_terminal=True).print(Syntax("{CODE}", "python", theme="monokai"))',
  expect=["def"], raw_expect=[r"\x1b\[[0-9;]*m"])
F("syntax.from_path", G, "Syntax.from_path", '''
import tempfile
from PKG.syntax import Syntax
p = os.path.join(tempfile.mkdtemp(), "demo.py")
open(p, "w").write("x = 42\\n")
console.print(Syntax.from_path(p))
''', expect=["x = 42"])
F("json.renderable", G, "JSON(str)", 'from PKG.json import JSON\nconsole.print(JSON(\'{"name": "x", "list": [1, 2]}\'))',
  expect=['"name": "x"', '"list": ['])
F("json.from_data", G, "JSON.from_data", 'from PKG.json import JSON\nconsole.print(JSON.from_data({"k": True}))',
  expect=['"k": true'])
F("pretty.pprint", G, "pretty.pprint", '''
from PKG.pretty import pprint
pprint({"numbers": [1, 2, 3], "nested": {"a": None}})
''', expect=["'numbers'", "None"])
F("pretty.Pretty", G, "Pretty(obj) wraps to width", '''
from PKG.pretty import Pretty
Console(width=40).print(Pretty(list(range(40))))
''', expect=["39"], max_width=40)
F("inspect", G, "inspect(obj, methods=True)", '''
from PKG import inspect
class Dog:
    """A good dog."""
    def bark(self):
        """Woof."""
inspect(Dog(), methods=True)
''', expect=["Dog", "bark"], allow_repr=True)
F("scope", G, "render_scope", 'from PKG.scope import render_scope\nconsole.print(render_scope({"alpha": 1}, title="scope"))',
  expect=["alpha", "1"])
F("repr.auto", G, "@repr.auto", '''
import PKG.repr
@PKG.repr.auto
class Point:
    def __init__(self, x, y=0):
        self.x = x
        self.y = y
assert repr(Point(1, 2)) == "Point(1, y=2)", repr(Point(1, 2))
print("OK")
''', expect=["OK"])
F("highlighter.repr", G, "ReprHighlighter", '''
from PKG.highlighter import ReprHighlighter
from PKG.text import Text
t = ReprHighlighter()(Text("value=123 'str' True None"))
assert len(t.spans) >= 3, t.spans
print("OK")
''', expect=["OK"])
F("highlighter.regex", G, "custom RegexHighlighter", '''
from PKG.highlighter import RegexHighlighter
class EmailHighlighter(RegexHighlighter):
    base_style = "example."
    highlights = [r"(?P<email>[\\w-]+@([\\w-]+\\.)+[\\w-]+)"]
t = EmailHighlighter()("mail me@example.com now")
assert any(str(s.style) == "example.email" for s in t.spans), t.spans
print("OK")
''', expect=["OK"])

# ---- Progress / Live ------------------------------------------------------- #
G = "Progress & Live"
F("progress.track", G, "track(iterable)", '''
from PKG.progress import track
total = sum(track(range(5), description="Working"))
assert total == 10, total
print("sum", total)
''', expect=["sum 10"])
F("progress.tasks", G, "Progress add_task / advance / finished", '''
from PKG.progress import Progress
with Progress() as p:
    t = p.add_task("Downloading", total=3)
    for _ in range(3):
        p.advance(t)
assert p.finished, "progress not finished"
print("OK")
''', expect=["OK", "Downloading"])
F("progress.columns", G, "custom progress columns", '''
from PKG.progress import Progress, SpinnerColumn, BarColumn, TextColumn, TimeElapsedColumn
with Progress(SpinnerColumn(), TextColumn("{task.description}"), BarColumn(), TimeElapsedColumn()) as p:
    t = p.add_task("Columns", total=2)
    p.update(t, advance=2)
print("OK")
''', expect=["OK", "Columns"])
F("progress.bar", G, "ProgressBar renderable", 'from PKG.progress_bar import ProgressBar\nconsole.print(ProgressBar(total=100, completed=50, width=20))',
  expect=["━"])
F("bar", G, "Bar renderable", 'from PKG.bar import Bar\nconsole.print(Bar(size=10, begin=2, end=5, width=20))', regex=[r"[█▌▐]"])
F("spinner", G, "Spinner renderable", 'from PKG.spinner import Spinner\nconsole.print(Spinner("dots", text="loading"))',
  expect=["loading"])
F("status.class", G, "Status context manager", '''
from PKG.status import Status
with Status("busy") as s:
    s.update("still busy")
print("done")
''', expect=["done"])
F("live.update", G, "Live + live.update", '''
from PKG.live import Live
from PKG.panel import Panel
with Live(Panel("start"), console=console) as live:
    live.update(Panel("final content"))
''', expect=["final content"])
F("live.table", G, "Live with a growing Table", '''
from PKG.live import Live
from PKG.table import Table
t = Table()
t.add_column("Row")
with Live(t, console=console, refresh_per_second=20):
    for i in range(3):
        t.add_row(f"row{i}")
''', expect=["row0", "row1", "row2"])

# ---- Prompt ----------------------------------------------------------------- #
G = "Prompt"
F("prompt.ask", G, "Prompt.ask", 'from PKG.prompt import Prompt\nv = Prompt.ask("Name")\nprint("GOT", v)',
  stdin="Alice\n", expect=["GOT Alice"])
F("prompt.default", G, "Prompt.ask(default=...)", 'from PKG.prompt import Prompt\nv = Prompt.ask("Name", default="Bob")\nprint("GOT", v)',
  stdin="\n", expect=["GOT Bob"])
F("prompt.choices", G, "Prompt.ask(choices=...) re-asks on invalid", '''
from PKG.prompt import Prompt
v = Prompt.ask("Pick", choices=["a", "b"])
print("GOT", v)
''', stdin="x\nb\n", expect=["GOT b"])
F("prompt.int", G, "IntPrompt.ask validates ints", 'from PKG.prompt import IntPrompt\nv = IntPrompt.ask("N")\nprint("GOT", v + 1)',
  stdin="abc\n41\n", expect=["GOT 42"])
F("prompt.confirm", G, "Confirm.ask", '''
from PKG.prompt import Confirm
a = Confirm.ask("Sure?")
b = Confirm.ask("Again?")
print("GOT", a, b)
''', stdin="y\nn\n", expect=["GOT True False"])
F("prompt.eof", G, "Prompt.ask on EOF raises (no hang)", '''
from PKG.prompt import Prompt
try:
    Prompt.ask("Name")
except EOFError:
    print("EOF handled")
''', stdin="", expect=["EOF handled"])

# ---- Traceback / logging / misc -------------------------------------------- #
G = "Traceback & logging"
F("traceback.print_exception", G, "console.print_exception()", '''
def boom():
    return 1 / 0
try:
    boom()
except Exception:
    console.print_exception()
''', expect=["ZeroDivisionError", "boom", "╭"], allow_tb=True)
F("traceback.locals", G, "print_exception(show_locals=True)", '''
def boom():
    secret_local = "s3cr3t"
    return 1 / 0
try:
    boom()
except Exception:
    console.print_exception(show_locals=True)
''', expect=["ZeroDivisionError", "secret_local", "s3cr3t"], allow_tb=True)
F("traceback.class", G, "Traceback() renderable", '''
from PKG.traceback import Traceback
try:
    {}["missing"]
except Exception:
    console.print(Traceback())
''', expect=["KeyError", "missing", "╭"], allow_tb=True)
F("traceback.install", G, "traceback.install() formats uncaught errors", '''
from PKG.traceback import install
install()
def explode():
    raise ValueError("kaboom")
explode()
''', expect=["ValueError", "kaboom", "explode", "╭"], allow_tb=True, exit_nonzero=True)
F("logging.handler", G, "logging RichHandler", '''
import logging
from PKG.logging import RichHandler
logging.basicConfig(level="INFO", format="%(message)s", handlers=[RichHandler()])
logging.getLogger("demo").info("hello from logging")
logging.getLogger("demo").warning("careful now")
''', expect=["hello from logging", "careful now", "INFO", "WARNING"])
F("filesize", G, "filesize.decimal", '''
from PKG.filesize import decimal
assert decimal(1_500_000) == "1.5 MB", decimal(1_500_000)
assert decimal(1000) == "1.0 kB", decimal(1000)
print("OK")
''', expect=["OK"])
F("segment", G, "Segment / Segment.line", '''
from PKG.segment import Segment
assert Segment("x").text == "x"
assert Segment.line().text == "\\n"
print("OK")
''', expect=["OK"])
F("measure", G, "Measurement.get", '''
from PKG.measure import Measurement
m = Measurement.get(console, console.options, "hello")
assert (m.minimum, m.maximum) == (5, 5), m
print("OK")
''', expect=["OK"])
F("control", G, "Control.bell", '''
from PKG.control import Control
assert "\\x07" in str(Control.bell())
print("OK")
''', expect=["OK"])
F("region", G, "Region", '''
from PKG.region import Region
r = Region(0, 0, 10, 5)
assert (r.width, r.height) == (10, 5)
print("OK")
''', expect=["OK"])

# ---- Native extension (fast_rich._core) ------------------------------------ #
G = "Native core (fast_rich._core)"
N = "import os\nfrom fast_rich import _core as C\nc = C.Console()\n"
F("core.print", G, "Console.print with markup", N + 'c.print("[bold]native[/bold] ok")', expect=["native ok"],
  forbid=["[bold"], native=True)
F("core.multiline", G, "Console.print keeps newlines", N + 'c.print("l1\\nl2")', regex=[r"l1[ \t]*\n[ \t]*l2"], native=True)
F("core.no_ansi_piped", G, "no ANSI codes when piped", N + 'c.print("[red]x[/]")', raw_forbid=[r"\x1b\["], native=True)
F("core.table", G, "print_table", N + 't = C.Table()\nt.add_column("H1")\nt.add_column("H2")\nt.add_row(["v1", "v2"])\nc.print_table(t)',
  expect=["H1", "H2", "v1", "v2"], equal_width=True, native=True)
F("core.panel", G, "print_panel", N + 'c.print_panel(C.Panel(C.Text("inside"), title="T"))', expect=["inside", "T"], native=True)
F("core.rule", G, "print_rule", N + 'c.print_rule(C.Rule("R"))\nprint("after")', expect=["R"], regex=[r"─\s*\nafter"], native=True)
F("core.tree", G, "print_tree with nested leaf", N + 't = C.Tree("root")\nch = t.add("child")\nch.add_leaf("leaf")\nc.print_tree(t)',
  expect=["root", "child", "leaf"], native=True)
F("core.markdown", G, "print_markdown", N + 'c.print_markdown(C.Markdown("# Title\\n\\n**bold** text"))',
  expect=["Title", "bold text"], forbid=["**"], native=True)
F("core.syntax", G, "print_syntax", N + f'c.print_syntax(C.Syntax("{CODE}", lexer="python", line_numbers=True))',
  expect=["def add", "return a + b"], native=True)
F("core.json", G, "print_json", N + 'c.print_json(\'{"a": [1, 2]}\')', expect=['"a"', "1"], native=True)
F("core.columns", G, "print_columns", N + 'c.print_columns(C.Columns([C.Text("c-one"), C.Text("c-two")]))',
  regex=[r"c-one\s+c-two"], native=True)
F("core.traceback", G, "print_traceback", N + 'c.print_traceback(C.Traceback("boom happened"))', expect=["boom happened"],
  native=True, allow_tb=True)
F("core.log", G, "log / debug / warn / error", N + 'c.log("m-log")\nc.debug("m-debug")\nc.warn("m-warn")\nc.error("m-error")',
  expect=["m-log", "m-debug", "m-warn", "m-error"], native=True)
F("core.export", G, "export_text / html / svg", N + '''
t = C.Text.from_markup("[bold]exp[/bold]")
assert c.export_text(t).strip() == "exp", repr(c.export_text(t))
h = c.export_renderable_html(t); assert "exp" in h and "<" in h, h
s = c.export_renderable_svg(t); assert "<svg" in s, s[:100]
print("OK")
''', expect=["OK"], native=True)
F("core.save_html", G, "record + save_html", N + '''
import tempfile
c.record = True
c.print("saved line")
p = os.path.join(tempfile.mkdtemp(), "o.html")
c.save_html(p)
assert "saved line" in open(p, encoding="utf-8").read()
print("OK")
''', expect=["OK"], native=True)
F("core.style_text", G, "Style.parse + Text.append", N + '''
s = C.Style.parse("bold red")
t = C.Text("a", style=s)
t.append("b", style=C.Style(italic=True))
c.print_text(t)
''', expect=["ab"], native=True)
F("core.progress", G, "Progress add_task/advance/print", N + 'p = C.Progress()\ni = p.add_task("Native task", total=4)\np.advance(i, 2)\np.print()',
  expect=["Native task"], native=True)
F("core.layout_live_align", G, "Layout / Live / Align construct", N + '''
l = C.Layout(); l.split_row([C.Layout(), C.Layout()])
live = C.Live(); live.start(); live.stop()
C.Align("x", "center")
print("OK")
''', expect=["OK"], native=True)
F("core.prompt", G, "Prompt / Confirm / IntPrompt", N + '''
a = C.Prompt.ask("Name")
b = C.Confirm.ask("Ok?")
n = C.IntPrompt.ask("N")
print("GOT", a, b, n)
''', stdin="Zed\ny\n7\n", expect=["GOT Zed True 7"], native=True)
F("core.prompt_eof", G, "Prompt.ask on EOF (no hang)", N + '''
try:
    v = C.Prompt.ask("Name")
    print("returned", repr(v))
except EOFError:
    print("EOF handled")
''', stdin="", regex=[r"EOF handled|returned"], native=True)
F("core.filesize", G, "filesize decimal / binary", N + '''
assert C.decimal(1_500_000) == "1.5 MB", C.decimal(1_500_000)
assert C.binary(1024) == "1.0 KiB", C.binary(1024)
print("OK")
''', expect=["OK"], native=True)
F("core.inspect", G, "inspect", N + 'C.inspect([1, 2, 3], methods=True)', expect=["list"], native=True)

# Multi-line renderables must actually render on several lines.
MIN_LINES = {
    "table.basic": 6, "table.title": 7, "table.no_header": 4, "table.box_ascii": 6, "table.box_styles": 20,
    "table.justify": 6, "table.renderable_cells": 5, "table.cjk": 7, "table.add_column_kwargs": 5,
    "panel.basic": 3, "panel.title": 3, "panel.box": 3, "panel.multiline": 5, "panel.cjk": 4,
    "panel.nested_table": 7, "panel.markup": 3, "tree.nested": 4, "tree.renderable": 4,
    "markdown.basic": 10, "markdown.table": 3, "syntax.basic": 2, "syntax.line_numbers": 2,
    "json.renderable": 4, "json.from_data": 3, "scope": 3, "inspect": 3,
    "core.table": 5, "core.panel": 3, "core.tree": 3, "core.markdown": 2, "core.syntax": 2,
    "console.print_json": 4, "console.print_json_data": 4, "traceback.class": 4, "traceback.print_exception": 4,
}
for _f in FEATURES:
    if _f["id"] in MIN_LINES:
        _f["min_lines"] = MIN_LINES[_f["id"]]


# --------------------------------------------------------------------------- #
# Runner
# --------------------------------------------------------------------------- #
ANSI_RE = re.compile(r"\x1b\][^\x07\x1b]*(?:\x07|\x1b\\)|\x1b\[[0-?]*[ -/]*[@-~]|\x1b[()][0-9A-Za-z]|\x1b[=>78]")
SHOW = False
DEFAULT_FORBID = ["object at 0x", "Rust Error"]
TB = "Traceback (most recent call last)"


def strip_ansi(s: str) -> str:
    return ANSI_RE.sub("", s).replace("\r\n", "\n").replace("\r", "\n")


def disp_width(line: str) -> int:
    w = 0
    for ch in line:
        if unicodedata.combining(ch) or ch in "‍️︎":
            continue
        w += 2 if unicodedata.east_asian_width(ch) in ("W", "F") else 1
    return w


TTY = False


def run_tty(code: str, stdin: str, timeout: float, env: dict, cwd: str) -> dict:
    """Run under an 80x25 pseudo-terminal (POSIX only). stdout+stderr are merged."""
    import fcntl, pty, select, struct, termios, time
    master, slave = pty.openpty()
    fcntl.ioctl(slave, termios.TIOCSWINSZ, struct.pack("HHHH", 25, 80, 0, 0))
    attrs = termios.tcgetattr(slave)
    attrs[3] &= ~(termios.ECHO | getattr(termios, "ECHOCTL", 0))  # don't echo our stdin back
    termios.tcsetattr(slave, termios.TCSANOW, attrs)
    proc = subprocess.Popen([sys.executable, "-X", "utf8", "-c", code], stdin=slave, stdout=slave, stderr=slave,
                            cwd=cwd, env=env, close_fds=True, start_new_session=True)
    os.close(slave)
    os.write(master, stdin.encode() + b"\x04")  # trailing Ctrl-D = EOF
    out, deadline = b"", time.monotonic() + timeout
    while True:
        if time.monotonic() > deadline:
            proc.kill(); proc.wait(); os.close(master)
            return dict(rc=None, out=out.decode("utf-8", "replace"), err="")
        r, _, _ = select.select([master], [], [], 0.1)
        if r:
            try:
                chunk = os.read(master, 65536)
            except OSError:
                chunk = b""
            if chunk:
                out += chunk
                continue
            if proc.poll() is not None:
                break
        elif proc.poll() is not None:
            break
    rc = proc.wait()
    os.close(master)
    return dict(rc=rc, out=out.decode("utf-8", "replace"), err="")


def run(code: str, stdin: str, timeout: float) -> dict:
    env = dict(os.environ, COLUMNS="80", LINES="25", PYTHONUTF8="1", PYTHONIOENCODING="utf-8", PAGER="cat", MANPAGER="cat")
    for var in ("FORCE_COLOR", "NO_COLOR", "TTY_COMPATIBLE", "TTY_INTERACTIVE"):
        env.pop(var, None)
    with tempfile.TemporaryDirectory(ignore_cleanup_errors=True) as cwd:
        if TTY:
            return run_tty(code, stdin, timeout, env, cwd)
        try:
            p = subprocess.run([sys.executable, "-X", "utf8", "-c", code], input=stdin.encode(), capture_output=True,
                               timeout=timeout, cwd=cwd, env=env)
        except subprocess.TimeoutExpired as e:
            return dict(rc=None, out=(e.stdout or b"").decode("utf-8", "replace"), err=(e.stderr or b"").decode("utf-8", "replace"))
    return dict(rc=p.returncode, out=p.stdout.decode("utf-8", "replace"), err=p.stderr.decode("utf-8", "replace"))


def last_error_line(err: str) -> str:
    lines = [l.strip() for l in strip_ansi(err).splitlines() if l.strip()]
    for l in reversed(lines):
        if re.match(r"^[A-Za-z_][\w.]*(Error|Exception|Exit|Interrupt)\b", l) or ": " in l and "Error" in l.split(":")[0]:
            return l[:200]
    return (lines[-1] if lines else "no output")[:200]


def evaluate(spec: dict, r: dict, timeout: float) -> tuple[str, str]:
    if r["rc"] is None:
        return "HANG", f"no exit after {timeout:.0f}s"
    raw = r["out"] + r["err"]
    plain = strip_ansi(raw)
    if not spec.get("exit_nonzero") and r["rc"] != 0:
        err = last_error_line(r["err"] or r["out"])
        if err.startswith(("ModuleNotFoundError", "ImportError", "AttributeError")) or (
            err.startswith("TypeError") and ("unexpected keyword" in err or "positional argument" in err)
        ):
            return "MISSING", err
        if err.startswith("AssertionError"):
            return "WRONG", err
        return "ERROR", err
    if spec.get("exit_nonzero") and r["rc"] == 0:
        return "WRONG", "expected the uncaught exception to exit non-zero"
    fails = []
    for e in spec.get("expect", []):
        if e not in plain:
            fails.append(f"missing {e!r}")
    forbid = list(spec.get("forbid", [])) + ([] if spec.get("allow_repr") else DEFAULT_FORBID) + ([] if spec.get("allow_tb") else [TB])
    for f in forbid:
        if f in plain:
            fails.append(f"unexpected {f!r}")
    for rx in spec.get("regex", []):
        if not re.search(rx, plain, re.M):
            fails.append(f"no match /{rx}/")
    for rx in spec.get("raw_expect", []):
        if not re.search(rx, raw):
            fails.append(f"raw output lacks /{rx.encode('unicode_escape').decode()}/")
    for rx in spec.get("raw_forbid", []):
        if re.search(rx, raw):
            fails.append("ANSI escape codes in piped output")
    for e in spec.get("stderr_expect", []):
        if e not in strip_ansi(r["err"]):
            fails.append(f"{e!r} not on stderr")
    for e in spec.get("stdout_forbid", []):
        if e in strip_ansi(r["out"]):
            fails.append(f"{e!r} on stdout")
    lines = [l.rstrip() for l in plain.split("\n")]
    nonempty = [l for l in lines if l.strip()]
    if "max_width" in spec:
        widest = max((disp_width(l) for l in lines), default=0)
        if widest > spec["max_width"]:
            fails.append(f"line width {widest} > {spec['max_width']}")
    if "line_width" in spec:
        widths = sorted({disp_width(l) for l in nonempty})
        if widths != [spec["line_width"]]:
            fails.append(f"line widths {widths}, expected {spec['line_width']}")
    if "min_lines" in spec and len(nonempty) < spec["min_lines"]:
        fails.append(f"only {len(nonempty)} line(s), expected >= {spec['min_lines']} (newlines collapsed?)")
    if spec.get("equal_width"):
        widths = sorted({disp_width(l) for l in nonempty})
        if len(widths) > 1:
            fails.append(f"misaligned: line widths {widths}")
    if not nonempty and not fails and not spec.get("forbid"):
        fails.append("no output")
    return ("WRONG", "; ".join(fails)) if fails else ("PASS", "")


PIPE_ONLY_KEYS = ("stderr_expect", "stdout_forbid", "raw_forbid")


def run_feature(spec: dict, timeout: float, have_rich: bool) -> dict:
    if TTY and any(k in spec for k in PIPE_ONLY_KEYS):
        return dict(id=spec["id"], group=spec["group"], name=spec["name"], status="N/A",
                    detail="pipe-only check", rich=None, same_as_rich=None)
    code = spec["code"]
    if not spec.get("native"):
        code = PRELUDE + code
    fast_code = code.replace("PKG", "fast_rich") + "\nimport sys as _s; assert 'rich' not in _s.modules, 'real rich imported'\n"
    fr = run(fast_code, spec.get("stdin", ""), timeout)
    status, detail = evaluate(spec, fr, timeout)
    if SHOW:
        print(f"\n######## {spec['id']} rc={fr['rc']}\n{strip_ansi(fr['out'])}\n--- stderr ---\n{strip_ansi(fr['err'])[-1500:]}", flush=True)
    out = dict(id=spec["id"], group=spec["group"], name=spec["name"], status=status, detail=detail,
               rich=None, same_as_rich=None)
    if have_rich and not spec.get("native"):
        rr = run(code.replace("PKG", "rich"), spec.get("stdin", ""), timeout)
        rstatus, rdetail = evaluate(spec, rr, timeout)
        out["rich"] = rstatus if rstatus == "PASS" else f"{rstatus}: {rdetail}"
        if rstatus != "PASS":
            out["status"], out["detail"] = "INVALID", f"reference Rich fails this test ({rstatus}: {rdetail})"
        elif fr["rc"] is not None:
            norm = lambda r: "\n".join(l.rstrip() for l in strip_ansi(r["out"] + r["err"]).strip("\n").split("\n"))
            out["same_as_rich"] = norm(fr) == norm(rr)
    return out


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--json")
    ap.add_argument("--only")
    ap.add_argument("--show", action="store_true", help="print raw outputs")
    ap.add_argument("--tty", action="store_true", help="run snippets in an 80x25 pseudo-terminal (POSIX)")
    ap.add_argument("--timeout", type=float, default=30)
    ap.add_argument("--jobs", type=int, default=min(8, (os.cpu_count() or 2)))
    a = ap.parse_args()
    global SHOW, TTY
    SHOW, TTY = a.show, a.tty

    try:
        import rich  # noqa: F401
        have_rich = True
    except ImportError:
        have_rich = False
    from importlib import metadata

    env = dict(
        python=platform.python_version(), system=platform.system(), machine=platform.machine(),
        libc="-".join(platform.libc_ver()) or ("musl" if os.path.exists("/etc/alpine-release") else ""),
        fast_rich=metadata.version("fast-rich"), rich=metadata.version("rich") if have_rich else None,
        image=os.environ.get("TEST_IMAGE", ""), mode="tty" if a.tty else "pipe",
    )
    feats = [f for f in FEATURES if not a.only or a.only in f["id"]]
    with cf.ThreadPoolExecutor(a.jobs) as ex:
        results = list(ex.map(lambda s: run_feature(s, a.timeout, have_rich), feats))

    counts: dict[str, int] = {}
    group = None
    print(f"== fast-rich {env['fast_rich']} feature test [{env['mode']}] | Python {env['python']} | {env['system']} {env['machine']} {env['libc']} | rich ref: {env['rich']}")
    for r in results:
        counts[r["status"]] = counts.get(r["status"], 0) + 1
        if r["group"] != group:
            group = r["group"]
            print(f"\n-- {group}")
        same = {True: " (=rich)", False: "", None: ""}[r["same_as_rich"]]
        print(f"  {r['status']:<8} {r['id']:<30} {r['detail'][:110]}{same}")
    print("\n== " + "  ".join(f"{k}={v}" for k, v in sorted(counts.items())))
    if a.json:
        with open(a.json, "w", encoding="utf-8") as fh:
            json.dump(dict(env=env, results=results), fh, indent=1, ensure_ascii=False)
    return 0


if __name__ == "__main__":
    sys.exit(main())
