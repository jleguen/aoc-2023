use std::str::FromStr;

// ---------------------------------------------------------------------------
#[derive(Debug)]
pub enum Op {
    Remove,
    Set(u32),
}

#[derive(Debug)]
pub struct Step {
    label: String,
    op: Op,
}

#[derive(Debug)]
pub struct ParseStepError;
impl FromStr for Step {
    type Err = ParseStepError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.ends_with('-') {
            return Ok(Step {
                label: s.replace("-", ""),
                op: Op::Remove,
            });
        }
        if s.contains("=") {
            if let Some((label, val)) = s.split_once("=") {
                return Ok(Step {
                    label: label.to_string(),
                    op: Op::Set(val.parse::<u32>().unwrap()),
                });
            }
        }
        return Err(ParseStepError);
    }
}

#[derive(Debug, Clone)]
pub struct Lense {
    pub label: String,
    pub focal: u32,
}

impl Lense {
    fn update(&mut self, value: u32) {
        self.focal = value;
    }
}

/// A box contains a list of lenses each with label and focal length
#[derive(Debug, Clone)]
pub struct Boite {
    pub lenses: Vec<Lense>,
}

impl Default for Boite {
    fn default() -> Self {
        Boite { lenses: Vec::new() }
    }
}

// ---------------------------------------------------------------------------
/// Split up input
#[aoc_generator(day15, part1)]
pub fn input_generator(input: &str) -> Vec<String> {
    input
        .replace("\n", "")
        .split(',')
        .map(|x| String::from(x))
        .collect()
}

#[aoc_generator(day15, part2)]
pub fn input_generator2(input: &str) -> Vec<Step> {
    input
        .replace("\n", "")
        .split(',')
        .map(|x| x.parse::<Step>().unwrap())
        .collect()
}

// ---------------------------------------------------------------------------
fn hash(input: &String) -> u32 {
    input
        .as_bytes()
        .iter()
        .fold(0, |acc: u32, &c| ((acc + c as u32) * 17) % 256) as u32
}

// ---------------------------------------------------------------------------
// ENTRY POINTS for cargo-aoc
// ---------------------------------------------------------------------------
#[aoc(day15, part1)]
pub fn part1(input: &Vec<String>) -> u32 {
    input.iter().map(|x| hash(x)).sum()
}

#[aoc(day15, part2)]
pub fn part2(input: &Vec<Step>) -> u32 {
    let mut res = 0;
    let mut boxes: Vec<Boite> = Vec::with_capacity(256);
    for _ in 0..256 {
        boxes.push(Boite::default());
    }

    // Insert lenses
    for step in input {
        let h = hash(&step.label) as usize;

        let mut it = boxes[h].lenses.iter();
        if let Some(index) = it.position(|x| x.label == step.label) {
            match step.op {
                Op::Remove => {
                    boxes[h].lenses.remove(index);
                }
                Op::Set(val) => {
                    boxes[h].lenses[index].update(val);
                }
            }
        } else {
            match step.op {
                Op::Remove => {
                    // Nothing
                }
                Op::Set(val) => {
                    boxes[h].lenses.push(Lense {
                        label: step.label.clone(),
                        focal: val,
                    });
                }
            }
        }
        println!("Step {:?} -> Box {} {:?}", step, h, boxes[h].lenses);
    }

    // Compute
    for (num, boite) in boxes.iter().enumerate() {
        for (slot, lense) in boite.lenses.iter().enumerate() {
            res += (num as u32 + 1) * (slot as u32 + 1) * lense.focal;
        }
    }

    res
}

// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_generator() {
        let i = input_generator(INPUT);
        assert_eq!(i.len(), 11);
    }

    #[test]
    fn test_hash() {
        assert_eq!(200, hash(&String::from("H")));
        assert_eq!(153, hash(&String::from("HA")));
        assert_eq!(52, hash(&String::from("HASH")));
    }
    #[test]
    fn test_part1() {
        let i = input_generator(INPUT);
        assert_eq!(1320, part1(&i));
    }

    #[test]
    fn test_part2() {
        let i = input_generator2(INPUT);
        assert_eq!(145, part2(&i));
    }
}
