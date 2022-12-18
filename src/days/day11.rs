use std::{collections::{VecDeque, hash_map::DefaultHasher}, hash::{Hasher, Hash}};

use itertools::Itertools;
use rustc_hash::FxHashMap;
use super::day::Day;
use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operation {
    Multiplication(Option<u8>),
    Addition(Option<u8>),
}
impl Operation {
    const fn operate(self, left: i64) -> i64 {
        match self {
            Self::Multiplication(right) => match right {
                Some(i) => left * i as i64,
                None => left * left,
            },
            Self::Addition(right) => match right {
                Some(i) => left + i as i64,
                None => left + left,
            },
        }
    }
}
#[derive(Debug, Clone)]
pub struct Monkey {
    items: VecDeque<i64>,
    operation: Operation,
    div_test: i64,
    if_true: usize,
    if_false: usize,
    throws: i64,
}

pub struct Day11;
impl Day for Day11 {
    type Parsed = Vec<Monkey>;
    type Output = i64;

    fn parse(input: String) -> Result<Self::Parsed> {
        let mut it = input.lines();
        let mut monkeys = vec![];
        while let Some(_monkey) = it.next() {
            let items: VecDeque<i64> = it.next().unwrap()[18..]
                .split(", ")
                .map(|s| s.parse().unwrap())
                .collect();
            let op = &it.next().unwrap()[23..];
            let val = op[2..].parse().ok();
            let operation = match &op[0..1] {
                "*" => Operation::Multiplication(val),
                "+" => Operation::Addition(val),
                _ => panic!(),
            };
            let test = it.next().unwrap()[21..].parse()?;
            let if_true = it.next().unwrap()[29..].parse()?;
            let if_false = it.next().unwrap()[30..].parse()?;
            it.next();
            monkeys.push(Monkey {
                items,
                operation,
                div_test: test,
                if_true,
                if_false,
                throws: 0,
            });
        }
        Ok(monkeys)
    }
    fn first(mut monkeys: Self::Parsed) -> Self::Output {
        for _ in 1..=20 {
            for i in 0..monkeys.len() {
                while let Some(mut item_worry) = monkeys[i].items.pop_front() {
                    item_worry = monkeys[i].operation.operate(item_worry) / 3;
                    let throw_to = if item_worry % monkeys[i].div_test == 0 {
                        monkeys[i].if_true
                    } else {
                        monkeys[i].if_false
                    };
                    monkeys[throw_to].items.push_back(item_worry);
                    monkeys[i].throws += 1;
                }
            }
        }
        monkeys
            .iter()
            .map(|m| m.throws)
            .sorted()
            .rev()
            .take(2)
            .product()
    }
    fn second(mut monkeys: Self::Parsed) -> Self::Output {
        // LCM of list of unique primes = product of list
        let lcm: i64 = monkeys
            .iter()
            .map(|m| m.div_test)
            .product();
        let mut monkey_states: FxHashMap<_, (_, Vec<_>)> = FxHashMap::default();
        let mut found_cycle = false;
        let mut x = 1;  // =10000
        
        while x <= 10000 {
            for i in 0..monkeys.len() {
                unsafe {
                    while let Some(mut item_worry) = monkeys.get_unchecked_mut(i).items.pop_front() {
                        item_worry = monkeys.get_unchecked(i).operation.operate(item_worry) % lcm;
                        let throw_to = if item_worry % monkeys.get_unchecked(i).div_test == 0 {
                            monkeys.get_unchecked(i).if_true
                        } else {
                            monkeys.get_unchecked(i).if_false
                        };
                        monkeys.get_unchecked_mut(throw_to).items.push_back(item_worry);
                        monkeys.get_unchecked_mut(i).throws += 1;
                    }
                }
            }
            if !found_cycle {
                let mut hasher = DefaultHasher::new();
                for monkey in &monkeys {
                    monkey.items.hash(&mut hasher);
                }
                let key = hasher.finish();
                if let Some((old_x, old_throws)) = monkey_states.get(&key) {
                    let same_diff = x - old_x;
                    let cycles = (10000 - x) / same_diff;
                    x += cycles * same_diff;
                    found_cycle = true;
                    for i in 0..monkeys.len() {
                        monkeys[i].throws += (monkeys[i].throws - old_throws[i]) * i64::from(cycles);
                    }
                    
                } else {
                    monkey_states.insert(key, (x, monkeys.iter().map(|m| m.throws).collect::<Vec<_>>()));
                }
            }
            x += 1;
        }
        monkeys
            .iter()
            .map(|m| m.throws)
            .sorted()
            .rev()
            .take(2)
            .product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
    fn parsed() -> <Day11 as Day>::Parsed {
        Day11::parse(INPUT.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day11::first(parsed()), 10605);
    }
    #[test]
    fn part2() {
        assert_eq!(Day11::second(parsed()), 2713310158);
    }
}
