use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day1 Part 1", |b| {
        b.iter(|| {
            day1::part1(day1::parse(&common::take_input(black_box(
                "input.txt",
            ))))
        })
    });

    c.bench_function("day1 Part 2", |b| {
        b.iter(|| {
            day1::part2(day1::parse(&common::take_input(black_box(
                "input.txt",
            ))))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
