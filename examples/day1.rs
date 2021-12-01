use aoc_2021::utils;

fn part_one(lines: &Vec<String>) -> i32 {
    lines
        .iter()
        .fold((0, None), |(count, prev), x| {
            let cur_val = x
                .parse::<i32>()
                .expect("could not parse invalid integer string");

            match prev {
                Some(prev_val) if cur_val > prev_val => (count + 1, Some(cur_val)),
                _ => (count, Some(cur_val)),
            }
        })
        .0
}

fn part_two(lines: &Vec<String>) -> i32 {
    lines
        .iter()
        .fold((0, None, None, None), |(count, a, b, c), x| {
            let cur_val = x
                .parse::<i32>()
                .expect("could not parse invalid integer string");

            match (a, b, c) {
                (Some(aa), Some(bb), Some(cc)) => { 
                    if bb + cc + cur_val > aa + bb + cc {
                        (count + 1, Some(bb), Some(cc), Some(cur_val))
                    } else {
                        (count, Some(bb), Some(cc), Some(cur_val))
                    }
                }
                (None, Some(bb), Some(cc)) => (count, Some(bb), Some(cc), Some(cur_val)),
                (None, None, Some(cc)) => (count, None, Some(cc), Some(cur_val)),
                _ => (count, None, None, Some(cur_val)),
            }
        })
        .0
}

fn main() {
    let lines = utils::load_lines("inputs/day1").expect("could not load input");

    println!("part one: {}", part_one(&lines));
    println!("part two: {}", part_two(&lines));
}
