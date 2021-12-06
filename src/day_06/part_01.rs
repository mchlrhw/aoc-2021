use super::{EXAMPLE_INPUT, PUZZLE_INPUT};

struct LanternFish {
    timer: i8,
}

impl LanternFish {
    fn new() -> Self {
        Self { timer: 8 }
    }

    fn from_timer(timer: i8) -> Self {
        Self { timer }
    }

    fn tick(&mut self) -> Option<LanternFish> {
        self.timer -= 1;

        if self.timer < 0 {
            self.timer = 6;

            Some(LanternFish::new())
        } else {
            None
        }
    }
}

fn parse_timers(input: &str) -> Vec<i8> {
    input
        .trim()
        .split(',')
        .map(|timer| timer.parse().expect("must be valid i8"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let timers = parse_timers(EXAMPLE_INPUT);
        let mut fish: Vec<LanternFish> = timers.into_iter().map(LanternFish::from_timer).collect();

        for _ in 0..18 {
            let mut new_fish = fish.iter_mut().filter_map(LanternFish::tick).collect();
            fish.append(&mut new_fish);
        }

        assert_eq!(fish.len(), 26);

        for _ in 18..80 {
            let mut new_fish = fish.iter_mut().filter_map(LanternFish::tick).collect();
            fish.append(&mut new_fish);
        }

        assert_eq!(fish.len(), 5934);
    }

    #[test]
    fn puzzle() {
        let timers = parse_timers(PUZZLE_INPUT);
        let mut fish: Vec<LanternFish> = timers.into_iter().map(LanternFish::from_timer).collect();

        for _ in 0..80 {
            let mut new_fish = fish.iter_mut().filter_map(LanternFish::tick).collect();
            fish.append(&mut new_fish);
        }

        assert_eq!(fish.len(), 383_160);
    }
}
