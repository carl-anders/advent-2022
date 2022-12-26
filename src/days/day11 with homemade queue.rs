use super::day::Day;
use anyhow::Result;
use itertools::Itertools;
use rustc_hash::FxHashMap;
use smallvec::SmallVec;
use std::collections::VecDeque;

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
    items: TotallySafeArray,
    operation: Operation,
    div_test: i64,
    if_true: usize,
    if_false: usize,
    throws: i64,
}
#[derive(Debug, Clone, Copy)]
pub struct TotallySafeArray {
    items: [i64; 32],
    front: usize,
    back: usize,
    len: usize,
}
impl TotallySafeArray {
    const fn new() -> Self {
        Self {
            items: [0; 32],
            front: 0,
            back: 0,
            len: 0,
        }
    }
    fn push_back(&mut self, val: i64) {
        unsafe {
            *self.items.get_unchecked_mut(self.back) = val;
            self.len += 1;
            self.back = (self.back + 1) % 32;
        }
    }
    fn pop_front(&mut self) -> Option<i64> {
        if self.len > 0 {
            unsafe {
                let val = *self.items.get_unchecked(self.front);
                self.len -= 1;
                self.front = (self.front + 1) % 32;
                Some(val)
            }
        } else {
            None
        }
    }

    /* fn get_unchecked(&self, index: usize) -> &i64 {
        let index = index % 32;
        unsafe {
            self.items.get_unchecked(index)
        }
    }
    fn copy_flatten(&self) -> SmallVec<[i64;16]> {
        if self.front < self.back {
            SmallVec::from_slice(&self.items[self.front..self.back])
        } else {
            let mut v = SmallVec::from_slice(&self.items[self.front..32]);
            v.extend_from_slice(&self.items[0..self.back]);
            v
        }
    } */
}
#[allow(clippy::copy_iterator)]
impl Iterator for TotallySafeArray {
    type Item = i64;
    fn next(&mut self) -> Option<Self::Item> {
        self.pop_front()
    }
}

/*impl PartialEq for TotallySafeArray {
    fn eq(&self, other: &Self) -> bool {
        if self.len != other.len {
            false
        } else {
            let self_overflows = self.back < self.front;
            let other_overflows = other.back < other.front;
            match (self_overflows, other_overflows) {
                (true, true) => {
                    println!("aa");
                    for i in 0..self.len {
                        let sp = (self.front + i) % 32;
                        let op = (other.front + i) % 32;
                        if self.items[sp] != other.items[op] {
                            return false;
                        }
                    }
                    true
                },
                (true, false) => {
                    println!("bb");
                    if self.items[self.front..32] == other.items[other.front..other.front+32-self.front] {
                        self.items[0..self.back] == other.items[other.front+32-self.front..other.back]
                    } else {
                        false
                    }
                },
                (false, true) => {
                    println!("cc");
                    if other.items[other.front..32] == self.items[self.front..self.front+32-other.front] {
                        other.items[0..other.back] == self.items[self.front+32-other.front..self.back]
                    } else {
                        false
                    }
                },
                (false, false) => {
                    println!("dd");
                    self.items[self.front..self.back] == other.items[other.front..other.back]
                },
            }
        }
    }
}
impl Eq for TotallySafeArray {}
impl std::hash::Hash for TotallySafeArray {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        state.write(&[self.len as u8]);
        if self.front + self.len >= 32 {
            let right = &self.items[self.front..32];
            state.write(unsafe { right.align_to::<u8>().1 });
            let left = &self.items[0..self.back];
            state.write(unsafe { left.align_to::<u8>().1 });
        } else {
            let right = &self.items[self.front..self.back];
            state.write(unsafe { right.align_to::<u8>().1 });
        }
        state.finish();
    }
}*/

pub struct Day11;
impl Day for Day11 {
    type Parsed = Vec<Monkey>;
    type Output = i64;

    fn parse(input: String) -> Result<Self::Parsed> {
        let mut it = input.lines();
        let mut monkeys = vec![];
        while let Some(_monkey) = it.next() {
            let vec: VecDeque<i64> = it.next().unwrap()[18..]
                .split(", ")
                .map(|s| s.parse().unwrap())
                .collect();
            let mut items = TotallySafeArray::new();
            for i in vec {
                items.push_back(i);
            }
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
        let lcm: i64 = monkeys.iter().map(|m| m.div_test).product();
        let mut monkey_states: FxHashMap<_, (_, Vec<_>)> = FxHashMap::default();
        let mut found_cycle = false;
        let mut x = 1; // =10000
        while x <= 10000 {
            for i in 0..monkeys.len() {
                unsafe {
                    while let Some(mut item_worry) = monkeys.get_unchecked_mut(i).items.pop_front()
                    {
                        item_worry = monkeys.get_unchecked(i).operation.operate(item_worry) % lcm;
                        let throw_to = if item_worry % monkeys.get_unchecked(i).div_test == 0 {
                            monkeys.get_unchecked(i).if_true
                        } else {
                            monkeys.get_unchecked(i).if_false
                        };
                        monkeys
                            .get_unchecked_mut(throw_to)
                            .items
                            .push_back(item_worry);
                        monkeys.get_unchecked_mut(i).throws += 1;
                    }
                }
            }
            if !found_cycle {
                let list: Vec<_> = monkeys
                    .iter()
                    .map(|m| m.items.into_iter().collect::<SmallVec<[_; 16]>>())
                    .collect();
                if let Some((old_x, old_throws)) = monkey_states.get(&list) {
                    let same_diff = x - old_x;
                    let cycles = (10000 - x) / same_diff;
                    x += cycles * same_diff;
                    found_cycle = true;
                    for i in 0..monkeys.len() {
                        monkeys[i].throws +=
                            (monkeys[i].throws - old_throws[i]) * i64::from(cycles);
                    }
                } else {
                    monkey_states.insert(
                        list,
                        (x, monkeys.iter().map(|m| m.throws).collect::<Vec<_>>()),
                    );
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
