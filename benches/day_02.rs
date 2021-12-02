use aoc_2021::{utils, day_02};
use criterion::{ Criterion, criterion_group, black_box };


fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day 02: dive");

    let lines = utils::load_lines("inputs/day_02").expect("could not load input");
    let commands: Vec<(&str, u64)> = lines.iter().map(|line| {
        let tokens: Vec<&str> = line.split(" ").collect();
        (
            tokens[0],
            tokens[1].parse::<u64>().expect("could not parse invalid integer string")
        )
    }).collect();

    group.bench_function("part 1", |b| b.iter(|| black_box(day_02::part_one(&commands))));
    group.bench_function("part 2", |b| b.iter(|| black_box(day_02::part_two(&commands))));
    group.finish();
}

criterion_group!(benches, benchmark);
