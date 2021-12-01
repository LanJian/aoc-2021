use aoc_2021::utils;

fn part_one(lines: &Vec<u64>) -> u64 {
    lines
        .iter()
        .fold((0, None), |(count, prev), x| match prev {
            Some(prev_val) if x > prev_val => (count + 1, Some(x)),
            _ => (count, Some(x)),
        })
        .0
}

fn part_two(lines: &Vec<u64>) -> u64 {
    lines
        .iter()
        .fold((0, None, None, None), |(count, a, b, c), x| {
            match (a, b, c) {
                (Some(aa), Some(bb), Some(cc)) => {
                    if bb + cc + x > aa + bb + cc {
                        (count + 1, Some(bb), Some(cc), Some(x))
                    } else {
                        (count, Some(bb), Some(cc), Some(x))
                    }
                }
                (None, Some(bb), Some(cc)) => (count, Some(bb), Some(cc), Some(x)),
                (None, None, Some(cc)) => (count, None, Some(cc), Some(x)),
                _ => (count, None, None, Some(x)),
            }
        })
        .0
}

fn main() {
    let lines = utils::load_lines("inputs/day1").expect("could not load input");
    let depths = lines
        .iter()
        .map(|x| {
            x.parse::<u64>()
                .expect("could not parse invalid integer string")
        })
        .collect::<Vec<u64>>();

    println!("part one: {}", part_one(&depths));
    println!("part two: {}", part_two(&depths));
}
