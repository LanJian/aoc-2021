use std::{convert::TryFrom, str::FromStr};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ChunkToken {
    chunk_type: ChunkType,
    chunk_op: ChunkOp,
}

impl TryFrom<char> for ChunkToken {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        let chunk_type = match c {
            '(' | ')' => ChunkType::Round,
            '[' | ']' => ChunkType::Square,
            '{' | '}' => ChunkType::Curly,
            '<' | '>' => ChunkType::Angle,
            _ => return Err("invalid token".to_string()),
        };

        let chunk_op = match c {
            '(' | '[' | '{' | '<' => ChunkOp::Open,
            ')' | ']' | '}' | '>' => ChunkOp::Close,
            _ => return Err("invalid token".to_string()),
        };

        Ok(Self {
            chunk_type,
            chunk_op,
        })
    }
}

impl ChunkToken {
    pub fn is_open(&self) -> bool {
        self.chunk_op == ChunkOp::Open
    }

    pub fn is_close(&self) -> bool {
        self.chunk_op == ChunkOp::Close
    }

    pub fn is_round(&self) -> bool {
        self.chunk_type == ChunkType::Round
    }

    pub fn is_square(&self) -> bool {
        self.chunk_type == ChunkType::Square
    }

    pub fn is_curly(&self) -> bool {
        self.chunk_type == ChunkType::Curly
    }

    pub fn is_angle(&self) -> bool {
        self.chunk_type == ChunkType::Angle
    }

    pub fn is_matching(&self, other: &ChunkToken) -> bool {
        self.chunk_type == other.chunk_type
            && ((self.is_open() && other.is_close()) || (self.is_close() && other.is_open()))
    }

    pub fn syntax_score(&self) -> u64 {
        match self.chunk_type {
            ChunkType::Round => 3,
            ChunkType::Square => 57,
            ChunkType::Curly => 1197,
            ChunkType::Angle => 25137,
        }
    }

