use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
    str::FromStr,
};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Segment {
    letter: char,
    possible_positions: HashSet<usize>,
}

impl From<char> for Segment {
    fn from(c: char) -> Self {
        Self {
            letter: c,
            possible_positions: HashSet::from_iter(0..7),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SSDigit {
    segments: Vec<Segment>,
    sorted_string: String,
}

impl From<&str> for SSDigit {
    fn from(s: &str) -> Self {
        let segments = s.chars().map(|x| Segment::from(x)).collect();
        let mut chars = Vec::from_iter(s.chars());
        chars.sort();
        let sorted_string = chars.into_iter().collect();
        Self {
            segments,
            sorted_string,
        }
    }
}

impl SSDigit {
    pub fn has_unique_segment_length(&self) -> bool {
        match self.segments.len() {
            2 | 3 | 4 | 7 => true,
            _ => false,
        }
    }

    pub fn segment_length(&self) -> usize {
        self.segments.len()
    }

    pub fn has_letter(&self, c: char) -> bool {
        self.segments.iter().any(|x| x.letter == c)
    }

    pub fn sorted_string(&self) -> String {
        self.sorted_string.clone()
    }
}

type Arrangement = [Option<char>; 7];
type Mapping = HashMap<String, usize>;

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

    pub fn deduce(&self) -> Result<Mapping, String> {
        let mut display_segments: Vec<Segment> = Vec::new();
        for c in 'a'..='g' {
            display_segments.push(Segment::from(c));
        }

        let one = self
            .digits
            .iter()
            .find(|x| x.segment_length() == 2)
            .ok_or_else(|| "expected digit with segment length 2")?;
        let seven = self
            .digits
            .iter()
            .find(|x| x.segment_length() == 3)
            .ok_or_else(|| "expected digit with segment length 3")?;
        let four = self
            .digits
            .iter()
            .find(|x| x.segment_length() == 4)
            .ok_or_else(|| "expected digit with segment length 4")?;

        // one
        for display_segment in &mut display_segments {
            if one.has_letter(display_segment.letter) {
                display_segment.possible_positions.remove(&0);
                display_segment.possible_positions.remove(&1);
                display_segment.possible_positions.remove(&4);
                display_segment.possible_positions.remove(&5);
                display_segment.possible_positions.remove(&6);
            } else {
                display_segment.possible_positions.remove(&2);
                display_segment.possible_positions.remove(&3);
            }
        }

        // seven
        for display_segment in &mut display_segments {
            if seven.has_letter(display_segment.letter) {
                display_segment.possible_positions.remove(&0);
                display_segment.possible_positions.remove(&4);
                display_segment.possible_positions.remove(&5);
                display_segment.possible_positions.remove(&6);
            } else {
                display_segment.possible_positions.remove(&1);
                display_segment.possible_positions.remove(&2);
                display_segment.possible_positions.remove(&3);
            }
        }

        // four
        for display_segment in &mut display_segments {
            if four.has_letter(display_segment.letter) {
                display_segment.possible_positions.remove(&1);
                display_segment.possible_positions.remove(&4);
                display_segment.possible_positions.remove(&5);
            } else {
                display_segment.possible_positions.remove(&0);
                display_segment.possible_positions.remove(&2);
                display_segment.possible_positions.remove(&3);
                display_segment.possible_positions.remove(&6);
            }
        }

        for i in 0..8 {
            let mut acc = i;
            let mut arrangement = Arrangement::default();

            for segment in &display_segments {
                let mut positions = Vec::from_iter(&segment.possible_positions);

                if segment.possible_positions.len() == 1 {
                    let position = *positions[0];
                    arrangement[position] = Some(segment.letter);
                    continue;
                }

                positions.sort();
                if arrangement[*positions[0]].is_none() && arrangement[*positions[1]].is_none() {
                    let index = acc & 1;
                    let position = *positions[index];
                    arrangement[position] = Some(segment.letter);
                    acc = acc >> 1;
                    continue;
                }

                if arrangement[*positions[0]].is_none() {
                    arrangement[*positions[0]] = Some(segment.letter);
                    continue;
                }

                arrangement[*positions[1]] = Some(segment.letter);
            }

            let mapping = self.try_create_mapping(&arrangement);
            if mapping.is_some() {
                return Ok(mapping.unwrap());
            }
        }

        Err("could not find an arrangement that satisfied".to_string())
    }

    fn try_create_mapping(&self, arrangement: &Arrangement) -> Option<Mapping> {
        let expected_positions = [
            vec![0, 1, 2, 3, 4, 5],
            vec![2, 3],
            vec![1, 2, 6, 5, 4],
            vec![1, 2, 6, 3, 4],
            vec![0, 6, 2, 3],
            vec![1, 0, 6, 3, 4],
            vec![1, 0, 6, 3, 4, 5],
            vec![1, 2, 3],
            vec![0, 1, 2, 3, 4, 5, 6],
            vec![0, 1, 2, 6, 3, 4],
        ];
        let mut mapping: HashMap<String, usize> = HashMap::new();

        for (num, expected_position) in expected_positions.iter().enumerate() {
            let result: Result<Vec<char>, &str> = expected_position
                .iter()
                .map(|x| {
                    arrangement[*x]
                        .ok_or_else(|| "expected letter to exist in arrangement, got none")
                })
                .collect();

            if result.is_err() {
                return None;
            }

            let mut chars = result.unwrap();
            chars.sort();
            let expected_string: String = chars.into_iter().collect();

            if self
                .digits
                .iter()
                .find(|x| x.sorted_string() == expected_string)
                .is_none()
            {
                return None;
            }

            mapping.insert(expected_string, num);
        }

        Some(mapping)
    }

    pub fn calculate_output(&self, mapping: &Mapping) -> usize {
        let mut acc = 0;
        for ss_digit in &self.outputs {
            let digit = mapping
                .get(&ss_digit.sorted_string())
                .expect("expected digit string to exist in mapping");
            acc = acc * 10 + digit;
        }
        acc
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
    let mut sum = 0;
    for display in displays {
        let mapping = display.deduce().expect("could not deduce arrangement");
        let output_number = display.calculate_output(&mapping);
        sum += output_number;
    }

    return sum;
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

    #[test]
    fn part_two_test_1() {
        let lines = vec![
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"
                .to_string(),
        ];

        let displays = parse_input(lines).expect("could not parse input");

        assert_eq!(part_two(&displays), 5353);
    }

    #[test]
    fn part_two_test_2() {
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

        assert_eq!(part_two(&displays), 61229);
    }
}
