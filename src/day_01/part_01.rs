use super::{to_depth_iter, EXAMPLE_INPUT, PUZZLE_INPUT};
use itertools::Itertools;

fn count_depth_increases(depths: impl Iterator<Item = u64>) -> usize {
    depths
        .tuple_windows::<(_, _)>()
        .map(|(a, b)| if a < b { 1 } else { 0 })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let depths = to_depth_iter(EXAMPLE_INPUT);
        let total_increases = count_depth_increases(depths);

        assert_eq!(total_increases, 7);
    }

    #[test]
    fn puzzle() {
        let depths = to_depth_iter(PUZZLE_INPUT);
        let total_increases = count_depth_increases(depths);

        assert_eq!(total_increases, 1527);
    }
}
