use std::{num::ParseIntError, str::FromStr};

pub enum FuelCost {
    Constant,
    Linear,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct CrabSubmarine {
    x: i64,
}

impl FromStr for CrabSubmarine {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.parse::<i64>()?.into())
    }
}

impl From<i64> for CrabSubmarine {
    fn from(value: i64) -> Self {
        CrabSubmarine { x: value }
    }
}

impl CrabSubmarine {
    pub fn fuel_cost_to(&self, x: i64, fuel_cost: FuelCost) -> i64 {
        match fuel_cost {
            FuelCost::Constant => (self.x - x).abs(),
            FuelCost::Linear => {
                let steps = (self.x - x).abs();
                (1 + steps) * steps / 2
            }
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct CrabFleet {
    fleet: Vec<CrabSubmarine>,
}

impl FromStr for CrabFleet {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.split(",");
        let fleet = tokens
            .into_iter()
            .map(|x| CrabSubmarine::from_str(x))
            .collect::<Result<Vec<CrabSubmarine>, ParseIntError>>()?;

        Ok(Self { fleet })
    }
}

impl CrabFleet {
    /// Returns the minimum amount of fuel needed to align all the crab submarines to the same
    /// x position
    pub fn align(&self, fuel_cost_type: FuelCost) -> i64 {
        match fuel_cost_type {
            FuelCost::Constant => self.align_constant_fuel_cost(),
            FuelCost::Linear => self.align_linear_fuel_cost(),
        }
    }

    fn align_constant_fuel_cost(&self) -> i64 {
        let mut sorted = self.fleet.clone();
        sorted.sort();
        let min_x = sorted[0].x;
        let len = sorted.len();

        let mut acc: i64 = sorted.iter().map(|sub| sub.x - min_x).sum();
        let mut ret = acc;

        for i in 1..len {
            let diff = sorted[i].x - sorted[i - 1].x;
            let left_count = i;
            let right_count = len - left_count;
            acc += diff * left_count as i64;
            acc -= diff * right_count as i64;
            ret = ret.min(acc);
        }

        ret
    }

    fn align_linear_fuel_cost(&self) -> i64 {
        let min_x = match self.fleet.iter().min() {
            Some(val) => val.x,
            None => return 0,
        };

        let max_x = match self.fleet.iter().max() {
            Some(val) => val.x,
            None => return 0,
        };

        (min_x..=max_x)
            .map(|i| {
                self.fleet
                    .iter()
                    .map(|sub| sub.fuel_cost_to(i, FuelCost::Linear))
                    .sum()
            })
            .min()
            .unwrap_or(0)
    }
}

pub fn parse_input(lines: Vec<String>) -> Result<CrabFleet, String> {
    let line = &lines[0];
    CrabFleet::from_str(line).or_else(|_| Err("could not parse input".to_string()))
}

pub fn part_one(fleet: &CrabFleet) -> i64 {
    fleet.align(FuelCost::Constant)
}

pub fn part_two(fleet: &CrabFleet) -> i64 {
    fleet.align(FuelCost::Linear)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crab_submarine_from_str_test() {
        let input = "3";

        let actual = CrabSubmarine::from_str(input);
        let expected = CrabSubmarine { x: 3 };

        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn crab_submarine_from_i64_test() {
        let input = 3;

        let actual = CrabSubmarine::from(input);
        let expected = CrabSubmarine { x: 3 };

        assert_eq!(actual, expected);
    }

    #[test]
    fn crab_submarine_sort_test() {
        let fleet: Vec<CrabSubmarine> = vec![1.into(), 5.into(), 6.into(), 2.into(), 2.into()];
        let mut sorted = fleet.clone();
        sorted.sort();
        assert_eq!(
            sorted,
            vec![1.into(), 2.into(), 2.into(), 5.into(), 6.into()]
        );
    }

    #[test]
    fn crab_submarine_fuel_cost_to_test() {
        let mut sub = CrabSubmarine::from(3);
        assert_eq!(sub.fuel_cost_to(0, FuelCost::Constant), 3);
        assert_eq!(sub.fuel_cost_to(3, FuelCost::Constant), 0);
        assert_eq!(sub.fuel_cost_to(19, FuelCost::Constant), 16);
        assert_eq!(sub.fuel_cost_to(0, FuelCost::Linear), 6);
        assert_eq!(sub.fuel_cost_to(3, FuelCost::Linear), 0);
        assert_eq!(sub.fuel_cost_to(19, FuelCost::Linear), 136);

        sub = CrabSubmarine::from(16);
        assert_eq!(sub.fuel_cost_to(5, FuelCost::Linear), 66);

        sub = CrabSubmarine::from(1);
        assert_eq!(sub.fuel_cost_to(5, FuelCost::Linear), 10);
    }

    #[test]
    fn crab_fleet_from_str_test() {
        let input = "1,5,6,2,3";

        let actual = CrabFleet::from_str(input);
        let expected = CrabFleet {
            fleet: vec![1.into(), 5.into(), 6.into(), 2.into(), 3.into()],
        };

        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn crab_fleet_from_str_invalid_input_test() {
        let input = "1,foo,6,2,3";

        let actual = CrabFleet::from_str(input);
        assert!(actual.is_err());
    }

    #[test]
    fn crab_fleet_align_test() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        let fleet = CrabFleet::from_str(input).expect("could not parse input");

        assert_eq!(fleet.align(FuelCost::Constant), 37);
        assert_eq!(fleet.align(FuelCost::Linear), 168);
    }

    #[test]
    fn parse_input_test() {
        let input = vec!["1,5,6,2,3".to_string()];

        let actual = parse_input(input);
        let expected = CrabFleet {
            fleet: vec![1.into(), 5.into(), 6.into(), 2.into(), 3.into()],
        };

        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn parse_input_invalid_input_test() {
        let input = vec!["1,foo,6,2,3".to_string()];

        let actual = parse_input(input);
        assert!(actual.is_err());
    }
}
