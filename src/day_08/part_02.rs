use super::{EXAMPLE_INPUT, PUZZLE_INPUT};
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, space1},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Digit(HashSet<char>);

impl<S: AsRef<str>> From<S> for Digit {
    fn from(s: S) -> Self {
        Self(s.as_ref().chars().collect())
    }
}

impl Digit {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl PartialEq<u8> for &Digit {
    fn eq(&self, d: &u8) -> bool {
        match self.0.len() {
            2 => *d == 1,
            3 => *d == 7,
            4 => *d == 4,
            7 => *d == 8,
            _ => false,
        }
    }
}

impl std::ops::Sub for &Digit {
    type Output = HashSet<char>;

    fn sub(self, other: &Digit) -> Self::Output {
        self.0.difference(&other.0).copied().collect()
    }
}

impl std::ops::Sub<HashSet<char>> for &Digit {
    type Output = HashSet<char>;

    fn sub(self, other: HashSet<char>) -> Self::Output {
        self.0.difference(&other).copied().collect()
    }
}

trait State {}

#[derive(Debug, PartialEq, Eq)]
struct Unsolved;
impl State for Unsolved {}

#[derive(Debug, PartialEq, Eq)]
struct FoundUniques {
    one: Digit,
    four: Digit,
    seven: Digit,
    eight: Digit,
}
impl State for FoundUniques {}

#[derive(Debug, PartialEq, Eq)]
struct FoundNine {
    one: Digit,
    four: Digit,
    seven: Digit,
    eight: Digit,
    nine: Digit,
}
impl State for FoundNine {}

#[derive(Debug, PartialEq, Eq)]
struct FoundTwo {
    one: Digit,
    two: Digit,
    four: Digit,
    seven: Digit,
    eight: Digit,
    nine: Digit,
}
impl State for FoundTwo {}

#[derive(Debug, PartialEq, Eq)]
struct FoundThree {
    one: Digit,
    two: Digit,
    three: Digit,
    four: Digit,
    seven: Digit,
    eight: Digit,
    nine: Digit,
}
impl State for FoundThree {}

#[derive(Debug, PartialEq, Eq)]
struct FoundFive {
    one: Digit,
    two: Digit,
    three: Digit,
    four: Digit,
    five: Digit,
    seven: Digit,
    eight: Digit,
    nine: Digit,
}
impl State for FoundFive {}

#[derive(Debug, PartialEq, Eq)]
struct FoundZero {
    zero: Digit,
    one: Digit,
    two: Digit,
    three: Digit,
    four: Digit,
    five: Digit,
    seven: Digit,
    eight: Digit,
    nine: Digit,
}
impl State for FoundZero {}

#[derive(Debug, PartialEq, Eq)]
struct Solved {
    zero: Digit,
    one: Digit,
    two: Digit,
    three: Digit,
    four: Digit,
    five: Digit,
    six: Digit,
    seven: Digit,
    eight: Digit,
    nine: Digit,
}
impl State for Solved {}

#[derive(Debug, PartialEq, Eq)]
struct Display<State> {
    signal_patterns: [Digit; 10],
    output_value: [Digit; 4],
    state: State,
}

impl<S: State> Display<S> {
    fn new(signal_patterns: [Digit; 10], output_value: [Digit; 4]) -> Display<Unsolved> {
        Display {
            signal_patterns,
            output_value,
            state: Unsolved,
        }
    }
}

impl Display<Unsolved> {
    fn find_uniques(self) -> Display<FoundUniques> {
        let mut one = None;
        let mut four = None;
        let mut seven = None;
        let mut eight = None;

        for digit in &self.signal_patterns {
            if digit == 1 {
                one = Some(digit.clone());
            } else if digit == 4 {
                four = Some(digit.clone());
            } else if digit == 7 {
                seven = Some(digit.clone());
            } else if digit == 8 {
                eight = Some(digit.clone());
            }
        }

        let one = one.expect("must have found one");
        let four = four.expect("must have found four");
        let seven = seven.expect("must have found seven");
        let eight = eight.expect("must have found eight");

        Display {
            signal_patterns: self.signal_patterns,
            output_value: self.output_value,
            state: FoundUniques {
                one,
                four,
                seven,
                eight,
            },
        }
    }

