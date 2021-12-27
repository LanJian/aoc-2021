use aoc_2021::{
    utils, day_13,
};
use criterion::{black_box, criterion_group, Criterion};

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day 13: transparent origami");

    group.bench_function("part 1", |b| {
        let lines = utils::load_input("inputs/day_13").expect("could not load input");
        let manual = day_13::parse_input(lines).expect("could not parse input");

        b.iter(|| {
            black_box(day_13::part_one(&manual))
        })
    });
    group.bench_function("part 2", |b| {
        let lines = utils::load_input("inputs/day_13").expect("could not load input");
        let manual = day_13::parse_input(lines).expect("could not parse input");

        b.iter(|| {
            black_box(day_13::part_two(&manual))
        })
    });
    group.finish();
}

criterion_group!(benches, benchmark);
