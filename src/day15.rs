/// Split up input
#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<String> {
    input
        .replace("\n", "")
        .split(',')
        .map(|x| String::from(x))
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
pub fn part2(input: &Vec<String>) -> u32 {
    let mut res = 0;
    // TODO
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
        let i = input_generator(INPUT);
        assert_eq!(2, part2(&i));
    }
}
