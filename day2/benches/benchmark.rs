use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day2 Part 1", |b| {
        b.iter(|| {
            day2::part1(day2::parse(&common::take_input(black_box::<_>(
                "input.txt",
            ))))
        })
    });

    c.bench_function("day2 Part 2", |b| {
        b.iter(|| {
            day2::part2(day2::parse(&common::take_input(black_box(
                "input.txt",
            ))))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
