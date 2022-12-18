use super::day::Day;
use ahash::HashSetExt;
use anyhow::Result;
use rustc_hash::FxHashSet;

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy, Debug)]
pub struct Move {
    direction: Direction,
    num_moves: i32,
}
impl Move {
    const fn right(self) -> i32 {
        match self.direction {
            Direction::Right => 1,
            Direction::Left => -1,
            _ => 0,
        }
    }
    const fn up(self) -> i32 {
        match self.direction {
            Direction::Up => 1,
            Direction::Down => -1,
            _ => 0,
        }
    }
}

pub struct Day9;
impl Day for Day9 {
    type Parsed = Vec<Move>;
    type Output = usize;

    fn parse(input: String) -> Result<Self::Parsed> {
        Ok(input
            .lines()
            .map(|line| Move {
                direction: match &line[0..1] {
                    "U" => Direction::Up,
                    "R" => Direction::Right,
                    "D" => Direction::Down,
                    "L" => Direction::Left,
                    _ => panic!(),
                },
                num_moves: line[2..].parse::<i32>().unwrap(),
            })
            .collect())
    }
    fn first(moves: Self::Parsed) -> Self::Output {
        snake_move::<2>(&moves)
    }
    fn second(moves: Self::Parsed) -> Self::Output {
        snake_move::<10>(&moves)
    }
}

fn snake_move<const SNAKE_LENGTH: usize>(moves: &Vec<Move>) -> usize {
    assert!(SNAKE_LENGTH >= 2 && SNAKE_LENGTH <= 10);
    let mut rope = [[0, 0]; SNAKE_LENGTH];
    let mut locations = FxHashSet::new();
    locations.insert(rope[0]);
    for m in moves {
        for _ in 0..m.num_moves {
            rope[0][0] += m.right();
            rope[0][1] += m.up();
            for follow in 0..(SNAKE_LENGTH - 1) {
                let (head, tail) = (rope[follow], &mut rope[follow + 1]);
                if tail[0].abs_diff(head[0]) > 1 || tail[1].abs_diff(head[1]) > 1 {
                    tail[0] += (head[0] - tail[0]).signum();
                    tail[1] += (head[1] - tail[1]).signum();
                }
            }
            locations.insert(rope[SNAKE_LENGTH - 1]);
        }
    }
    locations.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    const INPUT2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
    fn parsed(input: &str) -> <Day9 as Day>::Parsed {
        Day9::parse(input.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day9::first(parsed(INPUT1)), 13);
        assert_eq!(Day9::first(parsed(INPUT2)), 88);
    }
    #[test]
    fn part2() {
        assert_eq!(Day9::second(parsed(INPUT1)), 1);
        assert_eq!(Day9::second(parsed(INPUT2)), 36);
    }
}
