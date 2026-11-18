//! Benchmarks for `ReprHighlighter` (Rich's default print highlighting).

use criterion::{criterion_group, criterion_main, Criterion};
use fast_rich::highlighter::{Highlighter, ReprHighlighter};
use std::hint::black_box;

const TYPICAL: &str = "Server 192.168.1.10 returned 404 in 3.5s, retry=True, \
url=https://x.io/v1, file=/var/log/app.log, data=[1, 2, 3] 'ok'";

fn bench_highlight(c: &mut Criterion) {
    let hl = ReprHighlighter::new();
    c.bench_function("highlight_typical_line", |b| {
        b.iter(|| hl.highlight(black_box(TYPICAL)))
    });
    c.bench_function("highlight_plain_prose", |b| {
        b.iter(|| hl.highlight(black_box("the quick brown fox jumps over the lazy dog")))
    });
    let digits = "1".repeat(20_000);
    c.bench_function("highlight_20k_digits", |b| {
        b.iter(|| hl.highlight(black_box(&digits)))
    });
    let word = "a".repeat(20_000);
    c.bench_function("highlight_20k_word", |b| {
        b.iter(|| hl.highlight(black_box(&word)))
    });
}

criterion_group!(benches, bench_highlight);
criterion_main!(benches);
