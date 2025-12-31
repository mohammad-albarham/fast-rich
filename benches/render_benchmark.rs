use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rich_rust::console::RenderContext;
use rich_rust::markup;
use rich_rust::panel::Panel;
use rich_rust::prelude::*;
use rich_rust::table::Table;
use rich_rust::tree::Tree;

fn bench_markup_parsing(c: &mut Criterion) {
    let markup = "[bold red]Hello[/] [blue]World[/]! ".repeat(50);
    c.bench_function("markup parsing (1kb)", |b| {
        b.iter(|| markup::parse(black_box(&markup)))
    });
}

fn bench_text_rendering(c: &mut Criterion) {
    let parsing_markup = "[bold red]Hello[/] [blue]World[/]! ".repeat(50);
    let text = markup::parse(&parsing_markup);
    let context = RenderContext { width: 80 };

    c.bench_function("text render (wrap)", |b| {
        b.iter(|| text.render(black_box(&context)))
    });
}

fn bench_table_rendering_100(c: &mut Criterion) {
    let mut table = Table::new();
    table.add_column("Col 1");
    table.add_column("Col 2");
    table.add_column("Col 3");
    for i in 0..100 {
        table.add_row(vec![
            format!("Row {} Col 1 Data", i),
            format!("Row {} Col 2 Data", i),
            format!("Row {} Col 3 Data", i),
        ]);
    }
    let context = RenderContext { width: 100 };

    c.bench_function("table render (100 rows)", |b| {
        b.iter(|| table.render(black_box(&context)))
    });
}

fn bench_panel_rendering(c: &mut Criterion) {
    let content = "Panel content ".repeat(20);
    let panel = Panel::new(content).title("Benchmark Panel");
    let context = RenderContext { width: 80 };

    c.bench_function("panel render", |b| {
        b.iter(|| panel.render(black_box(&context)))
    });
}

fn bench_tree_rendering(c: &mut Criterion) {
    // Create a moderately deep tree
    let mut tree = Tree::new(rich_rust::text::Text::from("Root"));
    for i in 0..10 {
        let child = tree.add(rich_rust::text::Text::from(format!("Child {}", i)));
        for j in 0..5 {
            child.add(rich_rust::text::Text::from(format!("Grandchild {}", j)));
        }
    }
    let context = RenderContext { width: 80 };

    c.bench_function("tree render (50 nodes)", |b| {
        b.iter(|| tree.render(black_box(&context)))
    });
}

#[cfg(feature = "markdown")]
fn bench_markdown_rendering(c: &mut Criterion) {
    use rich_rust::markdown::Markdown;
    let md_content = r#"
# Heading
## Subheading
* List Item 1
* List Item 2
    * Nested

```python
def foo():
    return True
```
    "#
    .repeat(10);

    let md = Markdown::new(&md_content);
    let context = RenderContext { width: 80 };

    c.bench_function("markdown render", |b| {
        b.iter(|| md.render(black_box(&context)))
    });
}

#[cfg(feature = "syntax")]
fn bench_syntax_rendering(c: &mut Criterion) {
    use rich_rust::syntax::Syntax;
    let code = r#"
def factorial(n):
    if n == 0:
        return 1
    return n * factorial(n - 1)
"#
    .repeat(20);

    let syntax = Syntax::new(&code, "python");
    let context = RenderContext { width: 80 };

    c.bench_function("syntax highlight (python)", |b| {
        b.iter(|| syntax.render(black_box(&context)))
    });
}

fn bench_progress_operation(c: &mut Criterion) {
    use rich_rust::progress::Progress;

    c.bench_function("progress add_task + update", |b| {
        b.iter(|| {
            let progress = Progress::new();
            let task_id = progress.add_task("Downloading", Some(100));
            for i in 0..100 {
                progress.update(task_id, i + 1);
            }
        })
    });
}

fn bench_traceback_render(c: &mut Criterion) {
    use rich_rust::traceback::Traceback;

    let tb = Traceback::from_error("File not found: /path/to/missing/file.txt");
    let context = RenderContext { width: 80 };

    c.bench_function("traceback render", |b| {
        b.iter(|| tb.render(black_box(&context)))
    });
}

// Group definitions
#[cfg(all(feature = "markdown", feature = "syntax"))]
criterion_group!(
    benches,
    bench_markup_parsing,
    bench_text_rendering,
    bench_table_rendering_100,
    bench_panel_rendering,
    bench_tree_rendering,
    bench_markdown_rendering,
    bench_syntax_rendering,
    bench_progress_operation,
    bench_traceback_render
);

#[cfg(all(feature = "markdown", not(feature = "syntax")))]
criterion_group!(
    benches,
    bench_markup_parsing,
    bench_text_rendering,
    bench_table_rendering_100,
    bench_panel_rendering,
    bench_tree_rendering,
    bench_markdown_rendering,
    bench_progress_operation,
    bench_traceback_render
);

#[cfg(all(not(feature = "markdown"), feature = "syntax"))]
criterion_group!(
    benches,
    bench_markup_parsing,
    bench_text_rendering,
    bench_table_rendering_100,
    bench_panel_rendering,
    bench_tree_rendering,
    bench_syntax_rendering,
    bench_progress_operation,
    bench_traceback_render
);

#[cfg(all(not(feature = "markdown"), not(feature = "syntax")))]
criterion_group!(
    benches,
    bench_markup_parsing,
    bench_text_rendering,
    bench_table_rendering_100,
    bench_panel_rendering,
    bench_tree_rendering,
    bench_progress_operation,
    bench_traceback_render
);

criterion_main!(benches);
