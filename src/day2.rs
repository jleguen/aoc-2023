use itertools::Itertools;
use std::cmp::max;

#[derive(Default, Debug, PartialEq)]
pub struct Draw(u32, u32, u32);

impl Draw {
    fn valid(&self, other: &Draw) -> bool {
        self.0 <= other.0 && self.1 <= other.1 && self.2 <= other.2
    }

    fn power(&self) -> u32 {
        self.0 * self.1 * self.2
    }

    fn minimum_set(draws: &Vec<Draw>) -> Draw {
        let mut res = Draw(0, 0, 0);
        for draw in draws {
            res.0 = max(res.0, draw.0);
            res.1 = max(res.1, draw.1);
            res.2 = max(res.2, draw.2);
        }
        res
    }
}

#[derive(Debug)]
pub struct Game(u32, Vec<Draw>);

/// Create a Draw from a str
/// assert_eq!(Draw(1,2,3), "1 red, 2 green, 3 blue");
fn create_draw(input: &str) -> Draw {
    //println!("Draw {:?}", input);
    let mut draw = Draw::default();
    for d in input.split(", ") {
        let (num, color) = d.split(" ").collect_tuple().expect("Not a draw");
        let num = num.parse().expect("Not a number");
        match color {
            "red" => draw.0 = num,
            "green" => draw.1 = num,
            "blue" => draw.2 = num,
            _ => panic!(),
        }
    }
    draw
}

fn create_game(input: &str) -> Game {
    let mut draws: Vec<Draw> = Vec::new();
    // Extract Game number and list of draws
    let (game, dr) = input
        .split(": ")
        .collect_tuple()
        .expect("Unable to split line");
    let game: u32 = game
        .split(" ")
        .last()
        .unwrap()
        .parse()
        .expect("Not a Game Number");
    for draw in dr.split("; ") {
        draws.push(create_draw(draw));
    }
    Game(game, draws)
}

/// Split up input
#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Game> {
    //input.lines().map(|x| x.split(":")
    let mut res: Vec<Game> = Vec::new();

    // Each line is a Game
    for line in input.lines() {
        //println!("Line |{}|", line);
        let g = create_game(line);
        //println!("{:?}", g);
        res.push(g);
    }

    res
}
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// ENTRY POINTS for cargo-aoc
// ---------------------------------------------------------------------------
#[aoc(day2, part1)]
pub fn part1(input: &Vec<Game>) -> u32 {
    // 12 red, 13 green, 14 blue
    const MAX: Draw = Draw(12, 13, 14);
    let mut res: u32 = 0;

    'outer: for game in input {
        for draw in &game.1 {
            if !(*draw).valid(&MAX) {
                //println!("Game {} : {:?} > MAX {:?}", game.0, draw, MAX);
                continue 'outer;
            }
        }
        res += game.0;
        //println!("{:?} is valid => res {}\n", game, res);
    }
    res
}

#[aoc(day2, part2)]
pub fn part2(input: &Vec<Game>) -> u32 {
    let mut res = 0;
    for game in input {
        let min = Draw::minimum_set(&game.1);
        res += min.power();
    }
    res
}

// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_create_draw() {
        assert_eq!(Draw(1, 0, 0), create_draw("1 red"));
        assert_eq!(Draw(0, 1, 0), create_draw("1 green"));
        assert_eq!(Draw(0, 0, 1), create_draw("1 blue"));
        assert_eq!(Draw(1, 2, 3), create_draw("1 red, 2 green, 3 blue"));
    }

    #[test]
    fn test_generator() {
        let i = input_generator(INPUT);
        assert_eq!(i.len(), 5);
    }
    #[test]
    fn test_part1() {
        let i = input_generator(INPUT);
        assert_eq!(8, part1(&i));
    }

    #[test]
    fn test_part2() {
        let i = input_generator(INPUT);
        assert_eq!(2286, part2(&i));
    }
}
