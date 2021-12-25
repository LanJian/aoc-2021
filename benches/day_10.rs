use aoc_2021::{
    utils, day_10,
};
use criterion::{black_box, criterion_group, Criterion};

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day 10: syntax scoring");

    group.bench_function("part 1", |b| {
        let lines = utils::load_input("inputs/day_10").expect("could not load input");
        let subsystem = day_10::parse_input(lines).expect("could not parse input");

        b.iter(|| {
            black_box(day_10::part_one(&subsystem))
        })
    });
    group.bench_function("part 2", |b| {
        let lines = utils::load_input("inputs/day_10").expect("could not load input");
        let subsystem = day_10::parse_input(lines).expect("could not parse input");

        b.iter(|| {
            black_box(day_10::part_two(&subsystem))
        })
    });
    group.finish();
}

criterion_group!(benches, benchmark);
