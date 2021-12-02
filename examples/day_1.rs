use aoc_2021::utils;
use aoc_2021::day_1;

fn main() {
    let lines = utils::load_lines("inputs/day_1").expect("could not load input");
    let depths = lines
        .iter()
        .map(|x| {
            x.parse::<u64>()
                .expect("could not parse invalid integer string")
        })
        .collect::<Vec<u64>>();

    println!("part 1: {}", day_1::part_one(&depths));
    println!("part 2: {}", day_1::part_two(&depths));
}
