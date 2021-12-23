use std::str::FromStr;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SSDigit {
    segments: String,
}

impl From<&str> for SSDigit {
    fn from(s: &str) -> Self {
        Self {
            segments: s.to_string(),
        }
    }
}

impl SSDigit {
    pub fn has_unique_segment_length(&self) -> bool {
        match self.segments.chars().count() {
            2 | 3 | 4 | 7 => true,
            _ => false,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SSDisplay {
    digits: Vec<SSDigit>,
    outputs: Vec<SSDigit>,
}

impl FromStr for SSDisplay {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (part1, part2) = s
            .split_once(" | ")
            .ok_or_else(|| "invalid input".to_string())?;
        let digits: Vec<SSDigit> = part1.split_whitespace().map(|x| x.into()).collect();
        let outputs: Vec<SSDigit> = part2.split_whitespace().map(|x| x.into()).collect();
        Ok(Self { digits, outputs })
    }
}

impl SSDisplay {
    pub fn num_of_unique_segment_length_outputs(&self) -> usize {
        self.outputs
            .iter()
            .filter(|x| x.has_unique_segment_length())
            .count()
    }
}

pub fn parse_input(lines: Vec<String>) -> Result<Vec<SSDisplay>, String> {
    lines
        .iter()
        .map(|line| SSDisplay::from_str(line))
        .collect::<Result<Vec<SSDisplay>, String>>()
}

pub fn part_one(displays: &Vec<SSDisplay>) -> usize {
    displays
        .iter()
        .map(|x| x.num_of_unique_segment_length_outputs())
        .sum()
}

pub fn part_two(displays: &Vec<SSDisplay>) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ss_digit_has_unique_segment_length_test() {
        let examples = [
            (SSDigit::from("cg"), true),
            (SSDigit::from("cgb"), true),
            (SSDigit::from("gcbe"), true),
            (SSDigit::from("egdcabf"), true),
            (SSDigit::from("a"), false),
            (SSDigit::from("cbdgef"), false),
        ];

        for (ss_digit, expected) in examples {
            assert_eq!(ss_digit.has_unique_segment_length(), expected)
        }
    }

    #[test]
    fn parse_input_test() {
        let lines = vec![
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe".to_string(),
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc".to_string(),
        ];

        let actual = parse_input(lines);
        let expected = vec![
            SSDisplay {
                digits: vec![
                    "be".into(),
                    "cfbegad".into(),
                    "cbdgef".into(),
                    "fgaecd".into(),
                    "cgeb".into(),
                    "fdcge".into(),
                    "agebfd".into(),
                    "fecdb".into(),
                    "fabcd".into(),
                    "edb".into(),
                ],
                outputs: vec![
                    "fdgacbe".into(),
                    "cefdb".into(),
                    "cefbgd".into(),
                    "gcbe".into(),
                ],
            },
            SSDisplay {
                digits: vec![
                    "edbfga".into(),
                    "begcd".into(),
                    "cbg".into(),
                    "gc".into(),
                    "gcadebf".into(),
                    "fbgde".into(),
                    "acbgfd".into(),
                    "abcde".into(),
                    "gfcbed".into(),
                    "gfec".into(),
                ],
                outputs: vec!["fcgedb".into(), "cgb".into(), "dgebacf".into(), "gc".into()],
            },
        ];

        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn part_one_test() {
        let lines = vec![
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe".to_string(),
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc".to_string(),
            "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg".to_string(),
            "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb".to_string(),
            "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea".to_string(),
            "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb".to_string(),
            "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe".to_string(),
            "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef".to_string(),
            "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb".to_string(),
            "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce".to_string(),
        ];

        let displays = parse_input(lines).expect("could not parse input");

        assert_eq!(part_one(&displays), 26);
    }
}
