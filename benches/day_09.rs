use aoc_2021::{
    utils, day_09,
};
use criterion::{black_box, criterion_group, Criterion};

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day 09: smoke basin");

    group.bench_function("part 1", |b| {
        let lines = utils::load_input("inputs/day_09").expect("could not load input");
        let height_map = day_09::parse_input(lines).expect("could not parse input");

        b.iter(|| {
            black_box(day_09::part_one(&height_map))
        })
    });
    group.bench_function("part 2", |b| {
        let lines = utils::load_input("inputs/day_09").expect("could not load input");
        let height_map = day_09::parse_input(lines).expect("could not parse input");

        b.iter(|| {
            black_box(day_09::part_two(&height_map))
        })
    });
    group.finish();
}

criterion_group!(benches, benchmark);
