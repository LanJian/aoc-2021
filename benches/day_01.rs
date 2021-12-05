use aoc_2021::{day_01, utils};
use criterion::{black_box, criterion_group, Criterion};

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day 01: sonar sweep");

    group.bench_function("part 1", |b| {
        let lines = utils::load_input("inputs/day_01").expect("could not load input");
        let depths = lines
            .iter()
            .map(|x| {
                x.parse::<u64>()
                    .expect("could not parse invalid integer string")
            })
            .collect::<Vec<u64>>();

        b.iter(|| black_box(day_01::part_one(&depths)))
    });
    group.bench_function("part 2", |b| {
        let lines = utils::load_input("inputs/day_01").expect("could not load input");
        let depths = lines
            .iter()
            .map(|x| {
                x.parse::<u64>()
                    .expect("could not parse invalid integer string")
            })
            .collect::<Vec<u64>>();

        b.iter(|| black_box(day_01::part_two(&depths)))
    });
    group.finish();
}

criterion_group!(benches, benchmark);
