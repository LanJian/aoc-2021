use aoc_2021::{day_16::{parse_input, self}, utils};

fn main() {
    let lines = utils::load_input("inputs/day_16").expect("could not load input");
    let transmission = parse_input(lines).expect("could not parse input");

    println!("part 1: {}", day_16::part_one(&transmission));
    println!("part 2: {}", day_16::part_two(&transmission));
}