    pub fn completion_score(&self) -> u64 {
        match self.chunk_type {
            ChunkType::Round => 1,
            ChunkType::Square => 2,
            ChunkType::Curly => 3,
            ChunkType::Angle => 4,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ChunkType {
    Round,
    Square,
    Curly,
    Angle,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ChunkOp {
    Open,
    Close,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Line {
    tokens: Vec<ChunkToken>,
}

impl FromStr for Line {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s
            .chars()
            .map(|c| ChunkToken::try_from(c))
            .collect::<Result<Vec<ChunkToken>, String>>()?;

        Ok(Self { tokens })
    }
}

impl Line {
    pub fn first_mismatching_token(&self) -> Option<ChunkToken> {
        let mut stack: Vec<ChunkToken> = Vec::new();

        for token in &self.tokens {
            if token.is_open() {
                stack.push(*token);
                continue;
            }

            match stack.pop().and_then(|x| Some(x.is_matching(token))) {
                Some(false) => return Some(*token),
                None => return None,
                _ => (),
            };
        }

        None
    }

    pub fn completion_score(&self) -> Option<u64> {
        let mut stack: Vec<ChunkToken> = Vec::new();

        for token in &self.tokens {
            if token.is_open() {
                stack.push(*token);
                continue;
            }

            match stack.pop().and_then(|x| Some(x.is_matching(token))) {
                Some(false) => return None,
                None => return None,
                _ => (),
            };
        }

        // at this point we just calculate the score based on the remaining stack
        let score = stack
            .iter()
            .rev()
            .fold(0, |acc, x| acc * 5 + x.completion_score());

        Some(score)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Subsystem {
    lines: Vec<Line>,
}

impl TryFrom<Vec<String>> for Subsystem {
    type Error = String;

    fn try_from(input: Vec<String>) -> Result<Self, Self::Error> {
        let lines = input
            .iter()
            .map(|x| Line::from_str(x))
            .collect::<Result<Vec<Line>, String>>()?;

        Ok(Self { lines })
    }
}

impl Subsystem {
    pub fn total_syntax_error_score(&self) -> u64 {
        self.lines
            .iter()
            .filter_map(|x| x.first_mismatching_token())
            .map(|x| x.syntax_score())
            .sum()
    }

    pub fn median_completion_score(&self) -> u64 {
        let mut scores: Vec<u64> = self
            .lines
            .iter()
            .filter_map(|x| x.completion_score())
            .collect();

        scores.sort();

        // from problem description: there will always be an odd number of scores to consider
        scores[scores.len() / 2]
    }
}

pub fn parse_input(lines: Vec<String>) -> Result<Subsystem, String> {
    Subsystem::try_from(lines)
}

pub fn part_one(subsystem: &Subsystem) -> u64 {
    subsystem.total_syntax_error_score()
}

pub fn part_two(subsystem: &Subsystem) -> u64 {
    subsystem.median_completion_score()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunk_token_try_from_char_test() {
        let inputs: Vec<char> = "()[]{}<>".chars().collect();
        let expected = vec![
            ChunkToken {
                chunk_type: ChunkType::Round,
                chunk_op: ChunkOp::Open,
            },
            ChunkToken {
                chunk_type: ChunkType::Round,
                chunk_op: ChunkOp::Close,
            },
            ChunkToken {
                chunk_type: ChunkType::Square,
                chunk_op: ChunkOp::Open,
            },
            ChunkToken {
                chunk_type: ChunkType::Square,
                chunk_op: ChunkOp::Close,
            },
            ChunkToken {
                chunk_type: ChunkType::Curly,
                chunk_op: ChunkOp::Open,
            },
            ChunkToken {
                chunk_type: ChunkType::Curly,
                chunk_op: ChunkOp::Close,
            },
            ChunkToken {
                chunk_type: ChunkType::Angle,
                chunk_op: ChunkOp::Open,
            },
            ChunkToken {
                chunk_type: ChunkType::Angle,
                chunk_op: ChunkOp::Close,
            },
        ];

        for (i, e) in inputs.iter().zip(expected) {
            let actual = ChunkToken::try_from(*i);
            assert!(actual.is_ok());
            assert_eq!(actual.unwrap(), e);
        }
    }

    #[test]
    fn first_mismatching_token_test() {
        let inputs = [
            "{([(<{}[<>[]}>{[]{[(<()>",
            "[[<[([]))<([[{}[[()]]]",
            "[{[{({}]{}}([{[{{{}}([]",
            "[<(<(<(<{}))><([]([]()",
            "<{([([[(<>()){}]>(<<{{",
            "()",
            "",
            "([]{<>})",
            "([]",
        ];
        let lines: Vec<Line> = inputs.iter().map(|s| Line::from_str(s).unwrap()).collect();
        let expected = vec![
            Some(ChunkToken::try_from('}').unwrap()),
            Some(ChunkToken::try_from(')').unwrap()),
            Some(ChunkToken::try_from(']').unwrap()),
            Some(ChunkToken::try_from(')').unwrap()),
            Some(ChunkToken::try_from('>').unwrap()),
            None,
            None,
            None,
            None,
        ];

        for (l, e) in lines.iter().zip(expected) {
            let actual = l.first_mismatching_token();
            assert_eq!(actual, e);
        }
    }

    #[test]
    fn completion_score_test() {
        let inputs = [
            "{([(<{}[<>[]}>{[]{[(<()>",
            "[[<[([]))<([[{}[[()]]]",
            "[{[{({}]{}}([{[{{{}}([]",
            "[<(<(<(<{}))><([]([]()",
            "<{([([[(<>()){}]>(<<{{",
            "[({(<(())[]>[[{[]{<()<>>",
            "[(()[<>])]({[<{<<[]>>(",
            "(((({<>}<{<{<>}{[]{[]{}",
            "{<[[]]>}<{[{[{[]{()[[[]",
            "<{([{{}}[<[[[<>{}]]]>[]]",
        ];
        let lines: Vec<Line> = inputs.iter().map(|s| Line::from_str(s).unwrap()).collect();
        let expected = vec![
            None,
            None,
            None,
            None,
            None,
            Some(288957),
            Some(5566),
            Some(1480781),
            Some(995444),
            Some(294),
        ];

        for (l, e) in lines.iter().zip(expected) {
            let actual = l.completion_score();
            assert_eq!(actual, e);
        }
    }

    #[test]
    fn parse_input_test() {
        let lines = vec!["([])".to_string(), "{<)]}>".to_string()];

        let actual = parse_input(lines).expect("could not parse input");
        let expected = Subsystem {
            lines: vec![
                Line {
                    tokens: vec![
                        ChunkToken {
                            chunk_type: ChunkType::Round,
                            chunk_op: ChunkOp::Open,
                        },
                        ChunkToken {
                            chunk_type: ChunkType::Square,
                            chunk_op: ChunkOp::Open,
                        },
                        ChunkToken {
                            chunk_type: ChunkType::Square,
                            chunk_op: ChunkOp::Close,
                        },
                        ChunkToken {
                            chunk_type: ChunkType::Round,
                            chunk_op: ChunkOp::Close,
                        },
                    ],
                },
                Line {
                    tokens: vec![
                        ChunkToken {
                            chunk_type: ChunkType::Curly,
                            chunk_op: ChunkOp::Open,
                        },
                        ChunkToken {
                            chunk_type: ChunkType::Angle,
                            chunk_op: ChunkOp::Open,
                        },
                        ChunkToken {
                            chunk_type: ChunkType::Round,
                            chunk_op: ChunkOp::Close,
                        },
                        ChunkToken {
                            chunk_type: ChunkType::Square,
                            chunk_op: ChunkOp::Close,
                        },
                        ChunkToken {
                            chunk_type: ChunkType::Curly,
                            chunk_op: ChunkOp::Close,
                        },
                        ChunkToken {
                            chunk_type: ChunkType::Angle,
                            chunk_op: ChunkOp::Close,
                        },
                    ],
                },
            ],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn part_one_test() {
        let input = vec![
            "[({(<(())[]>[[{[]{<()<>>".to_string(),
            "[(()[<>])]({[<{<<[]>>(".to_string(),
            "{([(<{}[<>[]}>{[]{[(<()>".to_string(),
            "(((({<>}<{<{<>}{[]{[]{}".to_string(),
            "[[<[([]))<([[{}[[()]]]".to_string(),
            "[{[{({}]{}}([{[{{{}}([]".to_string(),
            "{<[[]]>}<{[{[{[]{()[[[]".to_string(),
            "[<(<(<(<{}))><([]([]()".to_string(),
            "<{([([[(<>()){}]>(<<{{".to_string(),
            "<{([{{}}[<[[[<>{}]]]>[]]".to_string(),
        ];
        let subsystem = parse_input(input).expect("could not parse input");

        assert_eq!(part_one(&subsystem), 26397);
    }

    #[test]
    fn part_two_test() {
        let input = vec![
            "[({(<(())[]>[[{[]{<()<>>".to_string(),
            "[(()[<>])]({[<{<<[]>>(".to_string(),
            "{([(<{}[<>[]}>{[]{[(<()>".to_string(),
            "(((({<>}<{<{<>}{[]{[]{}".to_string(),
            "[[<[([]))<([[{}[[()]]]".to_string(),
            "[{[{({}]{}}([{[{{{}}([]".to_string(),
            "{<[[]]>}<{[{[{[]{()[[[]".to_string(),
            "[<(<(<(<{}))><([]([]()".to_string(),
            "<{([([[(<>()){}]>(<<{{".to_string(),
            "<{([{{}}[<[[[<>{}]]]>[]]".to_string(),
        ];
        let subsystem = parse_input(input).expect("could not parse input");

        assert_eq!(part_two(&subsystem), 288957);
    }
}
