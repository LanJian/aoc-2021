use aoc_2021::{utils, day_02};


fn main() {
    let lines = utils::load_input("inputs/day_02").expect("could not load input");
    let commands: Vec<(&str, u64)> = lines.iter().map(|line| {
        let tokens: Vec<&str> = line.split(" ").collect();
        (
            tokens[0],
            tokens[1].parse::<u64>().expect("could not parse invalid integer string")
        )
    }).collect();
    
    println!("part 1: {}", day_02::part_one(&commands));
    println!("part 2: {}", day_02::part_two(&commands));
}
