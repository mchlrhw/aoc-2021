use super::{bingo, Boards, EXAMPLE_INPUT, PUZZLE_INPUT};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> anyhow::Result<()> {
        let mut losing_board = None;
        let mut losing_number = None;

        let (_, (numbers, boards)) = bingo(EXAMPLE_INPUT)?;
        let mut boards = Boards(boards);

        'bingo: for number in numbers {
            if boards.0.len() > 1 {
                boards.mark_all_and_remove_winners(number);
            } else if let Some(board) = boards.mark_all(number) {
                losing_board = Some(board);
                losing_number = Some(u64::from(number));
                break 'bingo;
            }
        }

        let sum = losing_board
            .expect("must have found a winning board")
            .unmarked_sum();

        assert_eq!(
            sum * losing_number.expect("must have found a winning number"),
            1924
        );

        Ok(())
    }

    #[test]
    fn puzzle() -> anyhow::Result<()> {
        let mut losing_board = None;
        let mut losing_number = None;

        let (_, (numbers, boards)) = bingo(PUZZLE_INPUT)?;
        let mut boards = Boards(boards);

        'bingo: for number in numbers {
            if boards.0.len() > 1 {
                boards.mark_all_and_remove_winners(number);
            } else if let Some(board) = boards.mark_all(number) {
                losing_board = Some(board);
                losing_number = Some(u64::from(number));
                break 'bingo;
            }
        }

        let sum = losing_board
            .expect("must have found a winning board")
            .unmarked_sum();

        assert_eq!(
            sum * losing_number.expect("must have found a winning number"),
            31755
        );

        Ok(())
    }
}
