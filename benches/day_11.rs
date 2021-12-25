use aoc_2021::{
    utils, day_11,
};
use criterion::{black_box, criterion_group, Criterion};

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day 11: dumbo octopus");

    group.bench_function("part 1", |b| {
        let lines = utils::load_input("inputs/day_11").expect("could not load input");
        let energy_map = day_11::parse_input(lines).expect("could not parse input");

        b.iter(|| {
            black_box(day_11::part_one(&energy_map))
        })
    });
    group.bench_function("part 2", |b| {
        let lines = utils::load_input("inputs/day_11").expect("could not load input");
        let energy_map = day_11::parse_input(lines).expect("could not parse input");

        b.iter(|| {
            black_box(day_11::part_two(&energy_map))
        })
    });
    group.finish();
}

criterion_group!(benches, benchmark);
