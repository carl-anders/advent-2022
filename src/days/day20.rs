use super::day::Day;
use anyhow::Result;

pub struct Day20;
impl Day for Day20 {
    type Parsed = String;
    type Output = i32;

    fn parse(input: String) -> Result<Self::Parsed> {
        Ok(input)
    }
    fn first(_lines: Self::Parsed) -> Self::Output {
        0
    }
    fn second(_lines: Self::Parsed) -> Self::Output {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "";
    fn parsed() -> <Day20 as Day>::Parsed {
        Day20::parse(INPUT.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day20::first(parsed()), 0);
    }
    #[test]
    fn part2() {
        assert_eq!(Day20::second(parsed()), 0);
    }
}
