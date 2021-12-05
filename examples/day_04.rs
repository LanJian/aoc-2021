use std::convert::TryFrom;

use aoc_2021::{day_04::{self, BingoBoard}, utils};

fn main() {
    let lines = utils::load_input("inputs/day_04").expect("could not load input");
    let mut sections = lines.split(|line| line.is_empty());

    let sequence: Vec<i64> = sections.next().expect("invalid input")[0]
        .split(",")
        .map(|x| x.parse::<i64>().expect("could not parse integer string"))
        .collect();

    let mut boards: Vec<BingoBoard> = sections.map(|section| {
        BingoBoard::try_from(section).expect("could not parse bingo board")
    }).collect();

    println!("part 1: {}", day_04::part_one(&sequence, &mut boards).expect("no winning board"));
    println!("part 2: {}", day_04::part_two(&sequence, &mut boards).expect("no winning board"));
}
