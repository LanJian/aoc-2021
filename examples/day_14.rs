use aoc_2021::{day_14::{parse_input, self}, utils};

fn main() {
    let lines = utils::load_input("inputs/day_14").expect("could not load input");
    let polymer = parse_input(lines).expect("could not parse input");

    println!("part 1: {}", day_14::part_one(&polymer));
    println!("part 2: {}", day_14::part_two(&polymer));
}
