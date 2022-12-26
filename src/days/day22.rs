use super::{
    day::Day,
    grid2d::{Direction4Way, Position2D, Turn},
};
use anyhow::Result;
use itertools::Itertools;
use lazy_static::lazy_static;
use ndarray::Array2;
use regex::Regex;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Point {
    #[default]
    Nothing,
    Open,
    Wall,
}

#[derive(Debug, Clone, Copy)]
pub enum Movement {
    Num(u8),
    Turn(Turn),
}
type Dir = Direction4Way;
type Pos = Position2D<usize>;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\d+)|([A-Z])").unwrap();
}

pub struct Day22;
impl Day for Day22 {
    type Parsed = (Array2<Point>, Vec<Movement>);
    type Output = usize;

    fn parse(input: String) -> Result<Self::Parsed> {
        let (map, path) = input.split_once("\n\n").unwrap();
        let width = map.lines().map(str::len).max().unwrap();
        let height = map.lines().count();
        let mut array = Array2::<Point>::default((height, width));
        for (y, line) in map.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '.' => array[[y, x]] = Point::Open,
                    '#' => array[[y, x]] = Point::Wall,
                    _ => {}
                }
            }
        }
        let movements: Vec<_> = RE
            .captures_iter(path)
            .map(|p| {
                if let Some(n) = p.get(1) {
                    Movement::Num(n.as_str().parse().unwrap())
                } else if let Some(d) = p.get(2) {
                    Movement::Turn(match d.as_str() {
                        "L" => Turn::Left,
                        _ => Turn::Right,
                    })
                } else {
                    panic!()
                }
            })
            .collect();
        Ok((array, movements))
    }
    fn first((array, movements): Self::Parsed) -> Self::Output {
        let top_left_x = {
            let row = array.row(0);
            row.iter().find_position(|&&p| p == Point::Open).unwrap().0
        };
        let mut current_pos = Pos::new(top_left_x, 0);
        let mut direction = Dir::Right;
        for mov in movements {
            match mov {
                Movement::Turn(turn) => direction = direction + turn,
                Movement::Num(num) => {
                    for _ in 0..num {
                        current_pos = move_by(current_pos, direction, &array);
                    }
                }
            }
        }
        (current_pos.y + 1) * 1000 + (current_pos.x + 1) * 4 + direction as usize
    }
    fn second((array, movements): Self::Parsed) -> Self::Output {
        let (height, width) = array.dim();

        let sector_size = (height / 3).min(width / 3);
        let sector_pos_to_pos = |pos: (usize, Pos)| -> [usize; 2] {
            let row = pos.0 / 3;
            let col = pos.0 % 3;
            [pos.1.y + row * sector_size, pos.1.x + col * sector_size]
        };
        let mut position = (1, Pos::new(0, 0));

        let mut direction = Dir::Right;
        for mov in movements {
            match mov {
                Movement::Turn(turn) => direction = direction + turn,
                Movement::Num(num) => {
                    for _ in 0..num {
                        let (try_position, try_direction) =
                            cube_next_pos(position, direction, sector_size);
                        if try_position != position
                            && array[sector_pos_to_pos(try_position)] == Point::Open
                        {
                            direction = try_direction;
                            position = try_position;
                        }
                    }
                }
            }
        }
        let pos = sector_pos_to_pos(position);
        let row = pos[0] + 1;
        let col = pos[1] + 1;
        let fac = direction as usize;
        row * 1000 + col * 4 + fac
    }
}

