use aoc_2021::{day_10::{parse_input, self}, utils};

fn main() {
    let lines = utils::load_input("inputs/day_10").expect("could not load input");
    let subsystem = parse_input(lines).expect("could not parse input");

    println!("part 1: {}", day_10::part_one(&subsystem));
    println!("part 2: {}", day_10::part_two(&subsystem));
}
