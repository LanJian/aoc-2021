use aoc_2021::{
    utils, day_05,
};
use criterion::{black_box, criterion_group, Criterion};

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day 05: hydrothermal venture");

    group.bench_function("part 1", |b| {
        let lines = utils::load_input("inputs/day_05").expect("could not load input");
        let vents = day_05::parse_input(lines).expect("could not parse input");

        b.iter(|| {
            black_box(day_05::part_one(&vents))
        })
    });
    group.bench_function("part 2", |b| {
        let lines = utils::load_input("inputs/day_05").expect("could not load input");
        let vents = day_05::parse_input(lines).expect("could not parse input");

        b.iter(|| {
            black_box(day_05::part_two(&vents))
        })
    });
    group.finish();
}

criterion_group!(benches, benchmark);
