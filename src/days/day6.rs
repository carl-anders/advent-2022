use super::day::Day;
use crate::helpers::BitArray;
use anyhow::Result;

fn find_first_unique<const UNIQUE: usize>(chars: &[u8]) -> usize {
    let a = chars
        .windows(UNIQUE)
        .enumerate()
        .find_map(|(i, slice)| {
            let mut f = 0;
            for item in slice.iter().take(UNIQUE) {
                f.set((item - b'a') as usize);
            }
            if f.count_ones() as usize == UNIQUE {
                Some(i)
            } else {
                None
            }
        })
        .unwrap();
    a + UNIQUE
}

pub struct Day6;
impl Day for Day6 {
    type Parsed = Vec<u8>;
    type Output = usize;

    fn parse(input: String) -> Result<Self::Parsed> {
        Ok(input.trim_end().as_bytes().to_vec())
    }
    fn first(chars: Self::Parsed) -> Self::Output {
        find_first_unique::<4>(&chars)
    }
    fn second(chars: Self::Parsed) -> Self::Output {
        find_first_unique::<14>(&chars)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTS: [(&str, usize, usize); 5] = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6, 23),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26),
    ];

    fn parse(input: &str) -> <Day6 as Day>::Parsed {
        Day6::parse(input.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        for i in TESTS {
            assert_eq!(Day6::first(parse(i.0)), i.1);
        }
    }
    #[test]
    fn part2() {
        for i in TESTS {
            assert_eq!(Day6::second(parse(i.0)), i.2);
        }
    }
}
