use aoc_2021::{
    utils, day_08,
};
use criterion::{black_box, criterion_group, Criterion};

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day 08: REPLACE ME");

    group.bench_function("part 1", |b| {
        let lines = utils::load_input("inputs/day_08").expect("could not load input");
        let displays = day_08::parse_input(lines).expect("could not parse input");

        b.iter(|| {
            black_box(day_08::part_one(&displays))
        })
    });
    group.bench_function("part 2", |b| {
        let lines = utils::load_input("inputs/day_08").expect("could not load input");
        let displays = day_08::parse_input(lines).expect("could not parse input");

        b.iter(|| {
            black_box(day_08::part_two(&displays))
        })
    });
    group.finish();
}

criterion_group!(benches, benchmark);
