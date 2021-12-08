use aoc_2021::{
    utils, day_06,
};
use criterion::{black_box, criterion_group, Criterion};

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day 06: Lanternfish");

    group.bench_function("part 1", |b| {
        let lines = utils::load_input("inputs/day_06").expect("could not load input");
        let school = day_06::parse_input(lines).expect("could not parse input");

        b.iter(|| {
            black_box(day_06::part_one(&school))
        })
    });
    group.bench_function("part 2", |b| {
        let lines = utils::load_input("inputs/day_06").expect("could not load input");
        let school = day_06::parse_input(lines).expect("could not parse input");

        b.iter(|| {
            black_box(day_06::part_two(&school))
        })
    });
    group.finish();
}

criterion_group!(benches, benchmark);
