use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day3 Part 1", |b| {
        b.iter(|| {
            day3::part1(day3::parse(&common::take_input(black_box(
                "input.txt",
            ))))
        })
    });

    c.bench_function("day3 Part 2", |b| {
        b.iter(|| {
            day3::part2(day3::parse(&common::take_input(black_box(
                "input.txt",
            ))))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
