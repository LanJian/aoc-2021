pub fn part_one(diagnostics: &Vec<u64>) -> u64 {
    let mut counts: Vec<usize> = Vec::new();

    for elem in diagnostics {
        let mut reading = *elem;
        let mut index = 0;

        while reading > 0 {
            if counts.len() <= index {
                counts.push(0);
            }
            counts[index] += (reading % 2) as usize;
            reading >>= 1;
            index += 1;
        }
    }

    let half_len = diagnostics.len() / 2;
    let gamma: u64 = counts
        .iter()
        .map(|x| if *x > half_len { 1 } else { 0 })
        .rfold(0, |acc, x| (acc << 1) + x);

    let epsilon = gamma ^ ((1 << counts.len()) - 1);

    gamma * epsilon
}

fn get_num_bits(num: u64) -> usize {
    let mut num_bits = 0;
    let mut acc = num;

    while acc > 0 {
        num_bits += 1;
        acc >>= 1;
    }

    num_bits
}

fn get_majority_bit(readings: &Vec<u64>, bit_mask: u64) -> usize {
    let half_len = (readings.len() + 1) / 2;
    let mut count: usize = 0;
    for reading in readings {
        let bit = ((reading & bit_mask) / bit_mask) as usize;
        count += bit;
    }

    if count >= half_len {
        1
    } else {
        0
    }
}

fn get_minority_bit(readings: &Vec<u64>, bit_mask: u64) -> usize {
    get_majority_bit(readings, bit_mask) ^ 1
}

pub fn part_two(diagnostics: &Vec<u64>) -> u64 {
    let max_reading = diagnostics.iter().max();

    let num_bits = match max_reading {
        Some(val) => get_num_bits(*val),
        None => 0,
    };

    if num_bits == 0 {
        return 0;
    }

    let mut o2_sift: Vec<u64> = diagnostics.clone();
    let mut co2_sift: Vec<u64> = diagnostics.clone();

    let mut bit_mask = 1 << (num_bits - 1);
    while o2_sift.len() > 1 || co2_sift.len() > 1 {
        // filter o2
        if o2_sift.len() > 1 {
            let majority_bit = get_majority_bit(&o2_sift, bit_mask);
            o2_sift = o2_sift
                .into_iter()
                .filter(|x| {
                    let bit = ((*x & bit_mask) / bit_mask) as usize;
                    bit == majority_bit
                })
                .collect();
        }

        // filter co2
        if co2_sift.len() > 1 {
            let minority_bit = get_minority_bit(&co2_sift, bit_mask);
            co2_sift = co2_sift
                .into_iter()
                .filter(|x| {
                    let bit = ((*x & bit_mask) / bit_mask) as usize;
                    bit == minority_bit
                })
                .collect();
        }

        bit_mask >>= 1;
    }

    o2_sift[0] * co2_sift[0]
}
