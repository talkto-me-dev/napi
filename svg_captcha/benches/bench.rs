use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use svg_captcha::render_svg;

fn bench_render_svg(c: &mut Criterion) {
    c.bench_function("render_svg", |b| {
        b.iter(|| render_svg(black_box(300), black_box(150), black_box(3)))
    });
}

criterion_group!(benches, bench_render_svg);
criterion_main!(benches);
