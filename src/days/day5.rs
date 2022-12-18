use itertools::Itertools;

use super::{day::Day, helpers::BorrowTwo};
use anyhow::Result;

#[derive(Clone, Copy, Debug)]
pub struct Move {
    num: usize,
    from: usize,
    to: usize,
}

#[derive(Clone, Debug)]
pub struct Stacks(Vec<Vec<char>>);
impl Stacks {
    fn move_one(&mut self, from: usize, to: usize) {
        if let Some(v) = self.0[from].pop() {
            self.0[to].push(v);
        }
    }
    fn move_many_one_by_one(&mut self, m: Move) {
        for _ in 0..m.num {
            self.move_one(m.from, m.to);
        }
    }
    fn move_many(&mut self, m: Move) {
        if m.num == 1 {
            self.move_one(m.from, m.to);
        } else {
            let split_at = self.0[m.from].len() - m.num;

            let (from, to) = self.0.borrow_two(m.from, m.to);
            to.extend(from.drain(split_at..));
        }
    }
    fn get_top(&self) -> String {
        self.0.iter().filter_map(|t| t.last()).collect()
    }
}

pub struct Day5;
impl Day for Day5 {
    type Parsed = (Stacks, Vec<Move>);
    type Output = String;

    fn parse(input: String) -> Result<Self::Parsed> {
        let (input_stacks, input_moves) = input.split_once("\n\n").unwrap();

        let num_stacks = (input_stacks.lines().next().unwrap().len() + 1) / 4;
        let mut stacks = vec![Vec::new(); num_stacks];

        for line in input_stacks.lines() {
            if line.chars().nth(1).unwrap() == '1' {
                break;
            }
            for (i, stack) in stacks.iter_mut().enumerate() {
                let letter = line.chars().nth(i * 4 + 1).unwrap();
                if letter >= 'A' {
                    stack.insert(0, letter);
                }
            }
        }

        let moves = input_moves.lines()
            .map(|line| {
                let m: (usize, usize, usize) = line
                    .split(' ')
                    .filter_map(|s| s.parse().ok())
                    .next_tuple()
                    .unwrap();
                Move {
                    num: m.0,
                    from: m.1 - 1,
                    to: m.2 - 1,
                }
            })
            .collect();

        Ok((Stacks(stacks), moves))
    }
    fn first((mut stacks, moves): Self::Parsed) -> Self::Output {
        for mov in moves {
            stacks.move_many_one_by_one(mov);
        }
        stacks.get_top()
    }
    fn second((mut stacks, moves): Self::Parsed) -> Self::Output {
        for mov in moves {
            stacks.move_many(mov);
        }
        stacks.get_top()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    fn parsed() -> <Day5 as Day>::Parsed {
        Day5::parse(INPUT.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day5::first(parsed()), "CMZ");
    }
    #[test]
    fn part2() {
        assert_eq!(Day5::second(parsed()), "MCD");
    }
}
