use super::{
    day::Day,
    grid2d::{Direction4Way, Position2D},
};
use ahash::{HashSet, HashSetExt};
use anyhow::Result;
use ndarray::Array2;
use pathfinding::prelude::dijkstra;

type Pos = Position2D<usize>;
type Dir = Direction4Way;

#[derive(Debug, Clone, Copy)]
pub struct Wind {
    pos: Pos,
    dir: Dir,
}
impl Wind {
    fn mov(&mut self) {
        self.pos = self.pos + self.dir;
    }
}

#[derive(Debug, Clone)]
pub struct Map {
    time: usize,
    walls: HashSet<Pos>,
    winds: Vec<Wind>,
    size: Pos,
    cache: Vec<Array2<bool>>,
}
impl Map {
    fn new(input: &str) -> Self {
        let mut walls = HashSet::new();
        let mut winds = Vec::new();
        let (mut maxx, mut maxy) = (0, 0);
        for (y, line) in input.lines().enumerate() {
            maxy = maxy.max(y);
            for (x, c) in line.chars().enumerate() {
                maxx = maxx.max(x);
                let pos = Pos { x, y };
                match c {
                    '#' => {
                        walls.insert(pos);
                    }
                    '>' => {
                        winds.push(Wind {
                            pos,
                            dir: Dir::Right,
                        });
                    }
                    'v' => {
                        winds.push(Wind {
                            pos,
                            dir: Dir::Down,
                        });
                    }
                    '<' => {
                        winds.push(Wind {
                            pos,
                            dir: Dir::Left,
                        });
                    }
                    '^' => {
                        winds.push(Wind { pos, dir: Dir::Up });
                    }
                    _ => {}
                }
            }
        }
        let mut s = Self {
            time: 0,
            walls,
            winds,
            size: Pos::new(maxx + 1, maxy + 1),
            cache: vec![],
        };
        s.make_cache();
        s
    }
    fn _print_time(&self, time: usize) {
        for y in 0..self.size.y {
            for x in 0..self.size.y {
                if self.cache[time][(x, y)] {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
    fn is_blocked(&mut self, time: usize, pos: Pos) -> bool {
        while self.time < time {
            self.simulate();
        }
        *self.cache[time].get(pos.xy()).unwrap_or(&true)
    }
    fn simulate(&mut self) {
        for wind in &mut self.winds {
            wind.mov();
            if self.walls.contains(&wind.pos) {
                match wind.dir {
                    Dir::Right => wind.pos.x = 1,
                    Dir::Down => wind.pos.y = 1,
                    Dir::Left => wind.pos.x = self.size.x - 2,
                    Dir::Up => wind.pos.y = self.size.y - 2,
                }
            }
        }
        self.time += 1;
        self.make_cache();
    }
    fn make_cache(&mut self) {
        if self.cache.len() == self.time {
            let mut map = Array2::<bool>::default(self.size.xy());
            for wall in &self.walls {
                map[wall.xy()] = true;
            }
            for wind in &self.winds {
                map[wind.pos.xy()] = true;
            }
            self.cache.push(map);
        }
    }
}

pub struct Day24;
impl Day for Day24 {
    type Parsed = Map;
    type Output = usize;

    fn parse(input: String) -> Result<Self::Parsed> {
        Ok(Map::new(&input))
    }
    fn first(mut map: Self::Parsed) -> Self::Output {
        let end = map.size.sub_x(2).sub_y(1);
        let start = Pos::new(1, 0);
        shortest_path(&mut map, start, end, 0)
    }
    fn second(mut map: Self::Parsed) -> Self::Output {
        let end = map.size.sub_x(2).sub_y(1);
        let start = Pos::new(1, 0);
        let mut total_time = shortest_path(&mut map, start, end, 0);
        total_time += shortest_path(&mut map, end, start, total_time);
        total_time += shortest_path(&mut map, start, end, total_time);
        total_time
    }
}

fn shortest_path(map: &mut Map, start: Pos, end: Pos, start_time: usize) -> usize {
    dijkstra(
        &(start_time, start),
        |(time, pos)| {
            let mut paths = vec![];
            let time = time+1;
            if !map.is_blocked(time, *pos) {
                paths.push(((time, *pos), 1));
            }
            if !map.is_blocked(time, pos.add_x(1)) {
                paths.push(((time, pos.add_x(1)), 1));
            }
            if !map.is_blocked(time, pos.add_y(1)) {
                paths.push(((time, pos.add_y(1)), 1));
            }
            if !map.is_blocked(time, pos.wrapping_sub_x(&1)) {
                paths.push(((time, pos.wrapping_sub_x(&1)), 1));
            }
            if !map.is_blocked(time, pos.wrapping_sub_y(&1)) {
                paths.push(((time, pos.wrapping_sub_y(&1)), 1));
            }
            paths
        },
        |(_, pos)| *pos == end,
    )
    .unwrap()
    .1
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";
    fn parsed() -> <Day24 as Day>::Parsed {
        Day24::parse(INPUT.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day24::first(parsed()), 18);
    }
    #[test]
    fn part2() {
        assert_eq!(Day24::second(parsed()), 54);
    }
}
