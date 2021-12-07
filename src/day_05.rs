use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
    num::ParseIntError,
};

#[derive(Debug, Copy, Default, Clone, PartialEq, Eq, Hash)]
pub struct IntegerPoint {
    x: i64,
    y: i64,
}

impl From<(i64, i64)> for IntegerPoint {
    fn from(coords: (i64, i64)) -> Self {
        IntegerPoint {
            x: coords.0,
            y: coords.1,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VentOrientation {
    Horizontal,
    Vertical,
    Diagonal,
}

/// A vent consists of a starting point and end point. Both points are integer points. Vents are
/// either horizontal or vertical, there are no diagonal vents.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vent {
    start: IntegerPoint,
    end: IntegerPoint,
    orientation: VentOrientation,
}

impl TryFrom<String> for Vent {
    type Error = String;

    fn try_from(line: String) -> Result<Self, Self::Error> {
        let mut parts = line.split(" -> ");
        let mut points: Vec<IntegerPoint> = Vec::new();
        for _ in 0..2 {
            let values = parts
                .next()
                .ok_or("invalid input")?
                .split(",")
                .map(|x| x.parse::<i64>())
                .collect::<Result<Vec<i64>, ParseIntError>>()
                .or(Err("invalid input"))?;

            if values.len() != 2 {
                return Err("invalid input".into());
            }

            points.push((values[0], values[1]).into());
        }

        let start = points[0];
        let end = points[1];
        let orientation = if start.x == end.x {
            VentOrientation::Vertical
        } else if start.y == end.y {
            VentOrientation::Horizontal
        } else {
            VentOrientation::Diagonal
        };

        Ok(Vent {
            start,
            end,
            orientation,
        })
    }
}

impl Vent {
    /// Returns a vector of all integer points that the vent covers
    pub fn integer_points(&self) -> Vec<IntegerPoint> {
        let s = self.start;
        let e = self.end;

        if self.orientation == VentOrientation::Vertical {
            // vertical
            let min = s.y.min(e.y);
            let max = s.y.max(e.y);
            (min..(max + 1))
                .map(|val| (s.x, val).into())
                .collect::<Vec<IntegerPoint>>()
        } else if self.orientation == VentOrientation::Horizontal {
            // horizontal
            let min = s.x.min(e.x);
            let max = s.x.max(e.x);
            (min..(max + 1))
                .map(|val| (val, s.y).into())
                .collect::<Vec<IntegerPoint>>()
        } else {
            let (min, max) = if s.x < e.x { (s, e) } else { (e, s) };
            let x_iter = min.x..(max.x + 1);
            let y_iter: Box<dyn Iterator<Item = _>> = if min.y < max.y {
                Box::new(min.y..(max.y + 1))
            } else {
                Box::new((max.y..(min.y + 1)).rev())
            };

            x_iter
                .zip(y_iter)
                .map(|(x, y)| (x, y).into())
                .collect::<Vec<IntegerPoint>>()
        }
    }
}

pub fn parse_input(lines: Vec<String>) -> Result<Vec<Vent>, String> {
    lines
        .into_iter()
        .map(|line| line.try_into())
        .collect::<Result<Vec<Vent>, String>>()
}

/// Returns the number of points where at least 2 vent lines overlap. Only consider horizontal
/// or vertical vents
pub fn part_one(vents: &Vec<Vent>) -> usize {
    let mut frequencies: HashMap<IntegerPoint, usize> = HashMap::new();
    for vent in vents
        .iter()
        .filter(|x| x.orientation != VentOrientation::Diagonal)
    {
        for point in vent.integer_points() {
            let entry = frequencies.entry(point).or_insert(0);
            *entry += 1
        }
    }

    frequencies.into_iter().filter(|(_, val)| *val > 1).count()
}

/// Returns the number of points where at least 2 vent lines overlap. Considers all vents
pub fn part_two(vents: &Vec<Vent>) -> usize {
    let mut frequencies: HashMap<IntegerPoint, usize> = HashMap::new();
    for vent in vents {
        for point in vent.integer_points() {
            let entry = frequencies.entry(point).or_insert(0);
            *entry += 1
        }
    }

    frequencies.into_iter().filter(|(_, val)| *val > 1).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vent_try_from_test() {
        let input = "1,2 -> 5,2".to_string();

        let actual = Vent::try_from(input);
        let expected = Vent {
            start: (1, 2).into(),
            end: (5, 2).into(),
            orientation: VentOrientation::Horizontal,
        };

        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn vent_try_from_invalid_input_test() {
        let inputs = [
            "1,2, 3,4".to_string(),
            "1 -> 3,4".to_string(),
            "".to_string(),
            " -> ".to_string(),
            "2,3 -> ".to_string(),
            "2,3 -> ".to_string(),
        ];

        for input in inputs {
            let actual = Vent::try_from(input);
            assert!(actual.is_err());
        }
    }

    #[test]
    fn vent_integer_points_test() {
        // horizontal
        let mut vent = Vent {
            start: (1, 2).into(),
            end: (4, 2).into(),
            orientation: VentOrientation::Horizontal,
        };

        let mut actual = vent.integer_points();
        let mut expected: Vec<IntegerPoint> =
            vec![(1, 2).into(), (2, 2).into(), (3, 2).into(), (4, 2).into()];

        assert_eq!(actual, expected);

        // vertical
        vent = Vent {
            start: (1, 2).into(),
            end: (1, 5).into(),
            orientation: VentOrientation::Vertical,
        };

        actual = vent.integer_points();
        expected = vec![(1, 2).into(), (1, 3).into(), (1, 4).into(), (1, 5).into()];

        assert_eq!(actual, expected);

        // horizontal reversed
        vent = Vent {
            start: (9, 2).into(),
            end: (6, 2).into(),
            orientation: VentOrientation::Horizontal,
        };

        actual = vent.integer_points();
        expected = vec![(6, 2).into(), (7, 2).into(), (8, 2).into(), (9, 2).into()];

        assert_eq!(actual, expected);

        // diagonal in top-right direction
        vent = Vent {
            start: (2, 3).into(),
            end: (6, 7).into(),
            orientation: VentOrientation::Diagonal,
        };

        actual = vent.integer_points();
        expected = vec![
            (2, 3).into(),
            (3, 4).into(),
            (4, 5).into(),
            (5, 6).into(),
            (6, 7).into(),
        ];

        assert_eq!(actual, expected);

        // diagonal in bottom-left direction
        vent = Vent {
            start: (5, 8).into(),
            end: (3, 6).into(),
            orientation: VentOrientation::Diagonal,
        };

        actual = vent.integer_points();
        expected = vec![(3, 6).into(), (4, 7).into(), (5, 8).into()];

        assert_eq!(actual, expected);

        // diagonal in bottom-right direction
        vent = Vent {
            start: (1, 8).into(),
            end: (3, 6).into(),
            orientation: VentOrientation::Diagonal,
        };

        actual = vent.integer_points();
        expected = vec![(1, 8).into(), (2, 7).into(), (3, 6).into()];

        assert_eq!(actual, expected);

        // diagonal in top-left direction
        vent = Vent {
            start: (7, 0).into(),
            end: (4, 3).into(),
            orientation: VentOrientation::Diagonal,
        };

        actual = vent.integer_points();
        expected = vec![(4, 3).into(), (5, 2).into(), (6, 1).into(), (7, 0).into()];

        assert_eq!(actual, expected);
    }

    #[test]
    fn part_one_test() {
        let input = "0,9 -> 5,9\n\
                     8,0 -> 0,8\n\
                     9,4 -> 3,4\n\
                     2,2 -> 2,1\n\
                     7,0 -> 7,4\n\
                     6,4 -> 2,0\n\
                     0,9 -> 2,9\n\
                     3,4 -> 1,4\n\
                     0,0 -> 8,8\n\
                     5,5 -> 8,2";

        let lines: Vec<String> = input.split("\n").map(|x| x.to_string()).collect();
        let vents = parse_input(lines).expect("could not parse input");

        let actual = part_one(&vents);
        let expected = 5;

        assert_eq!(actual, expected);
    }

    #[test]
    fn part_two_test() {
        let input = "0,9 -> 5,9\n\
                     8,0 -> 0,8\n\
                     9,4 -> 3,4\n\
                     2,2 -> 2,1\n\
                     7,0 -> 7,4\n\
                     6,4 -> 2,0\n\
                     0,9 -> 2,9\n\
                     3,4 -> 1,4\n\
                     0,0 -> 8,8\n\
                     5,5 -> 8,2";

        let lines: Vec<String> = input.split("\n").map(|x| x.to_string()).collect();
        let vents = parse_input(lines).expect("could not parse input");

        let actual = part_two(&vents);
        let expected = 12;

        assert_eq!(actual, expected);
    }
}
