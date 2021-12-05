use std::{collections::{HashMap, HashSet}, convert::TryFrom, num::ParseIntError};

#[derive(Default, Debug, Copy, Clone)]
struct BingoCell {
    value: i64,
    is_marked: bool,
}

impl BingoCell {
    pub fn new(value: i64) -> Self {
        Self {
            value,
            is_marked: false,
        }
    }

    pub fn mark(&mut self) {
        self.is_marked = true;
    }
}

#[derive(Default, Debug, Clone)]
pub struct BingoBoard {
    board: Vec<Vec<BingoCell>>,
    values_to_positions: HashMap<i64, Vec<(usize, usize)>>,
}

impl TryFrom<&[String]> for BingoBoard {
    type Error = ParseIntError;

    fn try_from(lines: &[String]) -> Result<Self, Self::Error> {
        let board: Vec<Vec<i64>> = lines
            .iter()
            .map(|line| {
                line.trim()
                    .split_whitespace()
                    .map(|x| x.parse())
                    .collect::<Result<Vec<i64>, ParseIntError>>()
            })
            .collect::<Result<_, ParseIntError>>()?;

        Ok(board.into())
    }
}

impl From<Vec<Vec<i64>>> for BingoBoard {
    fn from(values: Vec<Vec<i64>>) -> Self {
        let board: Vec<Vec<BingoCell>> = values
            .iter()
            .map(|row| {
                row.iter()
                    .map(|value| BingoCell::new(*value))
                    .collect::<Vec<BingoCell>>()
            })
            .collect();

        let mut values_to_positions: HashMap<i64, Vec<(usize, usize)>> = HashMap::new();

        for (i, row) in board.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                let value = cell.value;

                match values_to_positions.get_mut(&value) {
                    Some(list) => list.push((i, j)),
                    None => {
                        values_to_positions.insert(value, vec![(i, j)]);
                    }
                }
            }
        }

        Self {
            board,
            values_to_positions,
        }
    }
}

impl BingoBoard {
    pub fn mark_cell(&mut self, row: usize, col: usize) {
        self.board[row][col].mark();
    }

    pub fn mark_cells_with_value(&mut self, value: i64) -> Vec<(usize, usize)> {
        let positions = self
            .values_to_positions
            .get(&value)
            .cloned()
            .unwrap_or(Vec::new());

        for (i, j) in &positions {
            self.mark_cell(*i, *j);
        }

        positions
    }

    /// Given a row and column, determine if the row or column is in a winning state.
    pub fn is_winning(&self, row: usize, col: usize) -> bool {
        self.is_row_winning(row) || self.is_col_winning(col)
    }

    pub fn score(&self, value: i64) -> i64 {
        self.board
            .iter()
            .map(|row| {
                row.iter()
                    .filter_map(|cell| {
                        if cell.is_marked {
                            None
                        } else {
                            Some(cell.value)
                        }
                    })
                    .sum::<i64>()
            })
            .sum::<i64>()
            * value
    }

    fn is_row_winning(&self, row: usize) -> bool {
        self.board[row].iter().all(|cell| cell.is_marked)
    }

    fn is_col_winning(&self, col: usize) -> bool {
        self.board
            .iter()
            .map(|row| row[col])
            .all(|cell| cell.is_marked)
    }
}

pub fn part_one(sequence: &Vec<i64>, boards: &mut Vec<BingoBoard>) -> Result<i64, String> {
    for value in sequence {
        for board in boards.iter_mut() {
            let positions = board.mark_cells_with_value(*value);
            for (row, col) in positions {
                if board.is_winning(row, col) {
                    let score = board.score(*value) ;
                    return Ok(score);
                }
            }
        }
    }


    Err("no winning board".to_string())
}

pub fn part_two(sequence: &Vec<i64>, boards: &mut Vec<BingoBoard>) -> Result<i64, String> {
    let mut ongoing_board_indices: HashSet<usize> = HashSet::new();
    for i in 0..boards.len() {
        ongoing_board_indices.insert(i);
    }

    for value in sequence {
        for (i, board) in boards.iter_mut().enumerate() {
            let positions = board.mark_cells_with_value(*value);
            for (row, col) in positions {
                if board.is_winning(row, col) {
                    ongoing_board_indices.remove(&i);
                    if ongoing_board_indices.is_empty() {
                        let score = board.score(*value) ;
                        return Ok(score);
                    }
                }
            }
        }
    }


    Err("no winning board".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_test() {}
}
