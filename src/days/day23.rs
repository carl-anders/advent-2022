#![allow(clippy::cast_possible_wrap)]
use super::day::Day;
use ahash::{HashMap, HashMapExt};
use anyhow::Result;
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
pub struct Elf {
    proposed: Option<(isize, isize)>,
}

pub struct Day23;
impl Day for Day23 {
    type Parsed = HashMap<(isize, isize), Elf>;
    type Output = usize;

    fn parse(input: String) -> Result<Self::Parsed> {
        let mut map = HashMap::new();
        for (y, line) in input.lines().enumerate() {
            for (x, &c) in line.as_bytes().iter().enumerate() {
                let (x, y) = (x as isize, y as isize);
                if c == b'#' {
                    map.insert((x, y), Elf { proposed: None });
                }
            }
        }
        Ok(map)
    }
    fn first(mut map: Self::Parsed) -> Self::Output {
        let mut test_direction = 0;
        for _ in 0..10 {
            elf_round(&mut map, &mut test_direction);
        }
        let x_min_max = map.keys().minmax_by_key(|k| k.0).into_option().unwrap();
        let y_min_max = map.keys().minmax_by_key(|k| k.1).into_option().unwrap();
        let area = (x_min_max.1 .0 - x_min_max.0 .0 + 1) * (y_min_max.1 .1 - y_min_max.0 .1 + 1);
        area as usize - map.len()
    }
    fn second(mut map: Self::Parsed) -> Self::Output {
        let mut test_direction = 0;
        for round in 0.. {
            if elf_round(&mut map, &mut test_direction) == 0 {
                return round + 1;
            }
        }
        0
    }
}
const DIRS: [(isize, isize); 8] = [
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];
const PATHS: [[usize; 3]; 4] = [[7, 0, 1], [3, 4, 5], [5, 6, 7], [1, 2, 3]];

fn elf_possible_move(
    map: &HashMap<(isize, isize), Elf>,
    pos: (isize, isize),
    test: usize,
) -> Option<(isize, isize)> {
    let checks: [bool; 8] = DIRS
        .into_iter()
        .map(|(x, y)| map.contains_key(&(pos.0 + x, pos.1 + y)))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    if checks.iter().all(|c| !c) {
        return None;
    }
    for dir in [
        PATHS[test % 4],
        PATHS[(test + 1) % 4],
        PATHS[(test + 2) % 4],
        PATHS[(test + 3) % 4],
    ] {
        if !checks[dir[0]] && !checks[dir[1]] && !checks[dir[2]] {
            return Some((pos.0 + DIRS[dir[1]].0, pos.1 + DIRS[dir[1]].1));
        }
    }
    None
}
fn elf_round(map: &mut HashMap<(isize, isize), Elf>, test_direction: &mut usize) -> usize {
    let mut proposed_moves: HashMap<(isize, isize), usize> = HashMap::new();
    let keys: Vec<_> = map.clone().into_keys().collect();
    for pos in &keys {
        let mov = elf_possible_move(map, *pos, *test_direction);
        map.get_mut(pos).unwrap().proposed = mov;
        if let Some(new_pos) = mov {
            *proposed_moves.entry(new_pos).or_default() += 1;
        }
    }
    proposed_moves.retain(|_, num| *num == 1);
    for pos in &keys {
        if let Some(mov) = map.get(pos).unwrap().proposed {
            if proposed_moves.contains_key(&mov) {
                if let Some(elf) = map.remove(pos) {
                    map.insert(mov, elf);
                }
            }
        }
    }
    *test_direction = (*test_direction + 1) % 4;
    proposed_moves.len()
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
