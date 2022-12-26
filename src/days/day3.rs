use super::day::Day;
use anyhow::Result;
use itertools::Itertools;

const fn char_as_score(c: char) -> i32 {
    if c >= 'a' {
        c as i32 - 'a' as i32 + 1
    } else {
        c as i32 - 'A' as i32 + 27
    }
}

pub struct Day3;
impl Day for Day3 {
    type Parsed = String;
    type Output = i32;

    fn parse(input: String) -> Result<Self::Parsed> {
        Ok(input)
    }
    fn first(lines: Self::Parsed) -> Self::Output {
        lines
            .lines()
            .map(|line| {
                let (left, right) = line.split_at(line.len() / 2);
                left.chars()
                    .find_map(|c| right.find(c).map(|_| char_as_score(c)))
                    .unwrap_or(0)
            })
            .sum()
    }
    fn second(lines: Self::Parsed) -> Self::Output {
        lines
            .lines()
            .tuples()
            .filter_map(|(a, b, c)| {
                a.chars().find_map(|char| {
                    b.find(char)
                        .and_then(|_| c.find(char))
                        .map(|_| char_as_score(char))
                })
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
    fn parsed() -> <Day3 as Day>::Parsed {
        Day3::parse(INPUT.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day3::first(parsed()), 157);
    }
    #[test]
    fn part2() {
        assert_eq!(Day3::second(parsed()), 70);
    }
}
