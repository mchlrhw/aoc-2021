#[cfg(test)]
mod part_01;
#[cfg(test)]
mod part_02;

use nom::{
    character::complete::{
        char as parse_char, line_ending, multispace1, space0, space1, u8 as parse_u8,
    },
    combinator::map,
    multi::{count, separated_list1},
    sequence::{delimited, separated_pair},
    IResult,
};

use std::ops::Not;
const EXAMPLE_INPUT: &str = include_str!("../resources/day_04/example_input.txt");
const PUZZLE_INPUT: &str = include_str!("../resources/day_04/puzzle_input.txt");

#[derive(Clone, Debug, PartialEq, Eq)]
struct Cell {
    val: u8,
    marked: bool,
}

impl Cell {
    fn new(val: u8) -> Self {
        Self { val, marked: false }
    }

    fn new_marked(val: u8) -> Self {
        Self { val, marked: true }
    }

    fn mark_if_equal(&mut self, number: u8) {
        if self.val == number {
            self.marked = true;
        }
    }
}

type Row = [Cell; 5];

#[derive(Clone, Debug, PartialEq, Eq)]
struct Board {
    rows: [Row; 5],
}

impl Board {
    fn cells(&self) -> impl Iterator<Item = &Cell> {
        self.rows.iter().flatten()
    }

    fn cells_mut(&mut self) -> impl Iterator<Item = &mut Cell> {
        self.rows.iter_mut().flatten()
    }

    fn columns(&self) -> Vec<Vec<Cell>> {
        let mut columns = vec![];
        for _ in 0..5 {
            let mut column = vec![];
            for _ in 0..5 {
                column.push(Cell::new(0));
            }
            columns.push(column);
        }

        for row_idx in 0..5 {
            for (col_idx, column) in columns.iter_mut().enumerate() {
                column[row_idx] = self.rows[row_idx][col_idx].clone();
            }
        }

        columns
    }

    fn has_won(&self) -> bool {
        for row in &self.rows {
            if row.iter().all(|c| c.marked) {
                return true;
            }
        }

        for column in self.columns() {
            if column.iter().all(|c| c.marked) {
                return true;
            }
        }

        false
    }

    fn mark(&mut self, number: u8) {
        for cell in self.cells_mut() {
            cell.mark_if_equal(number);
        }
    }

    fn unmarked_sum(&self) -> u64 {
        self.cells()
            .filter(|c| c.marked.not())
            .map(|c| u64::from(c.val))
            .sum()
    }
}

#[derive(Debug)]
struct Boards(Vec<Board>);

impl Boards {
    fn mark_all(&mut self, number: u8) -> Option<Board> {
        for board in &mut self.0 {
            board.mark(number);
            if board.has_won() {
                return Some(board.clone());
            }
        }

        None
    }

    fn mark_all_and_remove_winners(&mut self, number: u8) {
        for board in &mut self.0 {
            board.mark(number);
        }

        self.0 = self
            .0
            .iter()
            .cloned()
            .filter(|b| b.has_won().not())
            .collect();
    }
}

fn cell(input: &str) -> IResult<&str, Cell> {
    map(parse_u8, Cell::new)(input)
}

fn row(input: &str) -> IResult<&str, Row> {
    let (remainder, cells) = separated_list1(space1, cell)(input)?;
    let row = cells.try_into().expect("barf");

    Ok((remainder, row))
}

fn board(input: &str) -> IResult<&str, Board> {
    let (remainder, rows) = count(delimited(space0, row, line_ending), 5)(input)?;
    let board = Board {
        rows: rows.try_into().expect("kaboom"),
    };

    Ok((remainder, board))
}

