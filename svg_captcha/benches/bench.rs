use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use svg_captcha::{render, render_svg};

fn bench_render_svg(c: &mut Criterion) {
    c.bench_function("render_svg", |b| {
        b.iter(|| render_svg(black_box(300), black_box(150), black_box(3)))
    });
}

fn bench_render_full(c: &mut Criterion) {
    c.bench_function("render_full", |b| {
        b.iter(|| render(black_box(300), black_box(150), black_box(3)))
    });
}

criterion_group!(benches, bench_render_svg, bench_render_full);
criterion_main!(benches);
