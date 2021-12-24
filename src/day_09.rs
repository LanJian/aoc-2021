use std::{
    collections::{HashSet, VecDeque},
    convert::TryFrom,
};

#[derive(Debug, PartialEq, Eq)]
pub struct HeightMap {
    grid: Vec<Vec<usize>>,
    n: usize,
    m: usize,
}

impl TryFrom<Vec<String>> for HeightMap {
    type Error = String;

    fn try_from(lines: Vec<String>) -> Result<Self, Self::Error> {
        let grid = lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| match c.to_digit(10) {
                        Some(digit) => Ok(digit as usize),
                        None => Err("could not parse digit".to_string()),
                    })
                    .collect::<Result<Vec<usize>, String>>()
            })
            .collect::<Result<Vec<Vec<usize>>, String>>()?;

        Ok(grid.into())
    }
}

impl From<Vec<Vec<usize>>> for HeightMap {
    fn from(grid: Vec<Vec<usize>>) -> Self {
        let n = grid.len();
        let m = grid[0].len();
        Self { grid, n, m }
    }
}

impl HeightMap {
    pub fn total_risk(&self) -> i64 {
        let mut risk = 0;

        for i in 0..self.n {
            for j in 0..self.m {
                if self.is_low_point(i, j) {
                    risk += self.risk_level(i, j);
                }
            }
        }

        risk
    }

    fn is_low_point(&self, i: usize, j: usize) -> bool {
        let signed_i = i as i64;
        let signed_j = j as i64;
        let signed_n = self.n as i64;
        let signed_m = self.m as i64;

        [
            (signed_i - 1, signed_j),
            (signed_i + 1, signed_j),
            (signed_i, signed_j - 1),
            (signed_i, signed_j + 1),
        ]
        .iter()
        .all(|(ii, jj)| {
            if !(0..signed_n).contains(ii)
                || !(0..signed_m).contains(jj)
                || self.grid[i][j] < self.grid[*ii as usize][*jj as usize]
            {
                true
            } else {
                false
            }
        })
    }

    fn risk_level(&self, i: usize, j: usize) -> i64 {
        (self.grid[i][j] + 1) as i64
    }

    pub fn basin_sizes(&self) -> Vec<usize> {
        let mut sizes: Vec<usize> = Vec::new();
        let mut visited: HashSet<(i64, i64)> = HashSet::new();

        for i in 0..self.n {
            for j in 0..self.m {
                let size = self.basin_size(i, j, &mut visited);
                if size > 0 {
                    sizes.push(size)
                }
            }
        }

        sizes
    }

    fn basin_size(&self, i: usize, j: usize, visited: &mut HashSet<(i64, i64)>) -> usize {
        let signed_n = self.n as i64;
        let signed_m = self.m as i64;

        let mut q: VecDeque<(i64, i64)> = VecDeque::new();
        let mut size = 0;

        q.push_back((i as i64, j as i64));

        while let Some((cur_i, cur_j)) = q.pop_front() {
            if !(0..signed_n).contains(&cur_i)
                || !(0..signed_m).contains(&cur_j)
                || visited.contains(&(cur_i, cur_j))
            {
                continue;
            }

            visited.insert((cur_i, cur_j));
            let height = self.grid[cur_i as usize][cur_j as usize];
            if height == 9 {
                continue;
            }

            size += 1;
            q.push_back((cur_i - 1, cur_j));
            q.push_back((cur_i + 1, cur_j));
            q.push_back((cur_i, cur_j - 1));
            q.push_back((cur_i, cur_j + 1));
        }

        size
    }
}

pub fn parse_input(lines: Vec<String>) -> Result<HeightMap, String> {
    HeightMap::try_from(lines)
}

pub fn part_one(height_map: &HeightMap) -> i64 {
    height_map.total_risk()
}

pub fn part_two(height_map: &HeightMap) -> i64 {
    let mut basin_sizes =  height_map.basin_sizes();
    basin_sizes.sort();
    basin_sizes.reverse();
    basin_sizes.iter().take(3).map(|x| *x as i64).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input_test() {
        let lines = vec!["123".to_string(), "419".to_string(), "950".to_string()];

        let actual = parse_input(lines).expect("could not parse input");
        let expected = HeightMap::from(vec![vec![1, 2, 3], vec![4, 1, 9], vec![9, 5, 0]]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn part_one_test() {
        let lines = vec![
            "2199943210".to_string(),
            "3987894921".to_string(),
            "9856789892".to_string(),
            "8767896789".to_string(),
            "9899965678".to_string(),
        ];
        let height_map = parse_input(lines).expect("could not parse input");

        assert_eq!(part_one(&height_map), 15);
    }

    #[test]
    fn part_two_test() {
        let lines = vec![
            "2199943210".to_string(),
            "3987894921".to_string(),
            "9856789892".to_string(),
            "8767896789".to_string(),
            "9899965678".to_string(),
        ];
        let height_map = parse_input(lines).expect("could not parse input");

        assert_eq!(part_two(&height_map), 1134);
    }
}
