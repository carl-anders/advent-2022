use super::day::Day;
use anyhow::Result;

pub struct Day19;
impl Day for Day19 {
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
    fn parsed() -> <Day19 as Day>::Parsed {
        Day19::parse(INPUT.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day19::first(parsed()), 0);
    }
    #[test]
    fn part2() {
        assert_eq!(Day19::second(parsed()), 0);
    }
}
