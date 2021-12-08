use aoc_2021::{utils, day_06};

//use aoc_2021::{day_06, utils};

fn main() {
    let lines = utils::load_input("inputs/day_06").expect("could not load input");
    let school = day_06::parse_input(lines).expect("could not parse input");

    println!("part 1: {}", day_06::part_one(&school));
    println!("part 2: {}", day_06::part_two(&school));
}
