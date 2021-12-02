use aoc_2021::{utils, day_1};
use criterion::{ Criterion, criterion_group, black_box };

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day1: sonar sweep");

    let lines = utils::load_lines("inputs/day_1").expect("could not load input");
    let depths = lines
        .iter()
        .map(|x| {
            x.parse::<u64>()
                .expect("could not parse invalid integer string")
        })
        .collect::<Vec<u64>>();

    group.bench_function("part 1", |b| b.iter(|| black_box(day_1::part_one(&depths))));
    group.bench_function("part 2", |b| b.iter(|| black_box(day_1::part_two(&depths))));
    group.finish();
}

criterion_group!(benches, benchmark);
