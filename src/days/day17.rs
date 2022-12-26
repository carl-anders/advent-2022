use super::day::Day;
use ahash::AHashMap;
use anyhow::Result;
use std::collections::hash_map::{DefaultHasher, Entry};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub enum Shape {
    SideLine,
    Plus,
    MirroredL,
    Line,
    Square,
}
impl Shape {
    const fn int(self) -> u32 {
        match self {
            Self::SideLine => 0b0001_1110,                           // widths: 4
            Self::Plus => 0b0000_1000_0001_1100_0000_1000,           // widths: 1,3,1
            Self::MirroredL => 0b0000_0100_0000_0100_0001_1100,      // widths: 1,1,3
            Self::Line => 0b0001_0000_0001_0000_0001_0000_0001_0000, // widths: 1,1,1,1
            Self::Square => 0b0001_1000_0001_1000,                   // widths: 2,2
        }
    }
}
const ALL_SHAPES: [Shape; 5] = [
    Shape::SideLine,
    Shape::Plus,
    Shape::MirroredL,
    Shape::Line,
    Shape::Square,
];
#[derive(Debug)]
pub struct Shaper(u32);
impl Shaper {
    const fn new(shape: Shape) -> Self {
        Self(shape.int())
    }
    fn wind(&mut self, dir: Direction, map: u32) {
        match dir {
            Direction::Right => {
                if self.0 & 0b0000_0001_0000_0001_0000_0001_0000_0001 == 0 && self.0 >> 1 & map == 0
                {
                    self.0 >>= 1;
                }
            }
            Direction::Left => {
                if self.0 & 0b0100_0000_0100_0000_0100_0000_0100_0000 == 0 && self.0 << 1 & map == 0
                {
                    self.0 <<= 1;
                }
            }
        }
        //println!("{:#034b}", self.array);
    }
    const fn crashes(&self, map: u32) -> bool {
        self.0 & map > 0
    }
}

fn map_mask(map: &[u8], height: usize) -> u32 {
    if height >= map.len() {
        0
    } else {
        let mut mask = 0;
        for byte in map[height..].iter().take(4).rev() {
            mask = (mask << 8) + u32::from(*byte);
        }
        mask
    }
}

#[allow(dead_code)]
fn map_display(m: &[u8]) {
    for line in m.iter().rev() {
        println!(
            "{}",
            &format!("{line:#010b}").replace('0', ".").replace('1', "#")[3..]
        );
    }
    println!("________________");
}

pub struct Day17;
impl Day for Day17 {
    type Parsed = Vec<Direction>;
    type Output = usize;

    fn parse(input: String) -> Result<Self::Parsed> {
        Ok(input
            .trim_end()
            .as_bytes()
            .iter()
            .map(|c| match c {
                b'>' => Direction::Right,
                _ => Direction::Left,
            })
            .collect())
    }
    fn first(moves: Self::Parsed) -> Self::Output {
        let mut move_it = moves.into_iter().enumerate().cycle();
        let mut shape_it = ALL_SHAPES.iter().cycle();
        let mut map: Vec<u8> = vec![];

        for _ in 0..2022 {
            simulate_block(&mut map, &mut move_it, &mut shape_it);
        }
        map.len()
    }
    fn second(moves: Self::Parsed) -> Self::Output {
        let mut move_it = moves.into_iter().enumerate().cycle();
        let mut shape_it = ALL_SHAPES.iter().cycle();
        let mut map: Vec<u8> = vec![];

        let mut hash: AHashMap<_, (usize, usize)> = AHashMap::new();
        let mut len_from_cycles = 0;
        let mut rocks = 0;
        while rocks < 1_000_000_000_000usize {
            let (wind_pos, shape_pos) = simulate_block(&mut map, &mut move_it, &mut shape_it);

            if len_from_cycles == 0 {
                let mut hasher = DefaultHasher::new();
                for &u in &map[map.len().saturating_sub(64)..] {
                    u.hash(&mut hasher);
                }

                match hash.entry((hasher.finish(), wind_pos, shape_pos)) {
                    Entry::Occupied(entry) => {
                        let cycle_num_increase = rocks - entry.get().1;
                        let cycles_to_skip = (1_000_000_000_000usize - rocks) / cycle_num_increase;
                        len_from_cycles = cycles_to_skip * (map.len() - entry.get().0);
                        rocks += cycles_to_skip * cycle_num_increase;
                    }
                    Entry::Vacant(entry) => {
                        entry.insert((map.len(), rocks));
                    }
                }
            }
            rocks += 1;
        }
        map.len() + len_from_cycles
    }
}

fn simulate_block<'a, I: Iterator<Item = (usize, Direction)>, J: Iterator<Item = &'a Shape>>(
    map: &mut Vec<u8>,
    move_it: &mut I,
    shape_it: &mut J,
) -> (usize, u32) {
    let mut shaper = Shaper::new(*shape_it.next().unwrap());
    let mut blockpos = map.len() + 3;
    loop {
        let wind = move_it.next().unwrap();
        shaper.wind(wind.1, map_mask(&*map, blockpos));
        if blockpos == 0 || shaper.crashes(map_mask(&*map, blockpos - 1)) {
            for b in shaper.0.to_le_bytes() {
                if b > 0 {
                    if blockpos < map.len() {
                        map[blockpos] |= b;
                    } else {
                        map.push(b);
                    }
                    blockpos += 1;
                }
            }
            return (wind.0, shaper.0);
        }
        blockpos -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    fn parsed() -> <Day17 as Day>::Parsed {
        Day17::parse(INPUT.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day17::first(parsed()), 3068);
    }
    #[test]
    fn part2() {
        assert_eq!(Day17::second(parsed()), 1514285714288);
    }
}
