use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rich_rust::console::RenderContext;
use rich_rust::markup;
use rich_rust::prelude::*;
use rich_rust::table::Table;

fn bench_markup_parsing(c: &mut Criterion) {
    let markup = "[bold red]Hello[/] [blue]World[/]! ".repeat(20);
    c.bench_function("markup parsing", |b: &mut criterion::Bencher| {
        b.iter(|| markup::parse(black_box(&markup)))
    });
}

fn bench_text_rendering(c: &mut Criterion) {
    let parsing_markup = "[bold red]Hello[/] [blue]World[/]! ".repeat(20);
    let text = markup::parse(&parsing_markup);
    // Simulate rendering context (e.g. 80 columns)
    // We can't easily mock full Console render loop without IO, but we can call .render() which returns Segments
    let context = RenderContext { width: 80 };

    c.bench_function("text render", |b: &mut criterion::Bencher| {
        b.iter(|| text.render(black_box(&context)))
    });
}

fn bench_table_rendering(c: &mut Criterion) {
    let mut table = Table::new();
    table.add_column("Col 1");
    table.add_column("Col 2");
    for i in 0..100 {
        table.add_row(vec![format!("Row {} Col 1", i), format!("Row {} Col 2", i)]);
    }
    let context = RenderContext { width: 80 };

    c.bench_function("table render (100 rows)", |b: &mut criterion::Bencher| {
        b.iter(|| table.render(black_box(&context)))
    });
}

// Tree, Markdown, Syntax benchmarks if features enabled?
// For simplicity in this initial file, we'll stick to core or assume features are on for dev/bench profile.

criterion_group!(
    benches,
    bench_markup_parsing,
    bench_text_rendering,
    bench_table_rendering
);
criterion_main!(benches);