fn move_by(current_pos: Pos, direction: Dir, array: &Array2<Point>) -> Pos {
    let (height, width) = array.dim();
    let mut look_pos = current_pos;
    loop {
        match direction {
            Dir::Up => {
                look_pos.y = look_pos.y.wrapping_sub(1).min(height - 1);
            }
            Dir::Right => {
                look_pos.x = if look_pos.x == width - 1 {
                    0
                } else {
                    look_pos.x + 1
                };
            }
            Dir::Down => {
                look_pos.y = if look_pos.y == height - 1 {
                    0
                } else {
                    look_pos.y + 1
                };
            }
            Dir::Left => {
                look_pos.x = look_pos.x.wrapping_sub(1).min(width - 1);
            }
        }

        match array[look_pos.yx()] {
            Point::Open => {
                return look_pos;
            }
            Point::Wall => {
                return current_pos;
            }
            Point::Nothing => {}
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum P {
    Right = 0,
    Bottom = 1,
    Left = 2,
    Top = 3,
}

const PORTALS: [[(usize, P); 4]; 12] = [
    // (right, down, left, up)
    // ((portal index, portal exit))
    [(0, P::Left), (0, P::Left), (0, P::Left), (0, P::Left)], // [0]
    [(2, P::Left), (4, P::Top), (6, P::Left), (9, P::Left)],  // 1
    [(7, P::Right), (4, P::Right), (1, P::Right), (9, P::Bottom)], // 2
    [(0, P::Left), (0, P::Left), (0, P::Left), (0, P::Left)], // [3]
    [(2, P::Bottom), (7, P::Top), (6, P::Top), (1, P::Bottom)], // 4
    [(0, P::Left), (0, P::Left), (0, P::Left), (0, P::Left)], // [5]
    [(7, P::Left), (9, P::Top), (1, P::Left), (4, P::Left)],  // 6
    [(2, P::Right), (9, P::Right), (6, P::Right), (4, P::Bottom)], // 7
    [(0, P::Left), (0, P::Left), (0, P::Left), (0, P::Left)], // [8]
    [(7, P::Bottom), (2, P::Top), (1, P::Top), (6, P::Bottom)], // 9
    [(0, P::Left), (0, P::Left), (0, P::Left), (0, P::Left)], // [10]
    [(0, P::Left), (0, P::Left), (0, P::Left), (0, P::Left)], // [11]
];

#[allow(clippy::too_many_lines)]
const fn cube_next_pos(pos: (usize, Pos), dir: Dir, sector_size: usize) -> ((usize, Pos), Dir) {
    let mut try_pos = pos;
    let mut try_dir = dir;
    match dir {
        Dir::Right => {
            if pos.1.x == sector_size - 1 {
                let portal = PORTALS[pos.0][0];
                try_pos.0 = portal.0;
                match portal.1 {
                    P::Right => {
                        try_pos.1.y = sector_size - pos.1.y - 1;
                        try_pos.1.x = sector_size - 1;
                    }
                    P::Bottom => {
                        try_pos.1.y = sector_size - 1;
                        try_pos.1.x = pos.1.y;
                    }
                    P::Left => {
                        try_pos.1.x = 0;
                    }
                    P::Top => {
                        // Untested direction. Might not work.
                        try_pos.1.y = 0;
                        try_pos.1.x = pos.1.y;
                    }
                }
                try_dir = try_dir.turn_right(portal.1 as usize + 2);
            } else {
                try_pos.1.x += 1;
            }
        }
        Dir::Down => {
            if pos.1.y == sector_size - 1 {
                let portal = PORTALS[pos.0][1];
                try_pos.0 = portal.0;
                match portal.1 {
                    P::Right => {
                        try_pos.1.y = pos.1.x;
                        try_pos.1.x = sector_size - 1;
                    }
                    P::Bottom => {
                        // Untested direction. Might not work.
                        try_pos.1.y = sector_size - pos.1.x - 1;
                        try_pos.1.x = sector_size - 1;
                    }
                    P::Left => {
                        // Untested direction. Might not work.
                        try_pos.1.y = pos.1.x;
                        try_pos.1.x = 0;
                    }
                    P::Top => {
                        try_pos.1.y = 0;
                    }
                }
                try_dir = try_dir.turn_right(portal.1 as usize + 1);
            } else {
                try_pos.1.y += 1;
            }
        }
        Dir::Left => {
            if pos.1.x == 0 {
                let portal = PORTALS[pos.0][2];
                try_pos.0 = portal.0;
                match portal.1 {
                    P::Right => {
                        try_pos.1.x = sector_size - 1;
                    }
                    P::Bottom => {
                        try_pos.1.y = sector_size - 1;
                        try_pos.1.x = pos.1.y;
                    }
                    P::Left => {
                        try_pos.1.y = sector_size - pos.1.y - 1;
                        try_pos.1.x = 0;
                    }
                    P::Top => {
                        try_pos.1.y = 0;
                        try_pos.1.x = pos.1.y;
                    }
                }
                try_dir = try_dir.turn_right(portal.1 as usize);
            } else {
                try_pos.1.x -= 1;
            }
        }
        Dir::Up => {
            if pos.1.y == 0 {
                let portal = PORTALS[pos.0][3];
                try_pos.0 = portal.0;
                match portal.1 {
                    P::Right => {
                        // Untested direction. Might not work.
                        try_pos.1.y = sector_size - pos.1.x - 1;
                        try_pos.1.x = sector_size - 1;
                    }
                    P::Bottom => {
                        try_pos.1.y = sector_size - 1;
                    }
                    P::Left => {
                        try_pos.1.y = pos.1.x;
                        try_pos.1.x = 0;
                    }
                    P::Top => {
                        // Untested direction. Might not work.
                        try_pos.1.y = 0;
                    }
                }
                try_dir = try_dir.turn_right(portal.1 as usize + 3);
            } else {
                try_pos.1.y -= 1;
            }
        }
    }
    (try_pos, try_dir)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";
    fn parsed() -> <Day22 as Day>::Parsed {
        Day22::parse(INPUT.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day22::first(parsed()), 6032);
    }
    #[test]
    fn part2() {
        //assert_eq!(Day22::second(parsed()), 0);
    }
}
