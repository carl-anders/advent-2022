#![allow(clippy::cast_possible_wrap)]
use super::day::Day;
use anyhow::Result;

const SNAFUS: &str = "=-012";

fn snafu_to_i(snafu: &str) -> isize {
    if snafu.is_empty() {
        0
    } else {
        let (rest, last) = snafu.split_at(snafu.len() - 1);
        snafu_to_i(rest) * 5 + SNAFUS.find(last).unwrap() as isize - 2
    }
}
fn i_to_snafu(i: isize) -> String {
    if i == 0 {
        String::new()
    } else {
        format!(
            "{}{}",
            i_to_snafu((i + 2) / 5),
            SNAFUS.chars().nth(((i + 2) % 5) as usize).unwrap()
        )
    }
}
pub struct Day25;
impl Day for Day25 {
    type Parsed = String;
    type Output = String;

    fn parse(input: String) -> Result<Self::Parsed> {
        Ok(input)
    }
    fn first(input: Self::Parsed) -> Self::Output {
        let mut sum = 0;
        for line in input.lines() {
            sum += snafu_to_i(line);
        }
        i_to_snafu(sum)
    }
    fn second(_lines: Self::Parsed) -> Self::Output {
        "0".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";
    fn parsed() -> <Day25 as Day>::Parsed {
        Day25::parse(INPUT.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day25::first(parsed()), "2=-1=0");
    }
}
