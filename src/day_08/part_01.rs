use super::{EXAMPLE_INPUT, PUZZLE_INPUT};
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, space1},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
struct Digit(String);

impl From<&str> for Digit {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl Digit {
    fn is_unique(&self) -> bool {
        matches!(self.0.len(), 2 | 3 | 4 | 7)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Display {
    signal_patterns: [Digit; 10],
    output_value: [Digit; 4],
}

impl Display {
    fn unique_output_count(&self) -> usize {
        self.output_value.iter().filter(|d| d.is_unique()).count()
    }
}

fn digits(input: &str) -> IResult<&str, Vec<Digit>> {
    map(separated_list1(space1, alpha1), |v: Vec<&str>| {
        v.iter().map(|s| Digit(s.to_string())).collect()
    })(input)
}

fn output_value(input: &str) -> IResult<&str, [Digit; 4]> {
    let (remainder, output) = digits(input)?;
    let output = output.try_into().expect("if only this could be an error");

    Ok((remainder, output))
}

fn signal_patterns(input: &str) -> IResult<&str, [Digit; 10]> {
    let (remainder, patterns) = digits(input)?;
    let patterns = patterns.try_into().expect("if only this could be an error");

    Ok((remainder, patterns))
}

fn display(input: &str) -> IResult<&str, Display> {
    map(
        separated_pair(signal_patterns, tag(" | "), output_value),
        |(s, o)| Display {
            signal_patterns: s,
            output_value: o,
        },
    )(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Display>> {
    separated_list1(line_ending, display)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_line_example() -> anyhow::Result<()> {
        let input =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let expected = vec![Display {
            signal_patterns: [
                "acedgfb".into(),
                "cdfbe".into(),
                "gcdfa".into(),
                "fbcad".into(),
                "dab".into(),
                "cefabd".into(),
                "cdfgeb".into(),
                "eafb".into(),
                "cagedb".into(),
                "ab".into(),
            ],
            output_value: [
                "cdfeb".into(),
                "fcadb".into(),
                "cdfeb".into(),
                "cdbaf".into(),
            ],
        }];

        let (_, displays) = parse_input(input)?;

        assert_eq!(displays, expected);

        Ok(())
    }

    #[test]
    fn example() -> anyhow::Result<()> {
        let (_, displays) = parse_input(EXAMPLE_INPUT)?;
        let count: usize = displays.iter().map(Display::unique_output_count).sum();

        assert_eq!(count, 26);

        Ok(())
    }

    #[test]
    fn puzzle() -> anyhow::Result<()> {
        let (_, displays) = parse_input(PUZZLE_INPUT)?;
        let count: usize = displays.iter().map(Display::unique_output_count).sum();

        assert_eq!(count, 237);

        Ok(())
    }
}
