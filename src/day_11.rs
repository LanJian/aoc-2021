use std::{convert::TryFrom, collections::HashSet, ops::{Index, IndexMut}};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Coordinate(i64, i64);

impl From<(usize, usize)> for Coordinate {
    fn from(coords: (usize, usize)) -> Self {
        Coordinate(coords.0 as i64, coords.1 as i64)
    }
}

impl Coordinate {
    pub fn north(&self) -> Self {
        Self(self.0 - 1, self.1)
    }

    pub fn south(&self) -> Self {
        Self(self.0 + 1, self.1)
    }

    pub fn east(&self) -> Self {
        Self(self.0, self.1 + 1)
    }

    pub fn west(&self) -> Self {
        Self(self.0, self.1 - 1)
    }

    pub fn northeast(&self) -> Self {
        self.north().east()
    }

    pub fn northwest(&self) -> Self {
        self.north().west()
    }

    pub fn southeast(&self) -> Self {
        self.south().east()
    }

    pub fn southwest(&self) -> Self {
        self.south().west()
    }

    pub fn neighbours(&self) -> [Self; 8] {
        [
            self.north(),
            self.south(),
            self.east(),
            self.west(),
            self.northeast(),
            self.northwest(),
            self.southeast(),
            self.southwest(),
        ]
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EnergyMap {
    grid: Vec<Vec<usize>>,
    n: usize,
    m: usize,
}

impl TryFrom<Vec<String>> for EnergyMap {
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

impl From<Vec<Vec<usize>>> for EnergyMap {
    fn from(grid: Vec<Vec<usize>>) -> Self {
        let n = grid.len();
        let m = grid[0].len();
        Self { grid, n, m }
    }
}

impl Index<Coordinate> for EnergyMap {
    type Output = usize;

    fn index(&self, idx: Coordinate) -> &Self::Output {
        &self.grid[idx.0 as usize][idx.1 as usize]
    }
}

impl IndexMut<Coordinate> for EnergyMap {
    fn index_mut(&mut self, idx: Coordinate) -> &mut Self::Output {
        &mut self.grid[idx.0 as usize][idx.1 as usize]
    }
}

impl EnergyMap {
    /// Simulates for the given number of steps, returns the total number of flashes that ocurred
    pub fn simulate(&mut self, steps: usize) -> usize {
        let mut total = 0;

        for _ in 0..steps {
            total += self.step();
        }

        total
    }

    /// Simulates until a synchronised flash occurs, returns the step in which it happens
    pub fn find_synchronisation_step(&mut self) -> usize {
        let mut step = 0;

        loop {
            let num_flashes = self.step();
            step += 1;
            if num_flashes == self.n * self.m {
                return step
            }
        }
    }

    /// Simulates 1 step, returns the number of flashes that ocurred
    fn step(&mut self) -> usize {
        // first increase energy level of everything by 1
        for i in 0..self.n {
            for j in 0..self.m {
                self.grid[i][j] += 1;
            }
        }

        // then do the fireworks
        let mut flashed: HashSet<Coordinate> = HashSet::new();
        for i in 0..self.n {
            for j in 0..self.m {
                self.dfs(Coordinate::from((i, j)), &mut flashed)
            }
        }

        // lastly set all the flashed coordinates to 0
        for coord in &flashed {
            self[*coord] = 0;
        }

        flashed.len()
    }

    fn dfs(&mut self, coord: Coordinate, flashed: &mut HashSet<Coordinate>) {
        let energy_level = self[coord];

        if energy_level <= 9 || flashed.contains(&coord) {
            return;
        }

        flashed.insert(coord);

        for neighbour in coord.neighbours() {
            if self.is_in_bounds(neighbour) {
                self[neighbour] += 1;
                self.dfs(neighbour, flashed)
            }
        }
    }

    fn is_in_bounds(&self, coord: Coordinate) -> bool {
        (0..self.n as i64).contains(&coord.0) && (0..self.m as i64).contains(&coord.1)
    }
}

pub fn parse_input(lines: Vec<String>) -> Result<EnergyMap, String> {
    EnergyMap::try_from(lines)
}

pub fn part_one(energy_map: &EnergyMap) -> usize {
    let mut cloned = energy_map.clone();
    cloned.simulate(100)
}

pub fn part_two(energy_map: &EnergyMap) -> usize {
    let mut cloned = energy_map.clone();
    cloned.find_synchronisation_step()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simulate_test() {
        let lines = vec![
            "11111".to_string(),
            "19991".to_string(),
            "19191".to_string(),
            "19991".to_string(),
            "11111".to_string(),
        ];
        let mut energy_map = parse_input(lines).expect("could not parse input");
        
        assert_eq!(energy_map.simulate(2), 9);
    }

    #[test]
    fn parse_input_test() {
        let lines = vec!["123".to_string(), "419".to_string(), "950".to_string()];

        let actual = parse_input(lines).expect("could not parse input");
        let expected = EnergyMap::from(vec![vec![1, 2, 3], vec![4, 1, 9], vec![9, 5, 0]]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn part_one_test() {
        let lines = vec![
            "5483143223".to_string(),
            "2745854711".to_string(),
            "5264556173".to_string(),
            "6141336146".to_string(),
            "6357385478".to_string(),
            "4167524645".to_string(),
            "2176841721".to_string(),
            "6882881134".to_string(),
            "4846848554".to_string(),
            "5283751526".to_string(),
        ];
        let energy_map = parse_input(lines).expect("could not parse input");

        assert_eq!(part_one(&energy_map), 1656);
    }

    #[test]
    fn part_two_test() {
        let lines = vec![
            "5483143223".to_string(),
            "2745854711".to_string(),
            "5264556173".to_string(),
            "6141336146".to_string(),
            "6357385478".to_string(),
            "4167524645".to_string(),
            "2176841721".to_string(),
            "6882881134".to_string(),
            "4846848554".to_string(),
            "5283751526".to_string(),
        ];
        let energy_map = parse_input(lines).expect("could not parse input");

        assert_eq!(part_two(&energy_map), 195);
    }
}
