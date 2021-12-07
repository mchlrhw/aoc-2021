#[cfg(test)]
mod part_01;
#[cfg(test)]
mod part_02;

const EXAMPLE_INPUT: &str = include_str!("../resources/day_07/example_input.txt");
const PUZZLE_INPUT: &str = include_str!("../resources/day_07/puzzle_input.txt");

fn parse_positions(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(|s| s.parse().expect("must be a i64"))
        .collect()
}
