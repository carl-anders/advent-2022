use super::day::Day;
use ahash::HashMap;
use anyhow::Result;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Operator {
    Mult,
    Add,
    Sub,
    Div,
}
impl Operator {
    const fn _to_char(self) -> char {
        match self {
            Self::Mult => '*',
            Self::Add => '+',
            Self::Sub => '-',
            Self::Div => '/',
        }
    }
    const fn calculate(self, a: i64, b: i64) -> i64 {
        match self {
            Self::Mult => a * b,
            Self::Add => a + b,
            Self::Sub => a - b,
            Self::Div => a / b,
        }
    }
    const fn backwards_eq(self, eq: i64, side: i64, swapped_side: bool) -> i64 {
        if swapped_side {
            match self {
                Self::Mult => eq / side, // side * x == eq -> eq / side
                Self::Add => eq - side,  // side + x == eq -> eq - side
                Self::Sub => side - eq,  // side - x == eq -> side - eq
                Self::Div => side / eq,  // side / x == eq -> side / eq
            }
        } else {
            match self {
                Self::Mult => eq / side, // x * side == eq -> eq / side
                Self::Add => eq - side,  // x + side == eq -> eq - side
                Self::Sub => eq + side,  // x - side == eq -> eq + side
                Self::Div => eq * side,  // x / side == eq -> eq * side
            }
        }
    }
}
impl TryFrom<char> for Operator {
    type Error = ();
    fn try_from(item: char) -> Result<Self, Self::Error> {
        match item {
            '+' => Ok(Self::Add),
            '-' => Ok(Self::Sub),
            '*' => Ok(Self::Mult),
            '/' => Ok(Self::Div),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Monkey {
    Number(i64),
    Operation(String, Operator, String),
}
impl Monkey {
    fn calculate(&self, monkeys: &HashMap<String, Self>) -> i64 {
        match self {
            Self::Number(num) => *num,
            Self::Operation(a, op, b) => {
                op.calculate(monkeys[a].calculate(monkeys), monkeys[b].calculate(monkeys))
            }
        }
    }
    fn has_human(&self, monkeys: &HashMap<String, Self>) -> bool {
        match self {
            Self::Operation(a, _, b) => {
                a == "humn"
                    || b == "humn"
                    || monkeys[a].has_human(monkeys)
                    || monkeys[b].has_human(monkeys)
            }
            Self::Number(_) => false,
        }
    }
    fn backwards_eq(&self, monkeys: &HashMap<String, Self>, eq: i64) -> i64 {
        match self {
            Self::Number(n) => *n,
            Self::Operation(a, op, b) => {
                if a == "humn" {
                    let side = monkeys[b].calculate(monkeys);
                    op.backwards_eq(eq, side, false)
                } else if b == "humn" {
                    let side = monkeys[a].calculate(monkeys);
                    op.backwards_eq(eq, side, true)
                } else if monkeys[a].has_human(monkeys) {
                    let side = monkeys[b].calculate(monkeys);
                    monkeys[a].backwards_eq(monkeys, op.backwards_eq(eq, side, false))
                } else if monkeys[b].has_human(monkeys) {
                    let side = monkeys[a].calculate(monkeys);
                    monkeys[b].backwards_eq(monkeys, op.backwards_eq(eq, side, true))
                } else {
                    panic!("Neither monkey depends on numan. Invalid input: {a} {b} -> {eq}")
                }
            }
        }
    }
}

pub struct Day21;
impl Day for Day21 {
    type Parsed = HashMap<String, Monkey>;
    type Output = i64;

    fn parse(input: String) -> Result<Self::Parsed> {
        Ok(input
            .lines()
            .map(|line| {
                let parts: Vec<_> = line.split(' ').collect();
                (
                    parts[0][0..4].to_string(),
                    match parts.len() {
                        2 => Monkey::Number(parts[1].parse().unwrap()),
                        _ => Monkey::Operation(
                            parts[1].to_string(),
                            parts[2].chars().next().unwrap().try_into().unwrap(),
                            parts[3].to_string(),
                        ),
                    },
                )
            })
            .collect())
    }
    fn first(monkeys: Self::Parsed) -> Self::Output {
        monkeys["root"].calculate(&monkeys)
    }
    fn second(monkeys: Self::Parsed) -> Self::Output {
        match monkeys.get("root").unwrap() {
            Monkey::Operation(a, _, b) => {
                if monkeys[a].has_human(&monkeys) {
                    monkeys[a].backwards_eq(&monkeys, monkeys[b].calculate(&monkeys))
                } else {
                    monkeys[b].backwards_eq(&monkeys, monkeys[a].calculate(&monkeys))
                }
            }
            Monkey::Number(n) => *n,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";
    fn parsed() -> <Day21 as Day>::Parsed {
        Day21::parse(INPUT.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day21::first(parsed()), 152);
    }
    #[test]
    fn part2() {
        assert_eq!(Day21::second(parsed()), 301);
    }
}
