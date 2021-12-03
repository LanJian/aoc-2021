use aoc_2021::{day_03, utils};

fn main() {
    let lines = utils::load_lines("inputs/day_03").expect("could not load input");
    let diagnostics: Vec<u64> = lines
        .iter()
        .map(|line| {
            u64::from_str_radix(line, 2).expect("could not parse invalid binary integer string")
        })
        .collect();

    println!("part 1: {}", day_03::part_one(&diagnostics));
    println!("part 2: {}", day_03::part_two(&diagnostics));
}
