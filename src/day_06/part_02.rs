use super::{EXAMPLE_INPUT, PUZZLE_INPUT};

struct School {
    adults: [u64; 7],
    juveniles: [u64; 9],
}

impl School {
    fn new(timers: Vec<usize>) -> Self {
        let mut adults = [0; 7];
        for timer in timers {
            adults[timer] += 1;
        }

        Self {
            adults,
            juveniles: [0; 9],
        }
    }

    fn spawn(&mut self, day: usize) {
        let adult_group = day % 7;
        let juvenile_group = day % 9;

        let new_adults = self.juveniles[juvenile_group];
        let new_juveniles = self.adults[adult_group] + self.juveniles[juvenile_group];

        self.juveniles[juvenile_group] = new_juveniles;
        self.adults[adult_group] += new_adults;
    }

    fn total_fish(&self) -> u64 {
        self.adults.iter().chain(self.juveniles.iter()).sum()
    }
}

fn parse_timers(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .map(|timer| timer.parse().expect("must be valid usize"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let timers = parse_timers(EXAMPLE_INPUT);
        let mut school = School::new(timers);

        for day in 0..18 {
            school.spawn(day);
        }

        assert_eq!(school.total_fish(), 26);

        for day in 18..80 {
            school.spawn(day);
        }

        assert_eq!(school.total_fish(), 5934);

        for day in 80..256 {
            school.spawn(day);
        }

        assert_eq!(school.total_fish(), 26_984_457_539);
    }

    #[test]
    fn puzzle() {
        let timers = parse_timers(PUZZLE_INPUT);
        let mut school = School::new(timers);

        for day in 0..256 {
            school.spawn(day);
        }

        assert_eq!(school.total_fish(), 1_721_148_811_504);
    }
}
