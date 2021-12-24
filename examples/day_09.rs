use aoc_2021::{day_09::{parse_input, self}, utils};

fn main() {
    let lines = utils::load_input("inputs/day_09").expect("could not load input");
    let height_map = parse_input(lines).expect("could not parse input");

    println!("part 1: {}", day_09::part_one(&height_map));
    println!("part 2: {}", day_09::part_two(&height_map));
}
