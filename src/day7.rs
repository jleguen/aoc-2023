use itertools::enumerate;
use parse_display::{Display, FromStr};
// ---------------------------------------------------------------------------
type Kind = u8;

#[derive(Debug, Clone)]
pub struct Hand {
    pub cards: [u8; 5],
    pub bid: u32,
    pub kind: Kind,
}

impl Hand {
    pub fn value(c: &char) -> u8 {
        match c {
            'J' => 1, // was 11 for part1
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => {
                panic!("Unknown card {}", c);
            }
        }
    }

    pub fn determine_kind(cards: &[u8; 5]) -> Kind {
        let mut count: Vec<u8> = vec![0; 15];
        for c in cards.iter() {
            count[*c as usize] += 1;
        }
        let joker = count[1];
        let len = count.len();
        let mut hand = &mut count[2..len];
        hand.sort();
        if let Some(last) = hand.last_mut() {
            *last += joker;
        }
        count = hand.iter().filter(|&x| x != &0).map(|&x| x).collect();
        match count.as_slice() {
            [5] => 6,
            [1, 4] => 5,
            [2, 3] => 4,
            [1, 1, 3] => 3,
            [1, 2, 2] => 2,
            [1, 1, 1, 2] => 1,
            [1, 1, 1, 1, 1] => 0,
            _ => panic!("Unknown Kind"),
        }
    }

    pub fn from(s: &str) -> Self {
        let (cards, bid) = s.split_once(' ').unwrap();
        let cards = cards
            .chars()
            .map(|c| Hand::value(&c))
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();

        Hand {
            cards,
            bid: bid.parse::<u32>().unwrap(),
            kind: Hand::determine_kind(&cards),
        }
    }
}

mod test_hand {
    use super::*;
    #[test]
    fn test_from_str() {
        let h = Hand::from("32T3K 765");
        assert_eq!(765, h.bid);
        assert_eq!([3, 2, 10, 3, 13], h.cards);
        assert_eq!(1, h.kind);
    }
}

// ---------------------------------------------------------------------------
/// Split up input
#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<Hand> {
    input.lines().map(|x| Hand::from(x)).collect()
}
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// ENTRY POINTS for cargo-aoc
// ---------------------------------------------------------------------------
#[aoc(day7, part1)]
pub fn part1(input: &Vec<Hand>) -> u32 {
    let mut res: u32 = 0;
    let mut hands: Vec<Hand> = input.clone();
    hands.sort_by(|a, b| {
        if a.kind == b.kind {
            a.cards.cmp(&b.cards)
        } else {
            a.kind.cmp(&b.kind)
        }
    });
    println!("{:?}", hands);
    for (rank, hand) in enumerate(hands) {
        res += (rank as u32 + 1) * hand.bid;
    }
    res
}

#[aoc(day7, part2)]
pub fn part2(input: &Vec<Hand>) -> u32 {
    part1(input)
}

// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_generator() {
        let i = input_generator(INPUT);
        assert_eq!(i.len(), 5);
    }
    #[test]
    fn test_part1() {
        let i = input_generator(INPUT);
        assert_eq!(6440, part1(&i));
    }

    #[test]
    fn test_part2() {
        let i = input_generator(INPUT);
        assert_eq!(5905, part2(&i));
    }
}
