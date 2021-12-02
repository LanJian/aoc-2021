pub fn part_one(lines: &Vec<u64>) -> u64 {
    lines
        .iter()
        .fold((0, None), |(count, prev), x| match prev {
            Some(prev_val) if x > prev_val => (count + 1, Some(x)),
            _ => (count, Some(x)),
        })
        .0
}

pub fn part_two(lines: &Vec<u64>) -> u64 {
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
