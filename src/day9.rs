trait Oasis {
    fn zero(&self) -> bool;
    fn diff(&self) -> Self;
    fn extrapolate(&self) -> i64;
    fn baxpolate(&self) -> i64;
}
type History = Vec<i64>;

impl Oasis for History {
    fn zero(&self) -> bool {
        self.iter().all(|&v| v == 0)
    }

    fn diff(&self) -> Self {
        //println!("Diff {:?}", self);
        self.windows(2).map(|w| w[1] - w[0]).collect()
    }

    fn extrapolate(&self) -> i64 {
        println!("Extrapolate {:?}", self);
        if self.zero() {
            return 0;
        } else {
            // Compute diff
            let d = self.diff();
            let new_d = d.extrapolate();
            // Compute new element
            return self.last().unwrap() + new_d;
        }
    }

    fn baxpolate(&self) -> i64 {
        println!("Baxpolate {:?}", self);
        if self.zero() {
            return 0;
        } else {
            // Compute diff
            let d = self.diff();
            let new_d = d.baxpolate();
            // Compute new element
            return self.first().unwrap() - new_d;
        }
    }
}

/// Split up input
#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<History> {
    input
        .lines()
        .inspect(|l| println!("\n Line {}", l))
        .map(|x| {
            x.split(' ')
                .inspect(|v| print!("{}", v))
                .map(|v| v.trim().parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// ENTRY POINTS for cargo-aoc
// ---------------------------------------------------------------------------
#[aoc(day9, part1)]
pub fn part1(input: &Vec<History>) -> i64 {
    input.iter().fold(0, |acc, h| acc + h.extrapolate())
}

#[aoc(day9, part2)]
pub fn part2(input: &Vec<History>) -> i64 {
    input.iter().fold(0, |acc, h| acc + h.baxpolate())
}

// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_generator() {
        let i = input_generator(INPUT);
        assert_eq!(i.len(), 3);
    }

    #[test]
    fn test_oasis() {
        let h = History::from([0, 3, 6, 9, 12, 15]);
        assert_eq!(18, h.extrapolate());
    }

    #[test]
    fn test_part1() {
        let i = input_generator(INPUT);
        assert_eq!(114, part1(&i));
    }

    #[test]
    fn test_part2() {
        let i = input_generator(INPUT);
        assert_eq!(2, part2(&i));
    }
}
