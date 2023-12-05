/// Split up input
#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|x| String::from(x)).collect()
}
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// ENTRY POINTS for cargo-aoc
// ---------------------------------------------------------------------------
#[aoc(day3, part1)]
pub fn part1(input: &Vec<String>) -> u32 {
    0
}

#[aoc(day3, part2)]
pub fn part2(input: &Vec<String>) -> u32 {
    0
}

// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_generator() {
        let i = input_generator(INPUT);
        assert_eq!(i.len(), 4);
    }
    #[test]
    fn test_part1() {
        let i = input_generator(INPUT);
        assert_eq!(4361, part1(&i));
    }

    #[test]
    fn test_part2() {
        let i = input_generator(INPUT);
        assert_eq!(281, part2(&i));
    }
}
