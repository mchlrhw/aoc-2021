#[cfg(test)]
mod part_01;
#[cfg(test)]
mod part_02;

use anyhow::bail;
use std::str::FromStr;

const EXAMPLE_INPUT: &str = include_str!("../resources/day_02/example_input.txt");
const PUZZLE_INPUT: &str = include_str!("../resources/day_02/puzzle_input.txt");

#[derive(Debug)]
enum Command {
    Forward(i64),
    Down(i64),
    Up(i64),
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Command::{Down, Forward, Up};

        let command = match s.split_once(" ") {
            Some((command, amount)) => {
                let parsed_amount = amount.parse()?;

                match command {
                    "forward" => Forward(parsed_amount),
                    "up" => Up(parsed_amount),
                    "down" => Down(parsed_amount),
                    _ => bail!("command prefix is invalid: '{}'", command),
                }
            }
            None => bail!("string is not a valid command: '{}'", s),
        };

        Ok(command)
    }
}

fn to_command_iter(input: &'static str) -> impl Iterator<Item = Command> {
    input
        .lines()
        .map(|s| s.parse::<Command>().expect("must be valid command"))
}
