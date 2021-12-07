use aoc_2021::{day_05::{parse_input, self}, utils};

fn main() {
    let lines = utils::load_input("inputs/day_05").expect("could not load input");
    let vents = parse_input(lines).expect("could not parse input");

    println!("part 1: {}", day_05::part_one(&vents));
    println!("part 2: {}", day_05::part_two(&vents));
}
