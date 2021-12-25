use aoc_2021::{day_11::{parse_input, self}, utils};

fn main() {
    let lines = utils::load_input("inputs/day_11").expect("could not load input");
    let energy_map = parse_input(lines).expect("could not parse input");

    println!("part 1: {}", day_11::part_one(&energy_map));
    println!("part 2: {}", day_11::part_two(&energy_map));
}
