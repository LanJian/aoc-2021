use aoc_2021::{utils, day_01};
use criterion::{ Criterion, criterion_group, black_box };

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day 01: sonar sweep");

    let lines = utils::load_input("inputs/day_01").expect("could not load input");
    let depths = lines
        .iter()
        .map(|x| {
            x.parse::<u64>()
                .expect("could not parse invalid integer string")
        })
        .collect::<Vec<u64>>();

    group.bench_function("part 1", |b| b.iter(|| black_box(day_01::part_one(&depths))));
    group.bench_function("part 2", |b| b.iter(|| black_box(day_01::part_two(&depths))));
    group.finish();
}

criterion_group!(benches, benchmark);
