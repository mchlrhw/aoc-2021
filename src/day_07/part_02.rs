use super::{parse_positions, EXAMPLE_INPUT, PUZZLE_INPUT};

fn fuel_used(from: i64, to: i64) -> i64 {
    let distance = (from - to).abs();

    (1..=distance).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(16, 5, 66)]
    #[test_case(1, 5, 10)]
    #[test_case(2, 5, 6)]
    #[test_case(0, 5, 15)]
    #[test_case(4, 5, 1)]
    #[test_case(7, 5, 3)]
    #[test_case(14, 5, 45)]
    fn fuel_used_works(from: i64, to: i64, expected: i64) {
        let fuel = fuel_used(from, to);

        assert_eq!(fuel, expected);
    }

    #[test]
    fn example() {
        let positions = parse_positions(EXAMPLE_INPUT);
        let min = *positions
            .iter()
            .min()
            .expect("must have at least one position");
        let max = *positions
            .iter()
            .max()
            .expect("must have at least one position");

        let mut candidates = vec![];
        for new_pos in min..=max {
            let fuel: i64 = positions
                .iter()
                .map(|crab_pos| fuel_used(*crab_pos, new_pos))
                .sum();
            candidates.push(fuel);
        }

        let best = *candidates
            .iter()
            .min()
            .expect("must have at least one candidate");

        assert_eq!(best, 168);
    }

    #[test]
    fn puzzle() {
        let positions = parse_positions(PUZZLE_INPUT);
        let min = *positions
            .iter()
            .min()
            .expect("must have at least one position");
        let max = *positions
            .iter()
            .max()
            .expect("must have at least one position");

        let mut candidates = vec![];
        for new_pos in min..=max {
            let fuel: i64 = positions
                .iter()
                .map(|crab_pos| fuel_used(*crab_pos, new_pos))
                .sum();
            candidates.push(fuel);
        }

        let best = *candidates
            .iter()
            .min()
            .expect("must have at least one candidate");

        assert_eq!(best, 99_634_572);
    }
}
