# fast_rich

**fast_rich** is a drop-in replacement for [Python Rich](https://github.com/Textualize/rich)
whose rendering engine is written in Rust. Change `rich` to `fast_rich` in your
imports; everything else stays the same.

```bash
pip install fast-rich
```

Wheels are published for Linux (glibc and musl, x86_64 and aarch64), Windows
x64 and macOS (Intel and Apple Silicon), for Python 3.10+. No Rust toolchain is
needed.

## Quick start

```python
from fast_rich.console import Console
from fast_rich.table import Table

console = Console()

table = Table(title="Star Wars Movies")
table.add_column("Title", style="cyan")
table.add_column("Year", style="magenta", justify="right")
table.add_row("A New Hope", "1977")
table.add_row("The Empire Strikes Back", "1980")

console.print(table)
```

## How it works

- The **object model is Rich's**: `Console`, `Text`, `Style`, `Segment`, markup,
  highlighters, `pretty`, `Progress`, `Live`, `logging`, `prompt` and friends keep
  Rich's exact API and behaviour.
- **Layout and text wrapping run in Rust**: `Text`, `Panel`, `Table`, `Tree`,
  `Rule`, `Align`, `Padding`, `Constrain`, `Columns`, `Styled`, `Markdown` and
  `Syntax` are laid out by the `fast-rich` Rust crate, which ports Rich's
  algorithms (column widths, wrapping, box drawing, tree guides).
- Your own renderables (`__rich__`, `__rich_console__`, `__rich_measure__`)
  work unchanged, including inside native panels and tables.

`Syntax` is highlighted with [syntect](https://github.com/trishume/syntect) and
`Markdown` is parsed with pulldown-cmark, so colours and Markdown details can
differ from Rich (which uses Pygments and markdown-it).

## Compatibility

The package is checked feature by feature against Rich 15 (152 checks,
`bindings/python/docker/feature_test.py`): all pass on Linux (8 Docker
images, piped and in a terminal), and 118 outputs are byte-identical to Rich.

## Performance

Release build, Apple M-series, printing to a 100-column truecolor console
(best of 5, `bindings/python/benchmarks/bench_vs_rich.py`):

| Operation | Rich 15 | fast_rich | Speedup |
| :--- | ---: | ---: | ---: |
| Table, 1000 rows × 5 columns | 153 ms | 44 ms | 3.5× |
| Panel with 100 KB of text | 15 ms | 5.5 ms | 2.8× |
| Syntax, `fast_rich/table.py` (~600 lines) | 53 ms | 19 ms | 2.8× |
| 1000 markup lines | 40 ms | 29 ms | 1.4× |

## License

MIT. Parts of this package (the Python object model) are adapted from Rich,
Copyright (c) 2020 Will McGugan, MIT License — see `fast_rich/LICENSE.rich`.

## Credits

- [Rich](https://github.com/Textualize/rich) by Will McGugan
- [PyO3](https://pyo3.rs/) and [maturin](https://www.maturin.rs/)
