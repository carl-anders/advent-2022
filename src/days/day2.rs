use std::convert::TryFrom;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Shape {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}
impl TryFrom<char> for Shape {
    type Error = ();
    fn try_from(v: char) -> Result<Self, Self::Error> {
        match v {
            'A' | 'X' => Ok(Self::Rock),
            'B' | 'Y' => Ok(Self::Paper),
            'C' | 'Z' => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}
impl Shape {
    pub const LOSE: Self = Self::Rock;
    pub const DRAW: Self = Self::Paper;
    pub const WIN: Self = Self::Scissors;
}

use itertools::Itertools;

use super::day::Day;
use anyhow::Result;

pub struct Day2;
impl Day for Day2 {
    type Parsed = Vec<(Shape, Shape)>;
    type Output = i32;

    fn parse(input: String) -> Result<Self::Parsed> {
        Ok(input
            .lines()
            .map(|line| {
                line.chars()
                    .filter_map(|s| s.try_into().ok())
                    .next_tuple()
                    .unwrap()
            })
            .collect())
    }
    fn first(lines: Self::Parsed) -> Self::Output {
        lines
            .iter()
            .map(|shapes| {
                match shapes.1 {
                    Shape::Rock => match shapes.0 {
                        Shape::Rock => 4,     // 1 + 3
                        Shape::Paper => 1,    // 1 + 0
                        Shape::Scissors => 7, // 1 + 6
                    },
                    Shape::Paper => match shapes.0 {
                        Shape::Rock => 8,     // 2 + 6
                        Shape::Paper => 5,    // 2 + 3
                        Shape::Scissors => 2, // 2 + 0
                    },
                    Shape::Scissors => match shapes.0 {
                        Shape::Rock => 3,     // 3 + 0
                        Shape::Paper => 9,    // 3 + 6
                        Shape::Scissors => 6, // 3 + 3
                    },
                }
            })
            .sum()
    }
    fn second(lines: Self::Parsed) -> Self::Output {
        lines
            .iter()
            .map(|shapes| {
                match shapes.1 {
                    Shape::LOSE => match shapes.0 {
                        Shape::Rock => 3,     // 3 + 0
                        Shape::Paper => 1,    // 1 + 0
                        Shape::Scissors => 2, // 2 + 0
                    },
                    Shape::DRAW => match shapes.0 {
                        Shape::Rock => 4,     // 1 + 3
                        Shape::Paper => 5,    // 2 + 3
                        Shape::Scissors => 6, // 3 + 3
                    },
                    Shape::WIN => match shapes.0 {
                        Shape::Rock => 8,     // 2 + 6
                        Shape::Paper => 9,    // 3 + 6
                        Shape::Scissors => 7, // 1 + 6
                    },
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "A Y
B X
C Z";
    fn parsed() -> <Day2 as Day>::Parsed {
        Day2::parse(INPUT.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day2::first(parsed()), 15);
    }
    #[test]
    fn part2() {
        assert_eq!(Day2::second(parsed()), 12);
    }
}
