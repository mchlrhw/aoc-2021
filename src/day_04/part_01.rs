use super::{bingo, Boards, EXAMPLE_INPUT, PUZZLE_INPUT};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> anyhow::Result<()> {
        let mut winning_board = None;
        let mut winning_number = None;

        let (_, (numbers, boards)) = bingo(EXAMPLE_INPUT)?;
        let mut boards = Boards(boards);

        'bingo: for number in numbers {
            if let Some(board) = boards.mark_all(number) {
                winning_board = Some(board);
                winning_number = Some(u64::from(number));
                break 'bingo;
            };
        }

        let sum = winning_board
            .expect("must have found a winning board")
            .unmarked_sum();

        assert_eq!(
            sum * winning_number.expect("must have found a winning number"),
            4512
        );

        Ok(())
    }

    #[test]
    fn puzzle() -> anyhow::Result<()> {
        let mut winning_board = None;
        let mut winning_number = None;

        let (_, (numbers, boards)) = bingo(PUZZLE_INPUT)?;
        let mut boards = Boards(boards);

        'bingo: for number in numbers {
            if let Some(board) = boards.mark_all(number) {
                winning_board = Some(board);
                winning_number = Some(u64::from(number));
                break 'bingo;
            };
        }

        let sum = winning_board
            .expect("must have found a winning board")
            .unmarked_sum();

        assert_eq!(
            sum * winning_number.expect("must have found a winning number"),
            6592
        );

        Ok(())
    }
}
