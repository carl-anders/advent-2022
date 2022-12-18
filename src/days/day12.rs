use super::day::Day;
use anyhow::Result;
use pathfinding::prelude::bfs;
// use pathfinding::prelude::dijkstra;
use smallvec::{smallvec, SmallVec};

fn explore_to_top(map: &[Vec<u32>], start: Position, top: Position) -> usize {
    let max = (map.len() - 1, map[1].len() - 1);
    let result = bfs(
        &(start.y, start.x),
        |&(x, y)| {
            let mut v: SmallVec<[(usize, usize); 4]> = smallvec![];
            if x < max.0 && map[x][y] + 1 >= map[x + 1][y] {
                v.push((x + 1, y));
            }
            if y < max.1 && map[x][y] + 1 >= map[x][y + 1] {
                v.push((x, y + 1));
            }
            if x > 0 && map[x][y] + 1 >= map[x - 1][y] {
                v.push((x - 1, y));
            }
            if y > 0 && map[x][y] + 1 >= map[x][y - 1] {
                v.push((x, y - 1));
            }
            v
        },
        |&p| p == (top.y, top.x),
    );
    result.unwrap().len() - 1
}
/* fn explore_to_top_dijkstra(map: &[Vec<u32>], start: Position, top: Position) -> usize {
    let max = (map.len() - 1, map[1].len() - 1);
    let result = dijkstra(
        &(start.y, start.x),
        |&(x, y)| {
            let mut v: SmallVec<[((usize, usize), i32); 4]> = smallvec![];
            if x < max.0 && map[x][y] + 1 >= map[x + 1][y] {
                v.push(((x + 1, y), 1));
            }
            if y < max.1 && map[x][y] + 1 >= map[x][y + 1] {
                v.push(((x, y + 1), 1));
            }
            if x > 0 && map[x][y] + 1 >= map[x - 1][y] {
                v.push(((x - 1, y), 1));
            }
            if y > 0 && map[x][y] + 1 >= map[x][y - 1] {
                v.push(((x, y - 1), 1));
            }
            v
        },
        |&p| p == (top.y, top.x),
    );
    result.unwrap().1 as usize
} */
fn explore_from_top(map: &[Vec<u32>], top: Position) -> usize {
    let max = (map.len() - 1, map[1].len() - 1);
    let result = bfs(
        &(top.y, top.x),
        |&(x, y)| {
            let mut v: SmallVec<[(usize, usize); 4]> = smallvec![];
            if x < max.0 && map[x + 1][y] + 1 >= map[x][y] {
                v.push((x + 1, y));
            }
            if y < max.1 && map[x][y + 1] + 1 >= map[x][y] {
                v.push((x, y + 1));
            }
            if x > 0 && map[x - 1][y] + 1 >= map[x][y] {
                v.push((x - 1, y));
            }
            if y > 0 && map[x][y - 1] + 1 >= map[x][y] {
                v.push((x, y - 1));
            }
            v
        },
        |&p| map[p.0][p.1] == 0,
    );
    result.unwrap().len() - 1
}
/* fn explore_from_top_djikstra(map: &[Vec<u32>], top: Position) -> usize {
    let max = (map.len() - 1, map[1].len() - 1);
    let result = dijkstra(
        &(top.y, top.x),
        |&(x, y)| {
            let mut v: SmallVec<[((usize, usize), i32); 4]> = smallvec![];
            if x < max.0 && map[x + 1][y] + 1 >= map[x][y] {
                v.push(((x + 1, y), 1));
            }
            if y < max.1 && map[x][y + 1] + 1 >= map[x][y] {
                v.push(((x, y + 1), 1));
            }
            if x > 0 && map[x - 1][y] + 1 >= map[x][y] {
                v.push(((x - 1, y), 1));
            }
            if y > 0 && map[x][y - 1] + 1 >= map[x][y] {
                v.push(((x, y - 1), 1));
            }
            v
        },
        |&p| map[p.0][p.1] == 0,
    );
    result.unwrap().1 as usize
} */

/* fn print(map: &[Vec<u32>], res: &Vec<(usize, usize)>) {
    let mut draw: Vec<Vec<char>> = map
        .iter()
        .map(|l| {
            l.iter()
                .map(|c| u8::try_from('a' as u32 + c).unwrap() as char)
                .collect()
        })
        .collect();
    for l in &draw {
        println!("{}", l.iter().collect::<String>());
    }
    for a in res {
        std::thread::sleep(std::time::Duration::from_millis(100));
        println!();
        println!();
        println!();
        draw[a.0][a.1] = 'X';
        for l in &draw {
            println!("{}", l.iter().collect::<String>());
        }
    }
} */

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    x: usize,
    y: usize,
}

pub struct Day12;
impl Day for Day12 {
    type Parsed = (Vec<Vec<u32>>, Position, Position);
    type Output = usize;

    fn parse(input: String) -> Result<Self::Parsed> {
        let mut start = Position { x: 0, y: 0 };
        let mut end = start;
        let v = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        'a'..='z' => c as u32 - 'a' as u32,
                        'S' => {
                            start = Position { x, y };
                            0
                        }
                        'E' => {
                            end = Position { x, y };
                            25
                        }
                        _ => panic!(),
                    })
                    .collect()
            })
            .collect();
        Ok((v, start, end))
    }
    fn first((map, start, end): Self::Parsed) -> Self::Output {
        explore_to_top(&map, start, end)
    }
    fn second((map, _start, end): Self::Parsed) -> Self::Output {
        explore_from_top(&map, end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
    fn parsed() -> <Day12 as Day>::Parsed {
        Day12::parse(INPUT.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day12::first(parsed()), 31);
    }
    #[test]
    fn part2() {
        assert_eq!(Day12::second(parsed()), 29);
    }
}
