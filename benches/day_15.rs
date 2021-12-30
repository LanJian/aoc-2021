use aoc_2021::{
    utils, day_15,
};
use criterion::{black_box, criterion_group, Criterion};

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day 15: chiton");

    group.bench_function("part 1", |b| {
        let lines = utils::load_input("inputs/day_15").expect("could not load input");
        let cavern = day_15::parse_input(lines).expect("could not parse input");

        b.iter(|| {
            black_box(day_15::part_one(&cavern))
        })
    });
    group.bench_function("part 2", |b| {
        let lines = utils::load_input("inputs/day_15").expect("could not load input");
        let cavern = day_15::parse_input(lines).expect("could not parse input");

        b.iter(|| {
            black_box(day_15::part_two(&cavern))
        })
    });
    group.finish();
}

criterion_group!(benches, benchmark);
