use aoc_2021::utils;
use aoc_2021::day_01;

fn main() {
    let lines = utils::load_input("inputs/day_01").expect("could not load input");
    let depths = lines
        .iter()
        .map(|x| {
            x.parse::<u64>()
                .expect("could not parse invalid integer string")
        })
        .collect::<Vec<u64>>();

    println!("part 1: {}", day_01::part_one(&depths));
    println!("part 2: {}", day_01::part_two(&depths));
}
