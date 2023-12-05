use itertools::Itertools;
use parse_display::{Display, FromStr};

// ---------------------------------------------------------------------------
/// we use a sparse representation of a Range
/// due to the size of the numbers in the full problem.
///
/// ```
/// use aoc_2023::day5::Range;
/// let range = Range { source: 0, dest: 10, len: 5};
/// assert_eq!(Some(12), range.get(2));
/// ```
///
#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{dest} {source} {len}")]
pub struct Range {
    pub dest: usize,
    pub source: usize,
    pub len: usize,
}

impl Range {
    /// Get mapping corresponding to `value`, or None
    pub fn get(&self, value: usize) -> Option<usize> {
        // If value is in [source, source+len] we can return the mapping.
        // Otherwise, we return None
        let res = if value >= self.source && value < self.source + self.len {
            let offset = value - self.source;
            Some(self.dest + offset)
        } else {
            None
        };
        res
    }
}

#[cfg(test)]
mod range_tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let r = "1 2 3".parse::<Range>().expect("Not a range?");
        assert_eq!(
            Range {
                dest: 1,
                source: 2,
                len: 3
            },
            r
        );
    }

    #[test]
    fn test_out_of_range() {
        let r = Range {
            dest: 10,
            source: 0,
            len: 5,
        };
        assert_eq!(None, r.get(10));
        assert_eq!(None, r.get(5));
    }

    #[test]
    fn test_in_range() {
        let r = Range {
            dest: 10,
            source: 0,
            len: 5,
        };
        assert_eq!(Some(12), r.get(2));
    }
}

pub struct Mapping {
    ranges: Vec<Range>,
}

impl Mapping {
    pub fn get(&self, value: usize) -> usize {
        for range in &self.ranges {
            if let Some(dest) = range.get(value) {
                return dest;
            }
        }
        value
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseMappingError;

impl std::str::FromStr for Mapping {
    type Err = ParseMappingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Mapping {
            ranges: s
                .lines()
                .filter_map(|line| line.parse::<Range>().ok())
                .collect::<Vec<Range>>(),
        })
    }
}

#[cfg(test)]
mod mapping_tests {
    use super::*;

    const INPUT: &str = "0 1 5
10 11 5";

    #[test]
    fn test_mapping() {
        let m = Mapping {
            // Simple mapping :
            // 0 -> 0
            //
            // 1 -> 0
            // 2 -> 1
            // ..
            // 5 -> 4
            //
            // 6 -> 6
            // ..
            // 10 -> 10
            //
            // 11 -> 10
            // ..
            // 15 -> 14
            ranges: Vec::from([
                "0 1 5".parse::<Range>().unwrap(),
                "10 11 5".parse::<Range>().unwrap(),
            ]),
        };

        assert_eq!(0, m.get(0));
        assert_eq!(0, m.get(1));
        assert_eq!(4, m.get(5));
        assert_eq!(6, m.get(6));
        assert_eq!(10, m.get(10));
        assert_eq!(10, m.get(11));
        assert_eq!(16, m.get(16));
    }

    #[test]
    fn test_parse() {
        let m = INPUT.parse::<Mapping>().unwrap();
        assert_eq!(4, m.get(5));
    }
}

pub struct Almanach {
    pub seeds: Vec<usize>,
    pub maps: Vec<Mapping>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseAlmanachError;

impl std::str::FromStr for Almanach {
    type Err = ParseAlmanachError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut blocks = s.split("\n\n");
        let seeds: Vec<usize> = blocks
            .next()
            .expect("Not a Seeds block?")
            .split(":")
            .last()
            .expect("Not a Seeds line?")
            .trim()
            .split(" ")
            .filter_map(|x| x.parse::<usize>().ok())
            .collect();
        println!("{:?}", seeds);

        let maps: Vec<Mapping> = blocks
            .map(|b| {
                b.split(":")
                    .last()
                    .expect("Not a block?")
                    .parse::<Mapping>()
                    .expect("Not a mapping?")
            })
            .collect();
        Ok(Almanach { seeds, maps })
    }
}

impl Almanach {
    pub fn get(&self, value: usize) -> usize {
        let mut res = value;
        for map in &self.maps {
            res = map.get(res);
        }
        res
    }

    pub fn lowest_location(&self, seeds: &Vec<usize>) -> usize {
        seeds.iter().map(|s| self.get(*s)).min().unwrap()
    }

    pub fn part1(&self) -> usize {
        self.lowest_location(&self.seeds)
    }

    pub fn part2(&self) -> usize {
        let mut lowest: usize = usize::MAX;

        for (start, len) in self
            .seeds
            .chunks(2)
            .inspect(|x| println!("{:?}", x))
            .map(|x| (x[0], x[1]))
            .collect::<Vec<(usize, usize)>>()
        {
            for value in start..start + len {
                lowest = std::cmp::min(lowest, self.get(value));
            }
            println!("Start {start} Len {len} Lowest {lowest}");
        }

        lowest
    }
}

// ---------------------------------------------------------------------------
/// Split up input
#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Almanach {
    input.parse::<Almanach>().expect("Not an almanach")
}
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// ENTRY POINTS for cargo-aoc
// ---------------------------------------------------------------------------
#[aoc(day5, part1)]
pub fn part1(input: &Almanach) -> usize {
    input.part1()
}

#[aoc(day5, part2)]
pub fn part2(input: &Almanach) -> usize {
    input.part2()
}

// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_generator() {
        let a = input_generator(INPUT);
        assert_eq!(a.seeds.len(), 4);
        assert_eq!(vec![79, 14, 55, 13], a.seeds);
        assert_eq!(a.maps.len(), 7);
    }
    #[test]
    fn test_part1() {
        let i = input_generator(INPUT);
        assert_eq!(35, part1(&i));
    }

    #[test]
    fn test_part2() {
        let i = input_generator(INPUT);
        assert_eq!(46, part2(&i));
    }
}
