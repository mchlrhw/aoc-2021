use super::{parse_positions, EXAMPLE_INPUT, PUZZLE_INPUT};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let positions = parse_positions(EXAMPLE_INPUT);
        let num_crabs = positions.len();

        let total: i64 = positions.iter().sum();
        let mean = total / num_crabs as i64;

        let mut candidates = vec![];
        for i in 0..(num_crabs / 3) {
            let pos_total_fuel: i64 = positions.iter().map(|p| (p - mean - i as i64).abs()).sum();
            let neg_total_fuel: i64 = positions.iter().map(|p| (p - mean + i as i64).abs()).sum();

            candidates.push(pos_total_fuel);
            candidates.push(neg_total_fuel);
        }

        let best = *candidates.iter().min().expect("must have candidates");

        assert_eq!(best, 37);
    }

    #[test]
    fn puzzle() {
        let positions = parse_positions(PUZZLE_INPUT);
        let num_crabs = positions.len();

        let total: i64 = positions.iter().sum();
        let mean = total / num_crabs as i64;

        let mut candidates = vec![];
        for i in 0..(num_crabs / 3) {
            let pos_total_fuel: i64 = positions.iter().map(|p| (p - mean - i as i64).abs()).sum();
            let neg_total_fuel: i64 = positions.iter().map(|p| (p - mean + i as i64).abs()).sum();

            candidates.push(pos_total_fuel);
            candidates.push(neg_total_fuel);
        }

        let best = *candidates.iter().min().expect("must have candidates");

        assert_eq!(best, 355_764);
    }
}
