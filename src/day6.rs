use std::iter::zip;
// ---------------------------------------------------------------------------
#[derive(Debug)]
pub struct Race {
    time: u64,
    record_dist: u64,
}

impl Race {
    const ACCEL: u64 = 1;

    pub fn new(time: u64, record_dist: u64) -> Self {
        Race { time, record_dist }
    }

    /// Compute distance travelled in total `time`  for a given `charging` time
    pub fn distance(time: u64, charging: u64) -> u64 {
        let speed = Race::ACCEL * charging;
        let race = time - charging;
        let dist = (time - charging) * charging;
        race * speed
    }

    pub fn algebra(&self) -> (u64, u64) {
        let inter = f64::sqrt((self.time * self.time - 4 * (self.record_dist)) as f64);
        print!("inter f64 {}", inter);
        let inter = inter as u64;
        println!(" | u64 {}", inter);
        let mut inf = (self.time - inter) / 2;
        // Ugly hack to ensure we go beyond the record distance
        while inf * (self.time - inf) <= self.record_dist {
            inf += 1;
        }
        let mut sup = (self.time + inter) / 2;
        while sup * (self.time - sup) <= self.record_dist {
            sup -= 1;
        }
        (inf, sup)
    }

    pub fn count(&self) -> u64 {
        let (inf, sup) = self.algebra();
        let count = sup - inf + 1;
        println!("{:?} -> inf {} sup {} -> count {}", self, inf, sup, count);
        count
    }
}

#[cfg(test)]
mod test_race {
    use super::*;

    #[test]
    fn test_algebra_and_count() {
        let r = Race::new(7, 9);
        let (inf, sup) = r.algebra();
        println!("{:?} {} {}", r, inf, sup);
        assert_eq!(2, inf);
        assert_eq!(5, sup);
        assert_eq!(4, r.count());
    }
}

// ---------------------------------------------------------------------------
/// Split up input
#[aoc_generator(day6, part1)]
pub fn input_generator(input: &str) -> Vec<Race> {
    let mut lines = input.lines();
    let time: Vec<u64> = lines
        .next()
        .expect("First line")
        .strip_prefix("Time:")
        .expect("Time")
        .split(' ')
        .filter_map(|x| x.trim().parse::<u64>().ok())
        .collect();
    let dist: Vec<u64> = lines
        .next()
        .expect("Second line")
        .strip_prefix("Distance:")
        .expect("Distance")
        .split(' ')
        .filter_map(|x| x.trim().parse::<u64>().ok())
        .collect();
    zip(time, dist).map(|(t, d)| Race::new(t, d)).collect()
}
// ---------------------------------------------------------------------------

#[aoc_generator(day6, part2)]
pub fn input_generator2(input: &str) -> Race {
    let mut lines = input.lines();
    let time: u64 = lines
        .next()
        .expect("First line")
        .strip_prefix("Time:")
        .expect("Time")
        .replace(" ", "")
        .parse::<u64>()
        .expect("Not a number ?");
    let dist: u64 = lines
        .next()
        .expect("Second line")
        .strip_prefix("Distance:")
        .expect("Distance")
        .replace(" ", "")
        .parse::<u64>()
        .expect("Not a number ?");
    Race::new(time, dist)
}

// ---------------------------------------------------------------------------
// ENTRY POINTS for cargo-aoc
// ---------------------------------------------------------------------------
#[aoc(day6, part1)]
pub fn part1(input: &Vec<Race>) -> u64 {
    input.iter().map(|x| x.count()).product()
}

#[aoc(day6, part2)]
pub fn part2(input: &Race) -> u64 {
    input.count()
}

// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_generator() {
        let i = input_generator(INPUT);
        assert_eq!(i.len(), 3);
    }
    #[test]
    fn test_part1() {
        let i = input_generator(INPUT);
        assert_eq!(288, part1(&i));
    }

    #[test]
    fn test_part2() {
        let i = input_generator2(INPUT);
        assert_eq!(71503, part2(&i));
    }
}
