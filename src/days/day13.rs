use std::cmp::Ordering;

use itertools::Itertools;

use super::day::Day;
use anyhow::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Packet {
    Parent(Vec<Self>),
    Number(u8),
}
impl Packet {
    fn push(&mut self, packet: Self) {
        if let Self::Parent(list) = self {
            list.push(packet);
        }
    }
    fn push_num(&mut self, num: u8) {
        if let Self::Parent(list) = self {
            list.push(Self::Number(num));
        }
    }
    const fn new() -> Self {
        Self::Parent(vec![])
    }
    fn _format(&self) -> String {
        match self {
            Self::Parent(childs) => format!("[{}]", childs.iter().map(Self::_format).join(",")),
            Self::Number(num) => num.to_string(),
        }
    }
    fn _get_first_num(&self) -> Option<u8> {
        match self {
            Self::Parent(childs) => childs.first().and_then(Self::_get_first_num),
            Self::Number(num) => Some(*num),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, right: &Self) -> Ordering {
        use Packet::*;
        match (self, right) {
            (Number(l_num), Number(r_num)) => l_num.cmp(r_num),
            (Parent(l_childs), Parent(r_childs)) => l_childs.cmp(r_childs),
            (Parent(l_childs), Number(r_num)) => l_childs[..].cmp(&[Number(*r_num)]),
            (Number(l_num), Parent(r_childs)) => [Number(*l_num)][..].cmp(&r_childs[..]),
        }
    }
}
impl PartialOrd for Packet {
    fn partial_cmp(&self, right: &Self) -> Option<Ordering> {
        Some(self.cmp(right))
    }
}

fn recurse_parse(input: &mut &str) -> Packet {
    let mut packet = Packet::new();
    loop {
        let mut it = input.chars();
        match it.next() {
            Some('[') => {
                *input = &input[1..];
                packet.push(recurse_parse(input));
            }
            Some(']') => {
                *input = &input[1..];
                return packet;
            }
            Some(first_num) if first_num >= '0' && first_num <= '9' => match it.next() {
                Some(second_num) if second_num >= '0' && second_num <= '9' => {
                    packet.push_num(input[0..2].parse().unwrap());
                    *input = &input[2..];
                }
                _ => {
                    *input = &input[1..];
                    packet.push_num(first_num.to_digit(10).unwrap().try_into().unwrap());
                }
            },
            Some(_) => {
                *input = &input[1..];
            }
            _ => return packet,
        }
    }
}

pub struct Day13;
impl Day for Day13 {
    type Parsed = Vec<Packet>;
    type Output = usize;

    fn parse(input: String) -> Result<Self::Parsed> {
        Ok(input
            .lines()
            .filter_map(|line| {
                if line.is_empty() {
                    None
                } else {
                    Some(recurse_parse(&mut &line[1..line.len() - 1]))
                }
            })
            .collect())
    }
    fn first(packets: Self::Parsed) -> Self::Output {
        packets
            .into_iter()
            .tuples()
            .enumerate()
            .map(|(i, (left, right))| {
                if left.cmp(&right) == Ordering::Greater {
                    0
                } else {
                    i + 1
                }
            })
            .sum()
    }
    fn second(mut packets: Self::Parsed) -> Self::Output {
        let two = Packet::Parent(vec![Packet::Parent(vec![Packet::Number(2)])]);
        let six = Packet::Parent(vec![Packet::Parent(vec![Packet::Number(6)])]);
        packets.push(two.clone());
        packets.push(six.clone());
        packets.sort_unstable();
        packets
            .into_iter()
            .enumerate()
            .filter_map(|(i, p)| {
                (p.cmp(&two) == Ordering::Equal || p.cmp(&six) == Ordering::Equal).then_some(i + 1)
            })
            .product::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
    fn parsed() -> <Day13 as Day>::Parsed {
        Day13::parse(INPUT.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day13::first(parsed()), 13);
    }
    #[test]
    fn part2() {
        assert_eq!(Day13::second(parsed()), 140);
    }
}
