use super::day::Day;
use anyhow::Result;

pub struct Day4;
impl Day for Day4 {
    type Parsed = Vec<[[i32; 2]; 2]>;
    type Output = i32;

    fn parse(input: String) -> Result<Self::Parsed> {
        Ok(input
            .lines()
            .map(|line| {
                line.split(',')
                    .map(|side| {
                        side.split('-')
                            .map(|val| val.parse::<i32>().unwrap())
                            .collect::<Vec<_>>()
                            .try_into()
                            .unwrap()
                    })
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect())
    }
    fn first(pairs: Self::Parsed) -> Self::Output {
        pairs
            .into_iter()
            .map(|[left, right]| {
                i32::from(
                    (left[0] <= right[1] && left[1] >= right[1])
                        || (right[0] <= left[0] && right[1] >= left[1]),
                )
            })
            .sum()
    }
    fn second(pairs: Self::Parsed) -> Self::Output {
        pairs
            .into_iter()
            .map(|[left, right]| i32::from(right[0] <= left[1] && right[1] >= left[0]))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
    fn parsed() -> <Day4 as Day>::Parsed {
        Day4::parse(INPUT.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day4::first(parsed()), 2);
    }
    #[test]
    fn part2() {
        assert_eq!(Day4::second(parsed()), 4);
    }
}
