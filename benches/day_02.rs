use aoc_2021::{day_02, utils};
use criterion::{black_box, criterion_group, Criterion};

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day 02: dive");

    group.bench_function("part 1", |b| {
        let lines = utils::load_input("inputs/day_02").expect("could not load input");
        let commands: Vec<(&str, u64)> = lines
            .iter()
            .map(|line| {
                let tokens: Vec<&str> = line.split(" ").collect();
                (
                    tokens[0],
                    tokens[1]
                        .parse::<u64>()
                        .expect("could not parse invalid integer string"),
                )
            })
            .collect();

        b.iter(|| black_box(day_02::part_one(&commands)))
    });
    group.bench_function("part 2", |b| {
        let lines = utils::load_input("inputs/day_02").expect("could not load input");
        let commands: Vec<(&str, u64)> = lines
            .iter()
            .map(|line| {
                let tokens: Vec<&str> = line.split(" ").collect();
                (
                    tokens[0],
                    tokens[1]
                        .parse::<u64>()
                        .expect("could not parse invalid integer string"),
                )
            })
            .collect();

        b.iter(|| black_box(day_02::part_two(&commands)))
    });
    group.finish();
}

criterion_group!(benches, benchmark);
