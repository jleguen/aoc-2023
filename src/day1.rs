use regex::Regex;

/// Split up input
#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|x| String::from(x)).collect()
}
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// ENTRY POINTS for cargo-aoc
// ---------------------------------------------------------------------------
#[aoc(day1, part1)]
pub fn part1(input: &Vec<String>) -> u32 {
    let mut res = 0;
    for line in input {
        let value: Vec<u32> = line.chars().filter_map(|x| x.to_digit(10)).collect();
        res += value.first().unwrap() * 10 + value.last().unwrap();
    }
    res
}

/// All patterns and replacements
/// Keep first and last letter to ensure overlapping digits are correctly found
const PATTERNS: [(&str, &str); 9] = [
    ("one", "o1e"),
    ("two", "t2o"),
    ("three", "t3e"),
    ("four", "f4r"),
    ("five", "f5e"),
    ("six", "s6x"),
    ("seven", "s7n"),
    ("eight", "e8t"),
    ("nine", "n9e"),
];

fn replace_digits(input: &String) -> String {
    let mut res: String = input.clone();
    for pat in PATTERNS {
        res = res.replace(pat.0, pat.1);
    }
    res
}

#[aoc(day1, part2)]
pub fn part2(input: &Vec<String>) -> u32 {
    let mut changed: Vec<String> = Vec::new();
    for line in input {
        changed.push(replace_digits(&line));
    }

    part1(&changed)
}

// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const INPUT2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_generator() {
        let i = input_generator(INPUT);
        assert_eq!(i.len(), 4);
    }
    #[test]
    fn test_part1() {
        let i = input_generator(INPUT);
        assert_eq!(142, part1(&i));
    }

    #[test]
    fn test_part2() {
        let i = input_generator(INPUT2);
        assert_eq!(281, part2(&i));
    }
}
