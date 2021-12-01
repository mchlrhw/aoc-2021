#[cfg(test)]
mod part_01;
#[cfg(test)]
mod part_02;

const EXAMPLE_INPUT: &str = include_str!("../resources/day_01/example_input.txt");
const PUZZLE_INPUT: &str = include_str!("../resources/day_01/puzzle_input.txt");

fn to_depth_iter(input: &'static str) -> impl Iterator<Item = u64> {
    input
        .split_whitespace()
        .map(|s| s.parse::<u64>().expect("must be valid number"))
}