fn bingo(input: &str) -> IResult<&str, (Vec<u8>, Vec<Board>)> {
    separated_pair(
        separated_list1(parse_char(','), parse_u8),
        multispace1,
        separated_list1(multispace1, board),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_board() -> anyhow::Result<()> {
        let input = "22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19
";
        let expected = Board {
            rows: [
                [
                    Cell::new(22),
                    Cell::new(13),
                    Cell::new(17),
                    Cell::new(11),
                    Cell::new(0),
                ],
                [
                    Cell::new(8),
                    Cell::new(2),
                    Cell::new(23),
                    Cell::new(4),
                    Cell::new(24),
                ],
                [
                    Cell::new(21),
                    Cell::new(9),
                    Cell::new(14),
                    Cell::new(16),
                    Cell::new(7),
                ],
                [
                    Cell::new(6),
                    Cell::new(10),
                    Cell::new(3),
                    Cell::new(18),
                    Cell::new(5),
                ],
                [
                    Cell::new(1),
                    Cell::new(12),
                    Cell::new(20),
                    Cell::new(15),
                    Cell::new(19),
                ],
            ],
        };

        let (_, board) = board(input)?;

        assert_eq!(board, expected);

        Ok(())
    }

    #[test]
    fn cells_works() -> anyhow::Result<()> {
        let input = "22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19
";
        let expected = vec![
            Cell::new(22),
            Cell::new(13),
            Cell::new(17),
            Cell::new(11),
            Cell::new(0),
            Cell::new(8),
            Cell::new(2),
            Cell::new(23),
            Cell::new(4),
            Cell::new(24),
            Cell::new(21),
            Cell::new(9),
            Cell::new(14),
            Cell::new(16),
            Cell::new(7),
            Cell::new(6),
            Cell::new(10),
            Cell::new(3),
            Cell::new(18),
            Cell::new(5),
            Cell::new(1),
            Cell::new(12),
            Cell::new(20),
            Cell::new(15),
            Cell::new(19),
        ];

        let (_, board) = board(input)?;
        let cells: Vec<Cell> = board.cells().cloned().collect();

        assert_eq!(cells, expected);

        Ok(())
    }

    #[test]
    fn hasnt_won() {
        let board = Board {
            rows: [
                [
                    Cell::new(22),
                    Cell::new(13),
                    Cell::new(17),
                    Cell::new(11),
                    Cell::new(0),
                ],
                [
                    Cell::new(8),
                    Cell::new(2),
                    Cell::new(23),
                    Cell::new(4),
                    Cell::new(24),
                ],
                [
                    Cell::new(21),
                    Cell::new(9),
                    Cell::new(14),
                    Cell::new(16),
                    Cell::new(7),
                ],
                [
                    Cell::new(6),
                    Cell::new(10),
                    Cell::new(3),
                    Cell::new(18),
                    Cell::new(5),
                ],
                [
                    Cell::new(1),
                    Cell::new(12),
                    Cell::new(20),
                    Cell::new(15),
                    Cell::new(19),
                ],
            ],
        };

        assert!(board.has_won().not());
    }

    #[test]
    fn hasnt_won_row() {
        let board = Board {
            rows: [
                [
                    Cell::new(22),
                    Cell::new(13),
                    Cell::new(17),
                    Cell::new(11),
                    Cell::new(0),
                ],
                [
                    Cell::new_marked(8),
                    Cell::new_marked(2),
                    Cell::new_marked(23),
                    Cell::new_marked(4),
                    Cell::new_marked(24),
                ],
                [
                    Cell::new(21),
                    Cell::new(9),
                    Cell::new(14),
                    Cell::new(16),
                    Cell::new(7),
                ],
                [
                    Cell::new(6),
                    Cell::new(10),
                    Cell::new(3),
                    Cell::new(18),
                    Cell::new(5),
                ],
                [
                    Cell::new(1),
                    Cell::new(12),
                    Cell::new(20),
                    Cell::new(15),
                    Cell::new(19),
                ],
            ],
        };

        assert!(board.has_won());
    }

    #[test]
    fn hasnt_won_column() {
        let board = Board {
            rows: [
                [
                    Cell::new(22),
                    Cell::new(13),
                    Cell::new_marked(17),
                    Cell::new(11),
                    Cell::new(0),
                ],
                [
                    Cell::new(8),
                    Cell::new(2),
                    Cell::new_marked(23),
                    Cell::new(4),
                    Cell::new(24),
                ],
                [
                    Cell::new(21),
                    Cell::new(9),
                    Cell::new_marked(14),
                    Cell::new(16),
                    Cell::new(7),
                ],
                [
                    Cell::new(6),
                    Cell::new(10),
                    Cell::new_marked(3),
                    Cell::new(18),
                    Cell::new(5),
                ],
                [
                    Cell::new(1),
                    Cell::new(12),
                    Cell::new_marked(20),
                    Cell::new(15),
                    Cell::new(19),
                ],
            ],
        };

        assert!(board.has_won());
    }
}
