pub fn part_one(commands: &Vec<(&str, u64)>) -> u64 {
    commands
        .iter()
        .fold(
            [0 as u64, 0 as u64],
            |[pos, depth], (dir, dist)| match *dir {
                "forward" => [pos + dist, depth],
                "down" => [pos, depth + dist],
                "up" => [pos, depth - dist],
                _ => [pos, depth],
            },
        )
        .iter()
        .fold(1, |acc, x| acc * x)
}

pub fn part_two(commands: &Vec<(&str, u64)>) -> u64 {
    commands
        .iter()
        .fold(
            ([0 as u64, 0 as u64], 0 as u64),
            |([pos, depth], aim), (dir, dist)| match *dir {
                "forward" => ([pos + dist, depth + dist * aim], aim),
                "down" => ([pos, depth], aim + dist),
                "up" => ([pos, depth], aim - dist),
                _ => ([pos, depth], aim),
            },
        )
        .0
        .iter()
        .fold(1, |acc, x| acc * x)
}
