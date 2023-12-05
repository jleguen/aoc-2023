/// Split up input
#[aoc_generator(dayN)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|x| String::from(x)).collect()
}
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// ENTRY POINTS for cargo-aoc
// ---------------------------------------------------------------------------
#[aoc(dayN, part1)]
pub fn part1(input: &Vec<String>) -> u32 {
    let mut res = 0;
    // TODO
    res
}

#[aoc(dayN, part2)]
pub fn part2(input: &Vec<String>) -> u32 {
    let mut res = 0;
    // TODO
    res
}

// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "";

    #[test]
    fn test_generator() {
        let i = input_generator(INPUT);
        assert_eq!(i.len(), 4);
    }
    #[test]
    fn test_part1() {
        let i = input_generator(INPUT);
        assert_eq!(1, part1(&i));
    }

    #[test]
    fn test_part2() {
        let i = input_generator(INPUT);
        assert_eq!(2, part2(&i));
    }
}
