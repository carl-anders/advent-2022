use itertools::Itertools;

use super::day::Day;
use anyhow::Result;

pub struct Day1;
impl Day for Day1 {
    type Parsed = Vec<Vec<i32>>;
    type Output = i32;

    fn parse(input: String) -> Result<Self::Parsed> {
        Ok(input.lines().fold(vec![vec![]], |mut elves, line| {
            match line {
                "" => elves.push(vec![]),
                _ => elves.last_mut().unwrap().push(line.parse::<i32>().unwrap()),
            }
            elves
        }))
    }
    fn first(elves: Self::Parsed) -> Self::Output {
        elves
            .into_iter()
            .map(|e| e.iter().sum::<i32>())
            .sorted()
            .last()
            .unwrap()
    }
    fn second(elves: Self::Parsed) -> Self::Output {
        elves
            .into_iter()
            .map(|e| e.iter().sum::<i32>())
            .sorted()
            .rev()
            .take(3)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
    fn parsed() -> <Day1 as Day>::Parsed {
        Day1::parse(INPUT.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day1::first(parsed()), 24000);
    }
    #[test]
    fn part2() {
        assert_eq!(Day1::second(parsed()), 45000);
    }
}
