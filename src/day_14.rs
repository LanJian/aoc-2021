use std::{convert::TryFrom, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Element {
    id: u64,
}

impl From<char> for Element {
    fn from(c: char) -> Self {
        let id = c as u64 - 'A' as u64 + 1;
        Self { id }
    }
}

impl FromStr for Element {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err("expected element to be 1 char".to_string());
        }

        let c = s
            .chars()
            .next()
            .ok_or_else(|| "expected element to be 1 char".to_string())?;

        Ok(Self::from(c))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rule {
    id: u64,
    key: (Element, Element),
    value: Element,
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (part1, part2) = s
            .split_once(" -> ")
            .ok_or_else(|| "invalid rule input".to_string())?;

        let mut iter = part1.chars();
        let key: (Element, Element) = (
            iter.next()
                .ok_or_else(|| "invalid rule input".to_string())?
                .into(),
            iter.next()
                .ok_or_else(|| "invalid rule input".to_string())?
                .into(),
        );
        let value = Element::from_str(part2)?;
        let id = Self::make_id(key.0, key.1);

        Ok(Self { id, key, value })
    }
}

impl Rule {
    pub fn make_id(a: Element, b: Element) -> u64 {
        a.id * 100 + b.id
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Polymer {
    template: Vec<Element>,
    rules: Vec<Option<Rule>>,
}

impl TryFrom<Vec<String>> for Polymer {
    type Error = String;

    fn try_from(lines: Vec<String>) -> Result<Self, Self::Error> {
        let mut iter = lines.iter();
        let part1 = iter
            .next()
            .ok_or_else(|| "expected polymer template".to_string())?;
        let template: Vec<Element> = part1.chars().map(|c| Element::from(c)).collect();

        // empty line
        iter.next().ok_or_else(|| "invalid input".to_string())?;

        let mut rules: Vec<Option<Rule>> = vec![None; 2727];
        for s in iter {
            let rule = Rule::from_str(s)?;
            rules[rule.id as usize] = Some(rule);
        }

        Ok(Self { template, rules })
    }
}

impl Polymer {
    pub fn min_max_after_steps(&self, steps: usize) -> (usize, usize) {
        let len = self.template.len();
        let mut quantities: [usize; 27] = [0; 27];
        let mut mem: Vec<Vec<Option<[usize; 27]>>> = vec![vec![None; steps + 1]; 2727];

        for i in 0..len {
            let left = self.template.get(i).and_then(|e| Some(*e));
            let right = self.template.get(i + 1).and_then(|e| Some(*e));
            let result = self.count_between(left, right, 0, steps, &mut mem);
            for i in 0..result.len() {
                quantities[i] += result[i];
            }
        }

        let filtered: Vec<usize> = quantities.iter().filter(|x| **x != 0).map(|x| *x).collect();
        let min = *filtered.iter().min().unwrap_or(&0);
        let max = *filtered.iter().max().unwrap_or(&0);
        (min, max)
    }

    /// Counts the quantities of the elements between 2 elements and updates the provided
    /// quantities vector, applying rules from the rule set until max depth is reached. The
    /// returned vector includes the left input element, and excludes the right input element
    fn count_between(
        &self,
        left: Option<Element>,
        right: Option<Element>,
        depth: usize,
        max_depth: usize,
        mem: &mut Vec<Vec<Option<[usize; 27]>>>,
    ) -> [usize; 27] {
        let mut ret = [0; 27];

        match (left, right, depth == max_depth) {
            (None, None, _) => (),
            (Some(e), None, _) | (Some(e), _, true) => {
                ret[e.id as usize] += 1;
            }
            (Some(a), Some(b), false) => {
                let index = Rule::make_id(a, b) as usize;
                if let Some(mem_val) = mem[index][depth] {
                    return mem_val;
                }

                let mid = self.rules[index].and_then(|r| Some(r.value));
                let left_result = self.count_between(left, mid, depth + 1, max_depth, mem);
                let right_result = self.count_between(mid, right, depth + 1, max_depth, mem);
                for i in 0..left_result.len() {
                    ret[i] += left_result[i];
                }
                for i in 0..right_result.len() {
                    ret[i] += right_result[i];
                }

                mem[index][depth] = Some(ret);
            }
            _ => unreachable!(),
        }

        ret
    }
}

pub fn parse_input(lines: Vec<String>) -> Result<Polymer, String> {
    Polymer::try_from(lines)
}

pub fn part_one(polymer: &Polymer) -> usize {
    let (min, max) = polymer.min_max_after_steps(10);
    max - min
}

pub fn part_two(polymer: &Polymer) -> usize {
    let (min, max) = polymer.min_max_after_steps(40);
    max - min
}

#[cfg(test)]
mod tests {
    use crate::utils::split_lines;

    use super::*;

    #[test]
    fn elemet_from_char_test() {
        assert_eq!(Element::from('N'), Element { id: 14 });
        assert_eq!(Element::from('P'), Element { id: 16 });
        assert_eq!(Element::from('A'), Element { id: 1 });
        assert_eq!(Element::from('Z'), Element { id: 26 });
    }

    #[test]
    fn parse_input_test() {
        let input = "NNCB

                    CH -> B
                    HH -> N
                    CN -> C";

        let actual = parse_input(split_lines(input)).expect("could not parse input");
        let mut expected_rules: Vec<Option<Rule>> = vec![None; 2727];
        expected_rules[308] = Some(Rule {
            id: 308,
            key: ('C'.into(), 'H'.into()),
            value: 'B'.into(),
        });
        expected_rules[808] = Some(Rule {
            id: 808,
            key: ('H'.into(), 'H'.into()),
            value: 'N'.into(),
        });
        expected_rules[314] = Some(Rule {
            id: 314,
            key: ('C'.into(), 'N'.into()),
            value: 'C'.into(),
        });
        let expected = Polymer {
            template: vec!['N'.into(), 'N'.into(), 'C'.into(), 'B'.into()],
            rules: expected_rules,
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn part_one_test() {
        let input = "NNCB

                    CH -> B
                    HH -> N
                    CB -> H
                    NH -> C
                    HB -> C
                    HC -> B
                    HN -> C
                    NN -> C
                    BH -> H
                    NC -> B
                    NB -> B
                    BN -> B
                    BB -> N
                    BC -> B
                    CC -> N
                    CN -> C";

        let polymer = parse_input(split_lines(input)).expect("could not parse input");
        assert_eq!(part_one(&polymer), 1588);
    }

    #[test]
    fn part_two_test() {
        let input = "NNCB

                    CH -> B
                    HH -> N
                    CB -> H
                    NH -> C
                    HB -> C
                    HC -> B
                    HN -> C
                    NN -> C
                    BH -> H
                    NC -> B
                    NB -> B
                    BN -> B
                    BB -> N
                    BC -> B
                    CC -> N
                    CN -> C";

        let polymer = parse_input(split_lines(input)).expect("could not parse input");
        assert_eq!(part_two(&polymer), 2188189693529);
    }
}
