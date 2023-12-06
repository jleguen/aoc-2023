use std::collections::HashMap;
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

    pub fn get_number(&self) -> u32 {
        match *self {
            Content::Number(v) => v,
            _ => {
                panic!("Content is not a number");
            }
        }
    }

    pub fn is_symbol(&self) -> bool {
        match *self {
            Content::Symbol(_) => true,
            _ => false,
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
    Part(u32),
    Unknown(u32),
    // Assumption : Part is only touching one gear only!
    GearPart(u32, Coordinate),
}

impl Number {
    pub fn to_part(&self) -> Number {
        match *self {
            Number::Part(v) => Number::Part(v),
            Number::Unknown(v) => Number::Part(v),
            Number::GearPart(v, pos) => Number::GearPart(v, pos),
        }
    }

    pub fn add_digit(&self, digit: u32) -> Number {
        match *self {
            Number::Part(v) => Number::Part(v * 10 + digit),
            Number::Unknown(v) => Number::Unknown(v * 10 + digit),
            Number::GearPart(v, pos) => Number::GearPart(v * 10 + digit, pos),
        }
    }

    pub fn part_number(&self) -> u32 {
        match *self {
            Number::Part(v) => v,
            Number::GearPart(v, _) => v,
            _ => 0,
        }
    }

    pub fn is_gear_part(&self) -> bool {
        match *self {
            Number::GearPart(_, _) => true,
            _ => false,
        }
    }

    pub fn to_gear_part(&self, pos: Coordinate) -> Number {
        match *self {
            Number::Part(v) => Number::GearPart(v, pos),
            Number::Unknown(v) => Number::GearPart(v, pos),
            Number::GearPart(_, p) => {
                if p != pos {
                    panic!("Part {:?} already part of a gear at pos {:?}", self, p);
                } else {
                    *self
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Eq)]
pub struct Plan {
    pub map: TooDee<Content>,
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

        Plan { map: _map }
    }

    /// Check if current should be transformed to part
    fn check_part(&self, current: &Option<Number>, adj: &Vec<Coordinate>) -> Option<Number> {
        if let Some(num) = current {
            for &pos in adj {
                match self.map[pos] {
                    Content::Symbol('*') => {
                        return Some(num.to_gear_part(pos));
                    }
                    Content::Symbol(_) => {
                        return Some(num.to_part());
                    }
                    _ => {}
                }
            }
            Some(*num)
        } else {
            None
        }
    }

    /// Extract all numbers (Parts or Unknown) from map
    pub fn numbers(&self) -> Vec<Number> {
        let mut res: Vec<Number> = Vec::new();
        // We start at 1,1.
        let mut current: Option<Number> = None;
        // Scan each line for digits.
        // Create numbers digit by digit, and check adjacent cells for symbols
        for row in 1..self.map.num_rows() - 1 {
            for col in 1..self.map.num_cols() - 1 {
                let pos: Coordinate = (col, row);
                // Check adjacent cells for part modifier
                let adj = Vec::from([(col, row - 1), (col, row + 1)]);
                current = self.check_part(&current, &adj);

                // Check current cell
                match self.map[pos] {
                    Content::Number(digit) => {
                        if let Some(num) = current {
                            current = Some(num.add_digit(digit));
                        } else {
                            current = Some(Number::Unknown(digit));
                            // check adjacent cells on the left
                            //println!("Pos {:?} -- New number", pos);
                            let adj = Vec::from([
                                (col - 1, row - 1),
                                (col - 1, row),
                                (col - 1, row + 1),
                                (col, row - 1),
                                (col, row + 1),
                            ]);
                            current = self.check_part(&current, &adj);
                        }
                    }
                    Content::Empty => {
                        if current.is_some() {
                            // check adjacent cells on top and bottom
                            let adj = Vec::from([(col, row - 1), (col, row + 1)]);
                            current = self.check_part(&current, &adj);
                            res.push(current.unwrap());
                            current = None;
                        }
                    }
                    Content::Symbol('*') => {
                        if let Some(num) = current {
                            res.push(num.to_gear_part(pos));
                            current = None;
                        }
                    }
                    Content::Symbol(_) => {
                        if let Some(num) = current {
                            res.push(num.to_part());
                            current = None;
                        }
                    }
                } // match self.map[pos]
            }
        }
        res
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
    let num = input.numbers();
    println!("{:?}", num);
    num.iter().map(|x| x.part_number()).sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &Plan) -> u32 {
    let mut gears = HashMap::new();
    let num = input.numbers();
    let gear_parts: Vec<_> = num.iter().filter(|x| x.is_gear_part()).collect();

    for part in gear_parts.iter() {
        if let Number::GearPart(num, pos) = part {
            gears.entry(pos).or_insert(Vec::new()).push(*num);
            /*
            println!(
                "num {} pos {:?} gear {:?}",
                num,
                pos,
                gears.get(pos).unwrap()
            );
            */
        }
    }

    println!("{:?}", gears);
    let total = gears
        .values()
        .filter(|x| x.len() == 2)
        .inspect(|x| println!("{:?}", x))
        .map(|x| x.iter().product::<u32>())
        .sum();
    total
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
        assert_eq!(i.map.num_rows(), 12);
        assert_eq!(i.map.num_cols(), 12);
    }

    #[test]
    fn test_part1() {
        let i = input_generator(INPUT);
        assert_eq!(4361, part1(&i));
    }

    #[test]
    fn test_part2() {
        let i = input_generator(INPUT);
        assert_eq!(467835, part2(&i));
    }
}
