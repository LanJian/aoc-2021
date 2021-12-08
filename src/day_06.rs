use std::{collections::HashMap, num::ParseIntError, str::FromStr};

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Lanternfish {
    timer: i64,
}

impl FromStr for Lanternfish {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.parse::<i64>()?.into())
    }
}

impl From<i64> for Lanternfish {
    fn from(value: i64) -> Self {
        Lanternfish { timer: value }
    }
}

impl Lanternfish {
    pub const CYCLE_LENGTH: i64 = 7;
    pub const NEWBORN_TIMER: i64 = 8;

    /// Simulates the life cycle of the lanternfish for the given number of days. Returns the
    /// total number of lanternfish at the end of the period.
    pub fn simulate(&self, days: i64, mem: &mut HashMap<(Lanternfish, i64), i64>) -> i64 {
        if let Some(val) = mem.get(&(*self, days)) {
            return *val;
        }

        let mut ret = 1;
        let mut days_remaining = days - self.timer - 1;

        while days_remaining >= 0 {
            ret += Lanternfish::from(Self::NEWBORN_TIMER).simulate(days_remaining, mem);
            days_remaining -= Self::CYCLE_LENGTH;
        }

        mem.insert((*self, days), ret);
        ret
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct School {
    fish: Vec<Lanternfish>,
}

impl FromStr for School {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.split(",");
        let fish = tokens
            .into_iter()
            .map(|x| Lanternfish::from_str(x))
            .collect::<Result<Vec<Lanternfish>, ParseIntError>>()?;

        Ok(School { fish })
    }
}

impl School {
    /// Simulate the life cycle for the school of lanternfish for the given number of days. Returns
    /// the number of fish remaining at the end.
    pub fn simulate(&self, days: i64) -> i64 {
        let mut mem: HashMap<(Lanternfish, i64), i64> = HashMap::new();

        self.fish.iter().map(|f| f.simulate(days, &mut mem)).sum()
    }
}

pub fn parse_input(lines: Vec<String>) -> Result<School, String> {
    let line = &lines[0];
    School::from_str(line).or(Err("could not parse input".to_string()))
}

pub fn part_one(school: &School) -> i64 {
    school.simulate(80)
}

pub fn part_two(school: &School) -> i64 {
    school.simulate(256)
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn lanternfish_from_str_test() {
        let input = "3";

        let actual = Lanternfish::from_str(input);
        let expected = Lanternfish::from(3);

        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn lanternfish_try_from_invalid_input_test() {
        let inputs = ["", "foo", "3.4"];

        for input in inputs {
            let actual = Lanternfish::from_str(input);
            assert!(actual.is_err());
        }
    }

    #[test]
    fn lanternfish_simulate_test() {
        let mut fish = Lanternfish::from(3);
        let mut days = 18;
        let mut mem: HashMap<(Lanternfish, i64), i64> = HashMap::new();

        let mut actual = fish.simulate(days, &mut mem);
        let mut expected = 5;

        assert_eq!(actual, expected);

        fish = Lanternfish::from(3);
        days = 5;
        mem.clear();

        actual = fish.simulate(days, &mut mem);
        expected = 2;

        assert_eq!(actual, expected);

        fish = Lanternfish::from(5);
        days = 5;
        mem.clear();

        actual = fish.simulate(days, &mut mem);
        expected = 1;

        assert_eq!(actual, expected);

        fish = Lanternfish::from(3);
        days = 1;
        mem.clear();

        actual = fish.simulate(days, &mut mem);
        expected = 1;

        assert_eq!(actual, expected);

        fish = Lanternfish::from(0);
        days = 21;
        mem.clear();

        actual = fish.simulate(days, &mut mem);
        expected = 8;

        assert_eq!(actual, expected);

        fish = Lanternfish::from(6);
        days = 21;
        mem.clear();

        actual = fish.simulate(days, &mut mem);
        expected = 5;

        assert_eq!(actual, expected);
    }

    #[test]
    fn school_simulate_test() {
        let school = School::from_str("3,4,3,1,2").expect("could not parse input");
        assert_eq!(school.simulate(18), 26);
        assert_eq!(school.simulate(80), 5934);
        assert_eq!(school.simulate(256), 26984457539);
    }

    #[test]
    fn parse_input_test() {
        let lines = vec!["1,5,6,2,3".to_string()];

        let actual = parse_input(lines);
        let expected = School {
            fish: vec![1.into(), 5.into(), 6.into(), 2.into(), 3.into()],
        };

        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn parse_input_invalid_input_test() {
        let lines = vec!["1,5,6,foo,3".to_string()];

        let actual = parse_input(lines);

        assert!(actual.is_err());
    }
}
