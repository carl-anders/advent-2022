use super::day::Day;
use ahash::{HashMap, HashMapExt};
use anyhow::Result;
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
pub struct Elf {
    proposed_move: Option<(isize, isize)>,
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
                    map.insert(
                        (x, y),
                        Elf {
                            proposed_move: None,
                        },
                    );
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
        let mut round = 0;
        loop {
            round += 1;
            if elf_round(&mut map, &mut test_direction) == 0 {
                break;
            }
        }
        round
    }
}

fn elf_round(map: &mut HashMap<(isize, isize), Elf>, test_direction: &mut i32) -> usize {
    let mut proposed_moves: HashMap<(isize, isize), usize> = HashMap::new();
    let keys: Vec<_> = map.clone().into_keys().map(|pos| pos).collect();
    for pos in &keys {
        map.get_mut(pos).unwrap().proposed_move = None;
        let n = map.contains_key(&(pos.0, pos.1 - 1));
        let ne = map.contains_key(&(pos.0 + 1, pos.1 - 1));
        let e = map.contains_key(&(pos.0 + 1, pos.1));
        let se = map.contains_key(&(pos.0 + 1, pos.1 + 1));
        let s = map.contains_key(&(pos.0, pos.1 + 1));
        let sw = map.contains_key(&(pos.0 - 1, pos.1 + 1));
        let w = map.contains_key(&(pos.0 - 1, pos.1));
        let nw = map.contains_key(&(pos.0 - 1, pos.1 - 1));
        if n || ne || e || se || s || sw || w || nw {
            let proposed = match *test_direction {
                0 => {
                    if !nw && !n && !ne {
                        Some((pos.0, pos.1 - 1))
                    } else if !sw && !s && !se {
                        Some((pos.0, pos.1 + 1))
                    } else if !nw && !w && !sw {
                        Some((pos.0 - 1, pos.1))
                    } else if !ne && !e && !se {
                        Some((pos.0 + 1, pos.1))
                    } else {
                        None
                    }
                }
                1 => {
                    if !sw && !s && !se {
                        Some((pos.0, pos.1 + 1))
                    } else if !nw && !w && !sw {
                        Some((pos.0 - 1, pos.1))
                    } else if !ne && !e && !se {
                        Some((pos.0 + 1, pos.1))
                    } else if !nw && !n && !ne {
                        Some((pos.0, pos.1 - 1))
                    } else {
                        None
                    }
                }
                2 => {
                    if !nw && !w && !sw {
                        Some((pos.0 - 1, pos.1))
                    } else if !ne && !e && !se {
                        Some((pos.0 + 1, pos.1))
                    } else if !nw && !n && !ne {
                        Some((pos.0, pos.1 - 1))
                    } else if !sw && !s && !se {
                        Some((pos.0, pos.1 + 1))
                    } else {
                        None
                    }
                }
                _ => {
                    if !ne && !e && !se {
                        Some((pos.0 + 1, pos.1))
                    } else if !nw && !n && !ne {
                        Some((pos.0, pos.1 - 1))
                    } else if !sw && !s && !se {
                        Some((pos.0, pos.1 + 1))
                    } else if !nw && !w && !sw {
                        Some((pos.0 - 1, pos.1))
                    } else {
                        None
                    }
                }
            };
            if let Some(new_pos) = proposed {
                *proposed_moves.entry(new_pos).or_default() += 1;
                map.get_mut(pos).unwrap().proposed_move = Some(new_pos);
            }
        }
    }
    *test_direction = (*test_direction + 1) % 4;
    proposed_moves.retain(|_, num| *num == 1);
    for pos in &keys {
        if let Some(mov) = map.get(pos).unwrap().proposed_move {
            if proposed_moves.contains_key(&mov) {
                if let Some(elf) = map.remove(pos) {
                    map.insert(mov, elf);
                }
            }
        }
    }
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
