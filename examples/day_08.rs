use aoc_2021::{day_08::{parse_input, self}, utils};

fn main() {
    let lines = utils::load_input("inputs/day_08").expect("could not load input");
    let displays = parse_input(lines).expect("could not parse input");

    println!("part 1: {}", day_08::part_one(&displays));
    println!("part 2: {}", day_08::part_two(&displays));
}
