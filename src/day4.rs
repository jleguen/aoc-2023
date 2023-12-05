use std::str::FromStr;

/// List of numbers
#[derive(Debug, PartialEq, Eq)]
pub struct Numbers(Vec<u32>);

#[derive(Debug, PartialEq, Eq)]
pub struct NumbersError;

impl FromStr for Numbers {
    type Err = NumbersError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Numbers(
            s.trim()
                .split(' ')
                //.inspect(|x| println!("About to parse |{}|", x))
                .filter_map(|x| x.trim().parse::<u32>().ok())
                .collect::<Vec<u32>>(),
        ))
    }
}

/// A card has a index, and two lists of numbers
pub struct Card(u32, Numbers, Numbers);

impl Card {
    fn points(&self) -> u32 {
        let mut res = 0;
        for num in &self.2 .0 {
            if self.1 .0.contains(&num) {
                res = match res {
                    0 => 1,
                    i => 2 * i,
                };
            }
        }
        res
    }

    fn matching(&self) -> u32 {
        let mut res = 0;
        for num in &self.2 .0 {
            if self.1 .0.contains(&num) {
                res += 1;
            }
        }
        res
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CardError;

impl FromStr for Card {
    type Err = CardError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //println!("\n{}", s);
        let mut parts = s.split(":");
        let num = parts
            .next()
            .expect("Not enough parts")
            .trim()
            .split(' ')
            .last()
            .expect("no number")
            .parse::<u32>()
            .expect("no number");
        let mut numbers = parts.next().expect("Not enough parts").split("|");
        let winning = numbers
            .next()
            .expect("Not enough parts")
            .parse::<Numbers>()
            .expect("Not enough numbers");
        let have = numbers
            .next()
            .expect("Not enough parts")
            .parse::<Numbers>()
            .expect("Not enough numbers");
        Ok(Card(num, winning, have))
    }
}

/// Split up input
#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|x| x.parse::<Card>().expect("Not a card"))
        .collect()
}
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// ENTRY POINTS for cargo-aoc
// ---------------------------------------------------------------------------
#[aoc(day4, part1)]
pub fn part1(input: &Vec<Card>) -> u32 {
    input.iter().fold(0, |res, i| res + i.points())
}

#[aoc(day4, part2)]
pub fn part2(input: &Vec<Card>) -> u32 {
    // Vec of original card numbers
    let mut cards = Vec::with_capacity(input.len());
    for _ in 0..input.len() {
        cards.push(1);
    }
    for card in input {
        let p = card.matching();
        let index = card.0 - 1; // cards are 1-indexed
        print!(
            "card {} ({}) matching {} -> copy cards ",
            index, cards[index as usize], p
        );
        for i in index + 1..index + 1 + p {
            cards[i as usize] += cards[index as usize];
            print!("{}, ", i);
        }
        println!("");
    }
    cards.iter().sum()
}

// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_parse() {
        assert_eq!(23, "23".trim().parse::<u32>().expect("Number"));
        assert_eq!(1, " 1".trim().parse::<u32>().expect("Number"));
    }

    #[test]
    fn test_numbers() {
        assert_eq!(Numbers(Vec::from([1, 2, 3])), "1 2 3".parse().unwrap())
    }

    #[test]
    fn test_card() {
        let card = "Card 0: 1 2 3 | 4".parse::<Card>().unwrap();
        assert_eq!(0, card.points());
        let card = "Card 0: 1 2 3 | 2".parse::<Card>().unwrap();
        assert_eq!(1, card.points());
        let card = "Card 0: 1 2 3 | 2 1".parse::<Card>().unwrap();
        assert_eq!(2, card.points());
        let card = "Card 0: 1 2 3 | 2  3 1".parse::<Card>().unwrap();
        assert_eq!(4, card.points());
    }

    #[test]
    fn test_generator() {
        let i = input_generator(INPUT);
        assert_eq!(i.len(), 6);
    }
    #[test]
    fn test_part1() {
        let i = input_generator(INPUT);
        assert_eq!(13, part1(&i));
    }

    #[test]
    fn test_part2() {
        let i = input_generator(INPUT);
        assert_eq!(30, part2(&i));
    }
}
