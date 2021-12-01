use itertools::Itertools;

fn to_depth_iter(input: &'static str) -> impl Iterator<Item = u64> {
    input
        .split_whitespace()
        .map(|s| s.parse::<u64>().expect("must be valid number"))
}

fn count_window_increases(depths: impl Iterator<Item = u64>) -> usize {
    depths
        .tuple_windows::<(_, _, _)>()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows::<(_, _)>()
        .map(|(a, b)| if a < b { 1 } else { 0 })
        .sum()
}

fn main() {
    let puzzle_input = include_str!("../resources/day-01-part-01-input.txt");

    let depths = to_depth_iter(puzzle_input);
    let total_increases = count_window_increases(depths);

    println!("{}", total_increases);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "
199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn example() {
        let depths = to_depth_iter(EXAMPLE_INPUT);
        let total_increases = count_window_increases(depths);

        assert_eq!(total_increases, 5);
    }
}
