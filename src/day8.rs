use itertools::Itertools;
use parse_display::{Display, FromStr};
use std::collections::HashMap;
// ---------------------------------------------------------------------------
type State = String;
type Network = HashMap<State, (State, State)>;

#[derive(FromStr, Display, Debug)]
pub enum Instruction {
    #[display("L")]
    Left,
    #[display("R")]
    Right,
}

#[derive(Debug)]
pub struct Map {
    instructions: Vec<Instruction>,
    network: Network,
}

impl Map {
    const START: &str = "AAA";
    const END: &str = "ZZZ";

    pub fn from(inst: &str, net: &str) -> Self {
        let i: Vec<Instruction> = inst
            .split("")
            .filter(|c| !c.is_empty())
            .map(|c| c.parse::<Instruction>().unwrap())
            .collect();

        let mut n = HashMap::new();
        for line in net.lines() {
            let (from, left, right) = line
                .split(&['=', '(', ',', ')'])
                .map(|x| String::from(x.trim()))
                .filter(|x| !x.is_empty())
                .collect_tuple()
                .unwrap();
            n.insert(from, (left, right));
        }

        Map {
            instructions: i,
            network: n,
        }
    }

    pub fn next(state: &State, inst: &Instruction, net: &Network) -> State {
        let v = net.get(state).unwrap();
        match inst {
            Instruction::Left => v.0.clone(),
            Instruction::Right => v.1.clone(),
        }
    }

    pub fn count(&self, start: &State) -> u32 {
        let mut res: u32 = 0;
        let mut next = start.clone();
        for inst in self.instructions.iter().cycle() {
            res += 1;
            next = Map::next(&next, &inst, &self.network);
            if next == Map::END.to_string() {
                return res;
            }
        }
        0
    }

    pub fn part2(&self) -> u32 {
        let mut res: u32 = 0;
        let mut start: Vec<State> = self
            .network
            .keys()
            .filter(|k| k.ends_with('A'))
            .map(|k| k.clone())
            .collect();
        0
    }
}

// ---------------------------------------------------------------------------
/// Split up input
#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Map {
    let (inst, net) = input.split("\n\n").collect_tuple().unwrap();
    Map::from(inst, net)
}
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// ENTRY POINTS for cargo-aoc
// ---------------------------------------------------------------------------
#[aoc(day8, part1)]
pub fn part1(input: &Map) -> u32 {
    println!("{:?}", input);
    input.count(&Map::START.to_string())
}

#[aoc(day8, part2)]
pub fn part2(input: &Map) -> u32 {
    input.part2()
}

// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const INPUT2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const INPUT3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_generator() {
        let i = input_generator(INPUT);
    }
    #[test]
    fn test_part1() {
        let i = input_generator(INPUT);
        assert_eq!(2, part1(&i));
        let i = input_generator(INPUT2);
        assert_eq!(6, part1(&i));
    }

    #[test]
    fn test_part2() {
        let i = input_generator(INPUT3);
        assert_eq!(6, part2(&i));
    }
}
