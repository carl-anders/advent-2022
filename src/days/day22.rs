use super::day::Day;
use anyhow::Result;

pub struct Day22;
impl Day for Day22 {
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
    fn parsed() -> <Day22 as Day>::Parsed {
        Day22::parse(INPUT.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day22::first(parsed()), 0);
    }
    #[test]
    fn part2() {
        assert_eq!(Day22::second(parsed()), 0);
    }
}
