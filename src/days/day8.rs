use super::day::Day;
use anyhow::Result;

enum Direction {
    Up,
    Right,
    Down,
    Left,
}
fn check_visible(val: u8, map: &Vec<Vec<u8>>, direction: Direction, x: usize, y: usize) -> bool {
    fn check(val: u8, map: &Vec<Vec<u8>>, direction: Direction, x: usize, y: usize) -> bool {
        if map[y][x] >= val {
            false
        } else {
            check_visible(val, map, direction, x, y)
        }
    }
    match direction {
        Direction::Up => {
            if y == 0 {
                true
            } else {
                check(val, map, direction, x, y - 1)
            }
        }
        Direction::Right => {
            if x == map[y].len() - 1 {
                true
            } else {
                check(val, map, direction, x + 1, y)
            }
        }
        Direction::Down => {
            if y == map.len() - 1 {
                true
            } else {
                check(val, map, direction, x, y + 1)
            }
        }
        Direction::Left => {
            if x == 0 {
                true
            } else {
                check(val, map, direction, x - 1, y)
            }
        }
    }
}

fn count_visible(val: u8, map: &Vec<Vec<u8>>, direction: Direction, x: usize, y: usize) -> usize {
    fn count(val: u8, map: &Vec<Vec<u8>>, direction: Direction, x: usize, y: usize) -> usize {
        if map[y][x] >= val {
            1
        } else {
            count_visible(val, map, direction, x, y) + 1
        }
    }
    match direction {
        Direction::Up => {
            if y == 0 {
                0
            } else {
                count(val, map, direction, x, y - 1)
            }
        }
        Direction::Right => {
            if x == map[y].len() - 1 {
                0
            } else {
                count(val, map, direction, x + 1, y)
            }
        }
        Direction::Down => {
            if y == map.len() - 1 {
                0
            } else {
                count(val, map, direction, x, y + 1)
            }
        }
        Direction::Left => {
            if x == 0 {
                0
            } else {
                count(val, map, direction, x - 1, y)
            }
        }
    }
}

pub struct Day8;
impl Day for Day8 {
    type Parsed = Vec<Vec<u8>>;
    type Output = usize;

    fn parse(input: String) -> Result<Self::Parsed> {
        Ok(input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|i| i.to_digit(10).unwrap().try_into().unwrap())
                    .collect()
            })
            .collect())
    }
    fn first(map: Self::Parsed) -> Self::Output {
        map.iter()
            .enumerate()
            .map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .map(|(x, &item)| {
                        usize::from(
                            check_visible(item, &map, Direction::Up, x, y)
                                || check_visible(item, &map, Direction::Right, x, y)
                                || check_visible(item, &map, Direction::Down, x, y)
                                || check_visible(item, &map, Direction::Left, x, y),
                        )
                    })
                    .sum::<usize>()
            })
            .sum()
    }
    fn second(map: Self::Parsed) -> Self::Output {
        map.iter()
            .enumerate()
            .filter_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .map(|(x, &item)| {
                        count_visible(item, &map, Direction::Up, x, y)
                            * count_visible(item, &map, Direction::Right, x, y)
                            * count_visible(item, &map, Direction::Down, x, y)
                            * count_visible(item, &map, Direction::Left, x, y)
                    })
                    .max()
            })
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "30373
25512
65332
33549
35390";
    fn parsed() -> <Day8 as Day>::Parsed {
        Day8::parse(INPUT.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day8::first(parsed()), 21);
    }
    #[test]
    fn part2() {
        assert_eq!(Day8::second(parsed()), 8);
    }
}
