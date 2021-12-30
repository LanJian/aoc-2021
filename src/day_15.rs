use std::{collections::BinaryHeap, convert::TryFrom};

use crate::grid::{Coordinate, Grid};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Risk(usize);

impl TryFrom<char> for Risk {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c.to_digit(10) {
            Some(digit) => Ok(Risk(digit as usize)),
            None => Err("could not parse digit".to_string()),
        }
    }
}

impl From<usize> for Risk {
    fn from(val: usize) -> Self {
        Self(val)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct State {
    cost: usize,
    coord: Coordinate,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl State {
    pub fn new(cost: usize, coord: Coordinate) -> Self {
        Self { cost, coord }
    }
}

pub struct Cavern {
    grid: Grid<Risk>,
}

impl TryFrom<Vec<String>> for Cavern {
    type Error = String;

    fn try_from(lines: Vec<String>) -> Result<Self, Self::Error> {
        Ok(Self {
            grid: Grid::try_from(lines)?,
        })
    }
}

impl Cavern {
    /// Returns the total risk of the path with the lowest risk
    pub fn lowest_risk(&self, target: Coordinate) -> Option<usize> {
        let start: Coordinate = (0_i64, 0_i64).into();
        let mut costs: Grid<usize> = Grid::new(self.grid.n * 5, self.grid.m * 5, usize::MAX);
        let mut q: BinaryHeap<State> = BinaryHeap::new();

        costs[start] = 0;
        q.push(State::new(0, start));

        while let Some(State { cost, coord }) = q.pop() {
            // if we are visiting the target cell, then this must be the shortest path
            if coord == target {
                return Some(cost);
            }

            // if the current cost is more than the recorded lowest cost, then prune this path
            if cost > costs[coord] {
                continue;
            }

            for neighbour in coord.cardinal_neighbours() {
                // skip if neighbour is not in bounds
                if !self.is_in_bounds(neighbour) {
                    continue;
                }

                let next_state = State::new(cost + self.get(neighbour).0, neighbour);
                if next_state.cost < costs[neighbour] {
                    q.push(next_state);
                    costs[neighbour] = next_state.cost;
                }
            }
        }

        None
    }

    pub fn is_in_bounds(&self, coord: Coordinate) -> bool {
        (0..(self.grid.n * 5) as i64).contains(&coord.row())
            && (0..(self.grid.m * 5) as i64).contains(&coord.col())
    }

    pub fn get(&self, coord: Coordinate) -> Risk {
        let row = coord.row() as usize;
        let col = coord.col() as usize;
        let row_quotient = row / self.grid.n;
        let col_quotient = col / self.grid.m;
        let row_remainder = row % self.grid.n;
        let col_remainder = col % self.grid.m;
        let new_coord = Coordinate::from((row_remainder, col_remainder));
        let risk = self.grid[new_coord];
        let new_risk = Risk((risk.0 - 1 + row_quotient + col_quotient) % 9 + 1);
        new_risk
    }

    pub fn southeast_corner(&self) -> Coordinate {
        (self.grid.n - 1, self.grid.m - 1).into()
    }
}

pub fn parse_input(lines: Vec<String>) -> Result<Cavern, String> {
    Cavern::try_from(lines)
}

pub fn part_one(cavern: &Cavern) -> usize {
    cavern
        .lowest_risk(cavern.southeast_corner())
        .expect("could not find a path")
}

pub fn part_two(cavern: &Cavern) -> usize {
    let target = Coordinate::from((cavern.grid.n * 5 - 1, cavern.grid.m * 5 - 1));
    cavern.lowest_risk(target).expect("could not find a path")
}

#[cfg(test)]
mod tests {
    use crate::utils::split_lines;

    use super::*;

    #[test]
    fn cavern_get_test() {
        let input = "1163751742
                     1381373672
                     2136511328
                     3694931569
                     7463417111
                     1319128137
                     1359912421
                     3125421639
                     1293138521
                     2311944581";
        let cavern = parse_input(split_lines(input)).expect("could not parse input");

        assert_eq!(cavern.get((0_usize, 1_usize).into()), Risk(1));
        assert_eq!(cavern.get((10_usize, 1_usize).into()), Risk(2));
        assert_eq!(cavern.get((10_usize, 11_usize).into()), Risk(3));
        assert_eq!(cavern.get((1_usize, 2_usize).into()), Risk(8));
        assert_eq!(cavern.get((1_usize, 12_usize).into()), Risk(9));
        assert_eq!(cavern.get((1_usize, 22_usize).into()), Risk(1));
    }

    #[test]
    fn part_one_test() {
        let input = "1163751742
                     1381373672
                     2136511328
                     3694931569
                     7463417111
                     1319128137
                     1359912421
                     3125421639
                     1293138521
                     2311944581";
        let cavern = parse_input(split_lines(input)).expect("could not parse input");

        assert_eq!(part_one(&cavern), 40);
    }

    #[test]
    fn part_two_test() {
        let input = "1163751742
                     1381373672
                     2136511328
                     3694931569
                     7463417111
                     1319128137
                     1359912421
                     3125421639
                     1293138521
                     2311944581";
        let cavern = parse_input(split_lines(input)).expect("could not parse input");

        assert_eq!(part_two(&cavern), 315);
    }
}
