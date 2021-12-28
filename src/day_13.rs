use std::{
    collections::HashSet, convert::TryFrom, fmt::Display, iter::FromIterator,
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point(u64, u64);

impl From<(u64, u64)> for Point {
    fn from(input: (u64, u64)) -> Self {
        Self(input.0, input.1)
    }
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(",").map(|x| x.parse::<u64>());
        let x = iter
            .next()
            .ok_or_else(|| "expected 2 numbers".to_string())?
            .or_else(|_| Err("could not parse int".to_string()))?;
        let y = iter
            .next()
            .ok_or_else(|| "expected 2 numbers".to_string())?
            .or_else(|_| Err("could not parse int".to_string()))?;

        Ok(Self(x, y))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fold(Axis, u64);

impl FromStr for Fold {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split_whitespace().collect();
        if tokens.len() < 3 {
            return Err("invalid fold input".to_string());
        }

        let (a, b) = tokens[2]
            .split_once("=")
            .ok_or_else(|| "invalid input".to_string())?;

        let axis = match a {
            "x" => Axis::X,
            "y" => Axis::Y,
            _ => return Err("invalid fold input".to_string()),
        };
        let value = b
            .parse::<u64>()
            .or_else(|_| Err("invalid fold input".to_string()))?;

        Ok(Self(axis, value))
    }
}

impl Fold {
    pub fn axis(&self) -> Axis {
        self.0
    }

    pub fn value(&self) -> u64 {
        self.1
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Manual {
    dots: HashSet<Point>,
    folds: Vec<Fold>,
}

impl TryFrom<Vec<String>> for Manual {
    type Error = String;

    fn try_from(lines: Vec<String>) -> Result<Self, Self::Error> {
        let mut parts_iter = lines.split(|line| line.is_empty());

        let part1 = parts_iter
            .next()
            .ok_or_else(|| "invalid input".to_string())?;
        let points = part1
            .iter()
            .map(|x| Point::from_str(x))
            .collect::<Result<Vec<Point>, String>>()?;
        let dots: HashSet<Point> = HashSet::from_iter(points.into_iter());

        let part2 = parts_iter
            .next()
            .ok_or_else(|| "invalid input".to_string())?;
        let folds = part2
            .iter()
            .map(|x| Fold::from_str(x))
            .collect::<Result<Vec<Fold>, String>>()?;

        Ok(Self { dots, folds })
    }
}

impl Manual {
    pub fn multi_fold(&mut self) {
        for i in 0..self.folds.len() {
            self.single_fold(i);
        }
    }

    pub fn single_fold(&mut self, index: usize) {
        let fold = self.folds[index];
        let axis = fold.axis();
        let value = fold.value();

        let mut to_insert: Vec<Point> = Vec::new();
        let mut to_remove: Vec<Point> = Vec::new();

        for point in self.dots.iter() {
            if (axis == Axis::X && point.0 < value) || (axis == Axis::Y && point.1 < value) {
                continue;
            }

            to_remove.push(*point);

            let diff = match axis {
                Axis::X => point.0 - value,
                Axis::Y => point.1 - value,
            };

            let new_point = match axis {
                Axis::X => Point(value - diff, point.1),
                Axis::Y => Point(point.0, value - diff),
            };

            if !self.dots.contains(&new_point) {
                to_insert.push(new_point);
            }
        }

        for point in to_remove {
            self.dots.remove(&point);
        }

        for point in to_insert {
            self.dots.insert(point);
        }
    }

    pub fn num_dots(&self) -> usize {
        self.dots.len()
    }
}

impl Display for Manual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_x = self
            .dots
            .iter()
            .max_by(|a, b| a.0.cmp(&b.0))
            .map(|v| v.0 + 1)
            .unwrap_or(0);
        let max_y = self
            .dots
            .iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .map(|v| v.1 + 1)
            .unwrap_or(0);

        let mut grid = vec![vec!['.'; max_x as usize]; max_y as usize];
        for p in self.dots.iter() {
            grid[p.1 as usize][p.0 as usize] = '█';
        }

        let s = grid
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", s)
    }
}

pub fn parse_input(lines: Vec<String>) -> Result<Manual, String> {
    Manual::try_from(lines)
}

pub fn part_one(manual: &Manual) -> usize {
    let mut clone = manual.clone();
    clone.single_fold(0);
    clone.num_dots()
}

pub fn part_two(manual: &Manual) -> String {
    let mut clone = manual.clone();
    clone.multi_fold();
    clone.to_string()
}

#[cfg(test)]
mod tests {
    use crate::utils::split_lines;

    use super::*;

    #[test]
    fn parse_input_test() {
        let input = "6,10
                     0,14
                     9,10

                     fold along y=7
                     fold along x=5";

        let actual = parse_input(split_lines(input)).expect("could not parse input");
        let expected = Manual {
            dots: HashSet::from_iter(vec![Point(6, 10), Point(0, 14), Point(9, 10)].into_iter()),
            folds: vec![Fold(Axis::Y, 7), Fold(Axis::X, 5)],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn part_one_test() {
        let input = "6,10
                     0,14
                     9,10
                     0,3
                     10,4
                     4,11
                     6,0
                     6,12
                     4,1
                     0,13
                     10,12
                     3,4
                     3,0
                     8,4
                     1,10
                     2,14
                     8,10
                     9,0

                     fold along y=7
                     fold along x=5";
        let manual = parse_input(split_lines(input)).expect("could not parse input");

        assert_eq!(part_one(&manual), 17);
    }
}
