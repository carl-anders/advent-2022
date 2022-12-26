#![allow(clippy::cast_possible_wrap)]
use super::day::Day;
use crate::helpers::grid2d::{Direction8Way, Position2D};
use ahash::{HashMap, HashMapExt, HashSet};
use anyhow::Result;
use itertools::Itertools;
use smallvec::SmallVec;

type Pos = Position2D<isize>;
type Dir = Direction8Way;

pub struct Day23;
impl Day for Day23 {
    type Parsed = HashSet<Pos>;
    type Output = usize;

    fn parse(input: String) -> Result<Self::Parsed> {
        Ok(input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.as_bytes()
                    .iter()
                    .enumerate()
                    .filter_map(move |(x, &c)| {
                        (c == b'#').then(|| Pos::new(x as isize, y as isize))
                    })
            })
            .collect())
    }
    fn first(mut map: Self::Parsed) -> Self::Output {
        let mut test_direction = 0;
        for _ in 0..10 {
            elf_round(&mut map, &mut test_direction);
        }
        let xmm = map.iter().map(|k| k.x).minmax().into_option().unwrap();
        let ymm = map.iter().map(|k| k.y).minmax().into_option().unwrap();
        ((xmm.1 - xmm.0 + 1) * (ymm.1 - ymm.0 + 1)) as usize - map.len()
    }
    fn second(mut map: Self::Parsed) -> Self::Output {
        let mut test_direction = 0;
        for round in 1.. {
            if elf_round(&mut map, &mut test_direction) == 0 {
                return round;
            }
        }
        0
    }
}
const PATHS: [[Dir; 3]; 4] = [
    [Dir::NW, Dir::N, Dir::NE],
    [Dir::SE, Dir::S, Dir::SW],
    [Dir::SW, Dir::W, Dir::NW],
    [Dir::NE, Dir::E, Dir::SE],
];
fn elf_possible_move(map: &HashSet<Pos>, pos: Pos, test: usize) -> Option<Pos> {
    // Equivalent code but slower:
    // let checks = Dir::EVERY.map(|dir| map.contains(&(pos + dir)));
    let checks = [
        map.contains(&(pos + Dir::N)),
        map.contains(&(pos + Dir::NE)),
        map.contains(&(pos + Dir::E)),
        map.contains(&(pos + Dir::SE)),
        map.contains(&(pos + Dir::S)),
        map.contains(&(pos + Dir::SW)),
        map.contains(&(pos + Dir::W)),
        map.contains(&(pos + Dir::NW)),
    ];
    if checks.iter().all(|c| !c) {
        return None;
    }
    for dir in [
        PATHS[test % 4],
        PATHS[(test + 1) % 4],
        PATHS[(test + 2) % 4],
        PATHS[(test + 3) % 4],
    ] {
        if !checks[dir[0] as usize] && !checks[dir[1] as usize] && !checks[dir[2] as usize] {
            return Some(pos + dir[1]);
        }
    }
    None
}
fn elf_round(map: &mut HashSet<Pos>, test_direction: &mut usize) -> usize {
    let mut proposed: HashMap<Pos, SmallVec<[Pos; 4]>> = HashMap::new();
    for &pos in map.iter() {
        if let Some(new_pos) = elf_possible_move(map, pos, *test_direction) {
            proposed.entry(new_pos).or_default().push(pos);
        }
    }
    for (proposed, from_pos) in &proposed {
        if from_pos.len() == 1 {
            map.remove(&from_pos[0]);
            map.insert(*proposed);
        }
    }
    *test_direction = (*test_direction + 1) % 4;
    proposed.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";
    fn parsed() -> <Day23 as Day>::Parsed {
        Day23::parse(INPUT.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day23::first(parsed()), 110);
    }
    #[test]
    fn part2() {
        assert_eq!(Day23::second(parsed()), 20);
    }
}
