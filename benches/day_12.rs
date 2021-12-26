use aoc_2021::{
    utils, day_12,
};
use criterion::{black_box, criterion_group, Criterion};

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day 12: passage pathing");

    group.bench_function("part 1", |b| {
        let lines = utils::load_input("inputs/day_12").expect("could not load input");
        let edges = day_12::parse_input(lines).expect("could not parse input");

        b.iter(|| {
            black_box(day_12::part_one(edges.clone()))
        })
    });
    group.bench_function("part 2", |b| {
        let lines = utils::load_input("inputs/day_12").expect("could not load input");
        let edges = day_12::parse_input(lines).expect("could not parse input");

        b.iter(|| {
            black_box(day_12::part_two(edges.clone()))
        })
    });
    group.finish();
}

criterion_group!(benches, benchmark);
