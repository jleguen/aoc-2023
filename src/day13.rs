use std::str::FromStr;

// ---------------------------------------------------------------------------
/// We store the pattern into integers for easy comparison
/// - Vector of integers for each row
/// - Vector of integers for each col
///
#[derive(Debug)]
pub struct Pattern {
    pub cols: Vec<u64>,
    pub rows: Vec<u64>,
}

#[derive(Debug)]
pub struct ParsePatternError;
impl FromStr for Pattern {
    type Err = ParsePatternError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rows = Vec::new();
        let mut cols = Vec::new();

        for (r, line) in s.lines().enumerate() {
            let mut rval: u64 = 0;
            for (c, symbol) in line.chars().enumerate() {
                let val = match symbol {
                    '.' => 0,
                    '#' => 1,
                    _ => panic!("Unknown symbol"),
                };
                // Construct row value
                rval |= val << c;
                //
                // Update col value
                if cols.len() > c {
                    let cval: &mut u64 = cols.get_mut(c).unwrap();
                    *cval |= val << r;
                } else {
                    cols.push(val << r);
                }
            }
            rows.push(rval);
        }
        println!("\nRows {:?}", rows);
        println!("Cols {:?}", cols);

        Ok(Self { cols, rows })
    }
}

impl Pattern {
    //
    fn first_symmetry(input: &Vec<u64>) -> u64 {
        println!("\n--------------");
        println!("TOP {:?}", input);
        for i in 0..input.len() {
            let left = i + 1;
            let right = input.len() - left;
            let mut count = 0;
            println!("i {} left {} right {}", i, left, right);
            for j in 0..usize::min(left, right) {
                println!("i {} j {}", i, j);
                if input[i - j] == input[i + j + 1] {
                    count += 1;
                }
            }
            println!("i {} -> count {}\n", i, count);
            if count > 0 && count >= usize::min(left, right) {
                return (i + 1) as u64;
            }
        }
        0
    }
}

// ---------------------------------------------------------------------------
/// Split up input
#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Vec<Pattern> {
    input
        .split("\n\n")
        .map(|x| x.parse::<Pattern>().unwrap())
        .collect()
}
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// ENTRY POINTS for cargo-aoc
// ---------------------------------------------------------------------------
#[aoc(day13, part1)]
pub fn part1(input: &Vec<Pattern>) -> u64 {
    let mut res: u64 = 0;
    for pattern in input {
        let left = Pattern::first_symmetry(&pattern.cols);
        let top = Pattern::first_symmetry(&pattern.rows);
        println!("{:?} LEFT {} TOP {}", pattern, left, top);
        res += left;
        res += 100 * top;
    }
    res
}

#[aoc(day13, part2)]
pub fn part2(input: &Vec<Pattern>) -> u64 {
    let mut res = 0;
    // TODO
    res
}

// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_generator() {
        let i = input_generator(INPUT);
        assert_eq!(i.len(), 2);
        assert_eq!(9, i[0].cols.len());
        assert_eq!(7, i[0].rows.len());
        assert_eq!(9, i[1].cols.len());
        assert_eq!(7, i[1].rows.len());

        assert_eq!(4, Pattern::first_symmetry(&i[1].rows));
        assert_eq!(5, Pattern::first_symmetry(&i[0].cols));
    }
    #[test]
    fn test_part1() {
        let i = input_generator(INPUT);
        assert_eq!(405, part1(&i));
    }

    #[test]
    fn test_part2() {
        let i = input_generator(INPUT);
        assert_eq!(2, part2(&i));
    }
}
