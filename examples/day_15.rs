use aoc_2021::{day_15::{parse_input, self}, utils};

fn main() {
    let lines = utils::load_input("inputs/day_15").expect("could not load input");
    let cavern = parse_input(lines).expect("could not parse input");

    println!("part 1: {}", day_15::part_one(&cavern));
    println!("part 2: {}", day_15::part_two(&cavern));
}
