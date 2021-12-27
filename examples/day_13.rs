use aoc_2021::{day_13::{parse_input, self}, utils};

fn main() {
    let lines = utils::load_input("inputs/day_13").expect("could not load input");
    let manual = parse_input(lines).expect("could not parse input");

    println!("part 1: {}", day_13::part_one(&manual));
    println!("part 2:\n{}", day_13::part_two(&manual));
}
