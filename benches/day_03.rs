use aoc_2021::{day_03, utils};
use criterion::{black_box, criterion_group, Criterion};

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day 03: binary diagnostic");

    group.bench_function("part 1", |b| {
        let lines = utils::load_input("inputs/day_03").expect("could not load input");
        let diagnostics: Vec<u64> = lines
            .iter()
            .map(|line| {
                u64::from_str_radix(line, 2).expect("could not parse invalid binary integer string")
            })
            .collect();

        b.iter(|| black_box(day_03::part_one(&diagnostics)))
    });
    group.bench_function("part 2", |b| {
        let lines = utils::load_input("inputs/day_03").expect("could not load input");
        let diagnostics: Vec<u64> = lines
            .iter()
            .map(|line| {
                u64::from_str_radix(line, 2).expect("could not parse invalid binary integer string")
            })
            .collect();

        b.iter(|| black_box(day_03::part_two(&diagnostics)))
    });
    group.finish();
}

criterion_group!(benches, benchmark);