    fn solve(self) -> Display<Solved> {
        self.find_uniques()
            .find_nine()
            .find_two()
            .find_three()
            .find_five()
            .find_zero()
            .find_six()
    }
}

impl Display<FoundUniques> {
    fn find_nine(self) -> Display<FoundNine> {
        let mut nine = None;

        for digit in &self.signal_patterns {
            if digit.len() == 6 && (digit - &self.state.four).len() == 2 {
                nine = Some(digit.clone());
            }
        }

        let nine = nine.expect("must have found nine");

        Display {
            signal_patterns: self.signal_patterns,
            output_value: self.output_value,
            state: FoundNine {
                one: self.state.one,
                four: self.state.four,
                seven: self.state.seven,
                eight: self.state.eight,
                nine,
            },
        }
    }
}

impl Display<FoundNine> {
    fn find_two(self) -> Display<FoundTwo> {
        let mut two = None;

        for digit in &self.signal_patterns {
            if digit.len() == 5 && (digit - (&self.state.eight - &self.state.nine)).len() == 4 {
                two = Some(digit.clone());
            }
        }

        let two = two.expect("must have found two");

        Display {
            signal_patterns: self.signal_patterns,
            output_value: self.output_value,
            state: FoundTwo {
                one: self.state.one,
                two,
                four: self.state.four,
                seven: self.state.seven,
                eight: self.state.eight,
                nine: self.state.nine,
            },
        }
    }
}

impl Display<FoundTwo> {
    fn find_three(self) -> Display<FoundThree> {
        let mut three = None;

        for digit in &self.signal_patterns {
            if digit.len() == 5 && (digit - &self.state.one).len() == 3 {
                three = Some(digit.clone());
            }
        }

        let three = three.expect("must have found three");

        Display {
            signal_patterns: self.signal_patterns,
            output_value: self.output_value,
            state: FoundThree {
                one: self.state.one,
                two: self.state.two,
                three,
                four: self.state.four,
                seven: self.state.seven,
                eight: self.state.eight,
                nine: self.state.nine,
            },
        }
    }
}

impl Display<FoundThree> {
    fn find_five(self) -> Display<FoundFive> {
        let mut five = None;

        for digit in &self.signal_patterns {
            if digit.len() == 5 && digit != &self.state.two && digit != &self.state.three {
                five = Some(digit.clone());
            }
        }

        let five = five.expect("must have found five");

        Display {
            signal_patterns: self.signal_patterns,
            output_value: self.output_value,
            state: FoundFive {
                one: self.state.one,
                two: self.state.two,
                three: self.state.three,
                five,
                four: self.state.four,
                seven: self.state.seven,
                eight: self.state.eight,
                nine: self.state.nine,
            },
        }
    }
}

impl Display<FoundFive> {
    fn find_zero(self) -> Display<FoundZero> {
        let mut zero = None;

        for digit in &self.signal_patterns {
            if digit.len() == 6 && (digit - &self.state.five).len() == 2 {
                zero = Some(digit.clone());
            }
        }

        let zero = zero.expect("must have found zero");

        Display {
            signal_patterns: self.signal_patterns,
            output_value: self.output_value,
            state: FoundZero {
                zero,
                one: self.state.one,
                two: self.state.two,
                three: self.state.three,
                four: self.state.four,
                five: self.state.five,
                seven: self.state.seven,
                eight: self.state.eight,
                nine: self.state.nine,
            },
        }
    }
}

impl Display<FoundZero> {
    fn find_six(self) -> Display<Solved> {
        let mut six = None;

        for digit in &self.signal_patterns {
            if digit.len() == 6 && digit != &self.state.zero && digit != &self.state.nine {
                six = Some(digit.clone());
            }
        }

        let six = six.expect("must have found six");

        Display {
            signal_patterns: self.signal_patterns,
            output_value: self.output_value,
            state: Solved {
                zero: self.state.zero,
                one: self.state.one,
                two: self.state.two,
                three: self.state.three,
                four: self.state.four,
                five: self.state.five,
                six,
                seven: self.state.seven,
                eight: self.state.eight,
                nine: self.state.nine,
            },
        }
    }
}

impl Display<Solved> {
    fn decode(&self) -> u64 {
        let mut output = String::new();
        for digit in &self.output_value {
            if digit == &self.state.zero {
                output.push('0');
            } else if digit == &self.state.one {
                output.push('1');
            } else if digit == &self.state.two {
                output.push('2');
            } else if digit == &self.state.three {
                output.push('3');
            } else if digit == &self.state.four {
                output.push('4');
            } else if digit == &self.state.five {
                output.push('5');
            } else if digit == &self.state.six {
                output.push('6');
            } else if digit == &self.state.seven {
                output.push('7');
            } else if digit == &self.state.eight {
                output.push('8');
            } else if digit == &self.state.nine {
                output.push('9');
            }
        }

        output.parse().expect("must be a valid number")
    }
}

fn digits(input: &str) -> IResult<&str, Vec<Digit>> {
    map(separated_list1(space1, alpha1), |v: Vec<&str>| {
        v.iter().map(std::convert::Into::into).collect()
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

fn display(input: &str) -> IResult<&str, Display<Unsolved>> {
    map(
        separated_pair(signal_patterns, tag(" | "), output_value),
        |(s, o)| Display::<Unsolved>::new(s, o),
    )(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Display<Unsolved>>> {
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
            state: Unsolved,
        }];

        let (_, displays) = parse_input(input)?;

        assert_eq!(displays, expected);

        Ok(())
    }

    #[test]
    fn example() -> anyhow::Result<()> {
        let (_, displays) = parse_input(EXAMPLE_INPUT)?;
        let total: u64 = displays.into_iter().map(|d| d.solve().decode()).sum();

        assert_eq!(total, 61229);

        Ok(())
    }

    #[test]
    fn puzzle() -> anyhow::Result<()> {
        let (_, displays) = parse_input(PUZZLE_INPUT)?;
        let total: u64 = displays.into_iter().map(|d| d.solve().decode()).sum();

        assert_eq!(total, 1_009_098);

        Ok(())
    }
}
