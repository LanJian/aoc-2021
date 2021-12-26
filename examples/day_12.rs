use aoc_2021::{day_12::{parse_input, self}, utils};

fn main() {
    let lines = utils::load_input("inputs/day_12").expect("could not load input");
    let edges = parse_input(lines).expect("could not parse input");

    println!("part 1: {}", day_12::part_one(edges.clone()));
    println!("part 2: {}", day_12::part_two(edges.clone()));
}
