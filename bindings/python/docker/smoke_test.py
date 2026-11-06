"""Smoke test for an installed fast-rich wheel.

Verifies that:
  1. No Rust toolchain is available (so the install could not have compiled anything).
  2. fast-rich was installed from a binary wheel, not built from the sdist.
  3. The native extension `fast_rich._core` loads and its renderers work.
  4. The public Python API works on top of it.

Exit code 0 means every check passed.
"""

from __future__ import annotations

import io
import platform
import shutil
import sys
from contextlib import redirect_stdout
from importlib import metadata

FAILURES: list[str] = []


def check(name: str, fn) -> None:
    try:
        fn()
    except Exception as exc:  # noqa: BLE001 - report every failure, keep going
        FAILURES.append(f"{name}: {type(exc).__name__}: {exc}")
        print(f"  FAIL  {name}: {type(exc).__name__}: {exc}")
    else:
        print(f"  ok    {name}")


def no_rust_toolchain() -> None:
    found = [tool for tool in ("cargo", "rustc", "rustup") if shutil.which(tool)]
    assert not found, f"Rust toolchain present on PATH: {found}"


def installed_from_wheel() -> None:
    dist = metadata.distribution("fast-rich")
    wheel_meta = dist.read_text("WHEEL") or ""
    tags = [line.split(":", 1)[1].strip() for line in wheel_meta.splitlines() if line.startswith("Tag:")]
    assert tags, "no WHEEL metadata found"
    # The published wheels are all cp310-abi3. Building from source is ruled out
    # separately by the no-Rust-toolchain check.
    assert any(t.startswith("cp310-abi3-") for t in tags), f"unexpected wheel tags: {tags}"
    generator = next((l for l in wheel_meta.splitlines() if l.startswith("Generator:")), "")
    print(f"        version={dist.version} tags={tags} {generator}")


def native_core() -> None:
    from fast_rich import _core

    print(f"        {_core.__file__}")
    c = _core.Console(force_terminal=False)

    t = _core.Table()
    t.add_column("Platform")
    t.add_column("Python")
    t.add_row([platform.machine(), platform.python_version()])
    c.print_table(t)

    c.print_panel(_core.Panel(_core.Text.from_markup("[bold]native panel[/]"), title="fast-rich"))
    c.print_rule(_core.Rule("rule"))

    tree = _core.Tree("root")
    tree.add("child").add_leaf("leaf")
    c.print_tree(tree)

    c.print_markdown(_core.Markdown("# Title\n\n- item **bold**\n\n```python\nx = 1\n```"))
    # Syntax exercises syntect + its regex engine (C code compiled into the wheel).
    c.print_syntax(_core.Syntax("def f(x):\n    return x * 2\n", lexer="python", line_numbers=True))
    c.print_json('{"ok": true, "n": [1, 2, 3]}')

    assert _core.decimal(1_500_000) == "1.5 MB", _core.decimal(1_500_000)


def python_api() -> None:
    import fast_rich
    from fast_rich import _native
    from fast_rich.console import Console
    from fast_rich.panel import Panel
    from fast_rich.table import Table

    table = Table("a", "b")
    table.add_row("1", "2")
    assert _native.is_native(table), "Table is not rendered by the native core"
    console = Console(file=io.StringIO(), width=30, color_system=None)
    console.print(Panel(table, title="t"))
    lines = console.file.getvalue().splitlines()
    assert lines[0].startswith("╭") and " t " in lines[0], lines
    assert "┃ a ┃ b ┃" in lines[2], lines
    assert all(len(line) == 30 for line in lines), [len(l) for l in lines]
    buf = io.StringIO()
    with redirect_stdout(buf):
        fast_rich.print("[green]hello[/] world")
    assert buf.getvalue() == "hello world\n", repr(buf.getvalue())


def main() -> int:
    print(f"== fast-rich smoke test | Python {platform.python_version()} | "
          f"{platform.system()} {platform.machine()} | libc={'-'.join(platform.libc_ver()) or 'n/a'}")
    check("no Rust toolchain on PATH", no_rust_toolchain)
    check("installed from published abi3 wheel", installed_from_wheel)
    check("native extension renders", native_core)
    check("public Python API", python_api)
    if FAILURES:
        print(f"== FAILED ({len(FAILURES)})")
        return 1
    print("== PASSED")
    return 0


if __name__ == "__main__":
    sys.exit(main())
