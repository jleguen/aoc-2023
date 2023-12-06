use toodee::{Coordinate, TooDee, TooDeeOps};
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Content {
    Number(u32),
    Symbol(char),
    Empty,
}

impl Content {
    pub fn new(c: &char) -> Self {
        match c {
            v @ '0'..='9' => Content::Number(v.to_digit(10).unwrap()),
            '.' => Content::Empty,
            _x => Content::Symbol(*_x),
        }
    }
}

impl Default for Content {
    fn default() -> Self {
        Content::Empty
    }
}

#[cfg(test)]
mod test_content {
    use super::*;
    #[test]
    fn create_content() {
        assert_eq!(Content::new(&'0'), Content::Number(0));
        assert_eq!(Content::new(&'9'), Content::Number(9));
        assert_eq!(Content::new(&'.'), Content::Empty);
        assert_eq!(Content::new(&'#'), Content::Symbol('#'));
    }
}

// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Number {
    Part(usize),
    Unknown(usize),
}
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Eq)]
pub struct Plan {
    pub schematic: TooDee<Content>,
}

impl Plan {
    pub fn new(input: &str) -> Self {
        let mut _map = TooDee::new(0, 0);
        // Convert to Content cells
        for line in input.lines() {
            _map.push_row(
                line.chars()
                    .map(|x| Content::new(&x))
                    .collect::<Vec<Content>>(),
            )
        }
        // Add an Empty border to ease computations
        _map.insert_row(0, vec![Content::Empty; _map.num_cols()]);
        _map.push_row(vec![Content::Empty; _map.num_cols()]);
        _map.insert_col(0, vec![Content::Empty; _map.num_rows()]);
        _map.push_col(vec![Content::Empty; _map.num_rows()]);

        Plan { schematic: _map }
    }

    // Extract one Number starting at a coordinate which contains a Content::Number
    // We suppose the cell directly left is not part of the number
    fn extract_one_number(&self, at: &Coordinate) -> Option<(Number, Coordinate)> {
        // Starting point
        let (col, row) = at;
        let part: bool = false;
        if let Content::Number(n) = self.schematic[*at] {}
        None
    }

    /// Extract all numbers (Parts or Unknown) from schematic
    pub fn numbers(&self) -> Vec<Number> {
        // We start at 1,1.
    }
}

// ---------------------------------------------------------------------------
/// Split up input
#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Plan {
    Plan::new(input)
}
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// ENTRY POINTS for cargo-aoc
// ---------------------------------------------------------------------------
#[aoc(day3, part1)]
pub fn part1(input: &Plan) -> u32 {
    0
}

#[aoc(day3, part2)]
pub fn part2(input: &Plan) -> u32 {
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
        assert_eq!(i.schematic.num_rows(), 12);
        assert_eq!(i.schematic.num_cols(), 12);
    }

    #[test]
    fn test_part1() {
        let i = input_generator(INPUT);
        assert_eq!(4361, part1(&i));
    }

    #[test]
    fn test_part2() {
        let i = input_generator(INPUT);
        assert_eq!(2, part2(&i));
    }
}
