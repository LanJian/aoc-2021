use aoc_2021::{
    utils, day_07,
};
use criterion::{black_box, criterion_group, Criterion};

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day 07: the treachery of whales");

    group.bench_function("part 1", |b| {
        let lines = utils::load_input("inputs/day_07").expect("could not load input");
        let fleet = day_07::parse_input(lines).expect("could not parse input");

        b.iter(|| {
            black_box(day_07::part_one(&fleet))
        })
    });
    group.bench_function("part 2", |b| {
        let lines = utils::load_input("inputs/day_07").expect("could not load input");
        let fleet = day_07::parse_input(lines).expect("could not parse input");

        b.iter(|| {
            black_box(day_07::part_two(&fleet))
        })
    });
    group.finish();
}

criterion_group!(benches, benchmark);
