use aoc_2021::{day_07::{parse_input, self}, utils};

fn main() {
    let lines = utils::load_input("inputs/day_07").expect("could not load input");
    let fleet = parse_input(lines).expect("could not parse input");

    println!("part 1: {}", day_07::part_one(&fleet));
    println!("part 2: {}", day_07::part_two(&fleet));
}
