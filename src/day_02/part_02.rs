use super::{
    to_command_iter,
    Command::{self, Down, Forward, Up},
    EXAMPLE_INPUT, PUZZLE_INPUT,
};

fn apply_commands(
    coords: (i64, i64),
    mut aim: i64,
    commands: impl Iterator<Item = Command>,
) -> (i64, i64) {
    let (mut pos, mut depth) = coords;

    for command in commands {
        match command {
            Forward(amount) => {
                pos += amount;
                depth += aim * amount;
            }
            Down(amount) => aim += amount,
            Up(amount) => aim -= amount,
        }
    }

    (pos, depth)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let commands = to_command_iter(EXAMPLE_INPUT);

        let (pos, depth) = apply_commands((0, 0), 0, commands);
        let coords_mul = pos * depth;

        assert_eq!(coords_mul, 900);
    }

    #[test]
    fn puzzle() {
        let commands = to_command_iter(PUZZLE_INPUT);

        let (pos, depth) = apply_commands((0, 0), 0, commands);
        let coords_mul = pos * depth;

        assert_eq!(coords_mul, 1_997_106_066);
    }
}
