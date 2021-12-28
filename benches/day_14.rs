use aoc_2021::{
    utils, day_14,
};
use criterion::{black_box, criterion_group, Criterion};

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day 14: extended polymerization");

    group.bench_function("part 1", |b| {
        let lines = utils::load_input("inputs/day_14").expect("could not load input");
        let polymer = day_14::parse_input(lines).expect("could not parse input");

        b.iter(|| {
            black_box(day_14::part_one(&polymer))
        })
    });
    group.bench_function("part 2", |b| {
        let lines = utils::load_input("inputs/day_14").expect("could not load input");
        let polymer = day_14::parse_input(lines).expect("could not parse input");

        b.iter(|| {
            black_box(day_14::part_two(&polymer))
        })
    });
    group.finish();
}

criterion_group!(benches, benchmark);
