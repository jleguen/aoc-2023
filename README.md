# [Advent of Code 2023](https://adventofcode.com/2023/)

View [Kaizen LeaderBoard](https://adventofcode.com/2023/leaderboard/private/view/796831).


## Cargo AoC
This project uses [`cargo-aoc`](https://github.com/gobanos/cargo-aoc) to simplify the interactions with the server.

```
cargo install cargo-aoc
```

### Run
```
cargo aoc
```

## Resolution

### [Day 1](https://adventofcode.com/2023/day/1)
Replace all textual occurences by the digits, but be careful to keep first and last letter to ensure overlapping digits are found.

### [Day 2](https://adventofcode.com/2023/day/2)
Don't try to be smart and simply split and brute-parse the input. Use custom types `Draw` and `Game` to clean up the inner loops.

### [Day 3](https://adventofcode.com/2023/day/3)
Use `toodee` and some very ugly matrix traversal.

### [Day 4](https://adventofcode.com/2023/day/4)
Don't try to be smart and simply split and brute-parse the input. Use custom types `Numbers` and `Card` to simplify inner loops, as always.

### [Day 5](https://adventofcode.com/2023/day/5)
Use a sparse Range representation to avoid storing too many numbers.

For part 2, some memoisation could help accelerate the computations.

### [Day 6](https://adventofcode.com/2023/day/6)
We know the second part usually uses much bigger numbers than the first one.
Directly use algebra instead of brute force :)
### [Day 7](https://adventofcode.com/2023/day/7)
Quick and dirty, but broke part1 to be able to pass part2.

### [Day 8](https://adventofcode.com/2023/day/8)
* Part1 : Brute force, simply follow the network
* Part2 : Might need cycle detection or smarter things...

