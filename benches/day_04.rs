use std::convert::TryFrom;

use aoc_2021::{
    day_04::{self, BingoBoard},
    utils,
};
use criterion::{black_box, criterion_group, Criterion};

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day 04: giant squid");

    group.bench_function("part 1", |b| {
        let lines = utils::load_input("inputs/day_04").expect("could not load input");
        let mut sections = lines.split(|line| line.is_empty());

        let sequence: Vec<i64> = sections.next().expect("invalid input")[0]
            .split(",")
            .map(|x| x.parse::<i64>().expect("could not parse integer string"))
            .collect();

        let boards: Vec<BingoBoard> = sections
            .map(|section| BingoBoard::try_from(section).expect("could not parse bingo board"))
            .collect();

        b.iter(|| {
            let mut boards = boards.clone();
            black_box(day_04::part_one(&sequence, &mut boards).expect("no winning board"))
        })
    });
    group.bench_function("part 2", |b| {
        let lines = utils::load_input("inputs/day_04").expect("could not load input");
        let mut sections = lines.split(|line| line.is_empty());

        let sequence: Vec<i64> = sections.next().expect("invalid input")[0]
            .split(",")
            .map(|x| x.parse::<i64>().expect("could not parse integer string"))
            .collect();

        let boards: Vec<BingoBoard> = sections
            .map(|section| BingoBoard::try_from(section).expect("could not parse bingo board"))
            .collect();

        b.iter(|| {
            let mut boards = boards.clone();
            black_box(day_04::part_two(&sequence, &mut boards).expect("no winning board"))
        })
    });
    group.finish();
}

criterion_group!(benches, benchmark);
