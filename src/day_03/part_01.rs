use super::{EXAMPLE_INPUT, PUZZLE_INPUT};

struct Counter {
    zeros: usize,
    ones: usize,
}

impl Counter {
    fn new() -> Self {
        Self { zeros: 0, ones: 0 }
    }

    fn update(&mut self, c: char) {
        match c {
            '0' => self.zeros += 1,
            '1' => self.ones += 1,
            _ => panic!("char is not a binary digit: '{}'", c),
        }
    }

    fn most_common(&self) -> char {
        if self.zeros > self.ones {
            '0'
        } else if self.ones > self.zeros {
            '1'
        } else {
            panic!("there is no most common char");
        }
    }
}

struct Tracker(Vec<Counter>);

impl Tracker {
    fn new(cap: usize) -> Self {
        let mut inner = Vec::with_capacity(cap);
        for _ in 0..cap {
            inner.push(Counter::new());
        }

        Self(inner)
    }

    fn update(&mut self, s: &str) {
        for (i, c) in s.chars().enumerate() {
            self.0[i].update(c);
        }
    }

    fn finalise(self) -> String {
        let mut output = String::new();
        for counter in self.0 {
            output.push(counter.most_common());
        }

        output
    }
}

fn find_gamma(input: &str) -> String {
    let line_len = input
        .lines()
        .next()
        .expect("input must have at least one line")
        .len();
    let mut tracker = Tracker::new(line_len);

    for line in input.lines() {
        tracker.update(line);
    }

    tracker.finalise()
}

fn flip_bits(s: &str) -> String {
    s.chars()
        .map(|c| if c == '1' { '0' } else { '1' })
        .collect()
}

fn calculate_power_consumption(gamma: &str) -> u64 {
    let epsilon = flip_bits(gamma);

    let gamma_num = u64::from_str_radix(&gamma, 2).unwrap();
    let epsilon_num = u64::from_str_radix(&epsilon, 2).unwrap();

    gamma_num * epsilon_num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let gamma = find_gamma(EXAMPLE_INPUT);
        let pc = calculate_power_consumption(&gamma);

        assert_eq!(pc, 198);
    }

    #[test]
    fn puzzle() {
        let gamma = find_gamma(PUZZLE_INPUT);
        let pc = calculate_power_consumption(&gamma);

        assert_eq!(pc, 2498354);
    }
}
