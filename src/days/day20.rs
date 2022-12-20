#![allow(clippy::cast_possible_truncation)]
use std::collections::VecDeque;

use super::day::Day;
use anyhow::Result;

pub struct Day20;
impl Day for Day20 {
    type Parsed = VecDeque<(i64, usize)>;
    type Output = i64;

    fn parse(input: String) -> Result<Self::Parsed> {
        Ok(input
            .lines()
            .enumerate()
            .map(|(i, line)| (line.parse().unwrap(), i))
            .collect())
    }
    fn first(mut data: Self::Parsed) -> Self::Output {
        shuffle(&mut data);
        key(&data)
    }
    fn second(mut data: Self::Parsed) -> Self::Output {
        for d in &mut data {
            d.0 *= 811_589_153;
        }
        for _ in 0..10 {
            shuffle(&mut data);
        }
        key(&data)
    }
}

fn key(data: &VecDeque<(i64, usize)>) -> i64 {
    let zero_pos = data.iter().position(|(d, _)| *d == 0).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|d| data[(zero_pos + d).rem_euclid(data.len())].0)
        .sum()
}

fn shuffle(data: &mut VecDeque<(i64, usize)>) {
    for i in 0..data.len() {
        let found = data.iter().position(|(_, d)| *d == i).unwrap();
        let removed = data.remove(found).unwrap();
        let new_index = ((found as i64) + removed.0).rem_euclid(data.len() as i64);
        data.insert(new_index as usize, removed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "1
2
-3
3
-2
0
4";
    fn parsed() -> <Day20 as Day>::Parsed {
        Day20::parse(INPUT.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day20::first(parsed()), 3);
    }
    #[test]
    fn part2() {
        assert_eq!(Day20::second(parsed()), 1623178306);
    }
}
