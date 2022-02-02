use aoc_2021::{
    utils, day_16,
};
use criterion::{black_box, criterion_group, Criterion};

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day 16: packet decoder");

    group.bench_function("part 1", |b| {
        let lines = utils::load_input("inputs/day_16").expect("could not load input");
        let transmission = day_16::parse_input(lines).expect("could not parse input");

        b.iter(|| {
            black_box(day_16::part_one(&transmission))
        })
    });
    group.bench_function("part 2", |b| {
        let lines = utils::load_input("inputs/day_16").expect("could not load input");
        let transmission = day_16::parse_input(lines).expect("could not parse input");

        b.iter(|| {
            black_box(day_16::part_two(&transmission))
        })
    });
    group.finish();
}

criterion_group!(benches, benchmark);
