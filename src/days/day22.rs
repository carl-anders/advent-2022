use super::day::Day;
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
    DrawRight,
    DrawDown,
    DrawLeft,
    DrawUp,
}

#[derive(Debug, Clone, Copy)]
pub enum Movement {
    Num(u8),
    Right,
    Left,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}
impl Direction {
    fn movement(&mut self, movement: Movement) {
        use Direction::*;
        match movement {
            Movement::Num(_) => {}
            Movement::Right => {
                *self = match self {
                    Up => Right,
                    Right => Down,
                    Down => Left,
                    Left => Up,
                }
            }
            Movement::Left => {
                *self = match self {
                    Up => Left,
                    Right => Up,
                    Down => Right,
                    Left => Down,
                }
            }
        }
    }
    fn turn_right(&mut self, times: usize) {
        for _ in 0..(times % 4) {
            self.movement(Movement::Right);
        }
    }
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\d+)|([A-Z])").unwrap();
}

pub struct Day22;
impl Day for Day22 {
    type Parsed = (Array2<Point>, Vec<Movement>);
    type Output = usize;

    fn parse(input: String) -> Result<Self::Parsed> {
        let (map, path) = input.split_once("\n\n").unwrap();
        let width = map.lines().map(|l| l.len()).max().unwrap();
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
                    match d.as_str() {
                        "L" => Movement::Left,
                        _ => Movement::Right,
                    }
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
        let mut current_pos = [0, top_left_x];
        let mut direction = Direction::Right;
        for mov in movements {
            direction.movement(mov);
            if let Movement::Num(num) = mov {
                for _ in 0..num {
                    move_by(&mut current_pos, direction, &array);
                }
            }
        }
        let row = current_pos[0] + 1;
        let col = current_pos[1] + 1;
        let fac = direction as usize;
        row * 1000 + col * 4 + fac
    }
    fn second((array, movements): Self::Parsed) -> Self::Output {
        let (height, width) = array.dim();
        #[derive(Debug, Clone, Copy)]
        pub enum IP {
            Right = 0,
            Bottom = 1,
            Left = 2,
            Top = 3,
        }

        let mut draw_array = array.clone();

        use gif::{Encoder, Frame, Repeat};
        use std::borrow::Cow;
        use std::fs::File;
        // empty: fffcf2 - 0
        // bg: ccc5b9 - 1
        // rock: 403d39 - 2
        // arrow: eb5e28 -3
        let color_map = &[
            0xFF, 0xFc, 0xF2, 0xCC, 0xC5, 0xB9, 0x40, 0x3d, 0x39, 0xEB, 0x5E, 0x28,
        ];
        let mut image = File::create("day22_anim.gif").unwrap();
        let mut encoder = Encoder::new(&mut image, 450, 600, color_map).unwrap();
        encoder.set_repeat(Repeat::Infinite).unwrap();

        fn _w_frame(encoder: &mut Encoder<&mut File>, map: &Array2<Point>) {
            let (width, height) = (450, 600);
            let mut state = [0; 450 * 600];
            let mut frame = Frame::default();
            frame.width = width;
            frame.height = height;
            for y in 0..(200 as usize) {
                for x in 0..(150 as usize) {
                    let point = map[[y as usize, x as usize]];
                    let draw = match point {
                        Point::Nothing => [[0; 3]; 3],
                        Point::Open => [[1; 3]; 3],
                        Point::Wall => [[2; 3]; 3],
                        Point::DrawRight => [[1, 3, 1], [1, 1, 3], [1, 3, 1]],
                        Point::DrawDown => [[1, 1, 1], [3, 1, 3], [1, 3, 1]],
                        Point::DrawLeft => [[1, 3, 1], [3, 1, 1], [1, 3, 1]],
                        Point::DrawUp => [[1, 3, 1], [3, 1, 3], [1, 1, 1]],
                    };
                    if point != Point::Nothing {
                        for dy in 0..3 {
                            for dx in 0..3 {
                                state[((y * 3 + dy) * width as usize + x * 3 + dx)] = draw[dy][dx];
                            }
                        }
                    }
                }
            }
            frame.buffer = Cow::Borrowed(&state);
            encoder.write_frame(&frame).unwrap();
        }

        let portals = [
            // (right, down, left, up)
            // ((index, (add right x times)))
            [(0, IP::Left), (0, IP::Left), (0, IP::Left), (0, IP::Left)], // [0]
            [(2, IP::Left), (4, IP::Top), (6, IP::Left), (9, IP::Left)],  // 1
            [
                (7, IP::Right),
                (4, IP::Right),
                (1, IP::Right),
                (9, IP::Bottom),
            ], // 2
            [(0, IP::Left), (0, IP::Left), (0, IP::Left), (0, IP::Left)], // [3]
            [(2, IP::Bottom), (7, IP::Top), (6, IP::Top), (1, IP::Bottom)], // 4
            [(0, IP::Left), (0, IP::Left), (0, IP::Left), (0, IP::Left)], // [5]
            [(7, IP::Left), (9, IP::Top), (1, IP::Left), (4, IP::Left)],  // 6
            [
                (2, IP::Right),
                (9, IP::Right),
                (6, IP::Right),
                (4, IP::Bottom),
            ], // 7
            [(0, IP::Left), (0, IP::Left), (0, IP::Left), (0, IP::Left)], // [8]
            [(7, IP::Bottom), (2, IP::Top), (1, IP::Top), (6, IP::Bottom)], // 9
            [(0, IP::Left), (0, IP::Left), (0, IP::Left), (0, IP::Left)], // [10]
            [(0, IP::Left), (0, IP::Left), (0, IP::Left), (0, IP::Left)], // [11]
        ];
        let sector_size = (height / 3).min(width / 3);
        let sector_pos_to_pos = |pos: (usize, [usize; 2])| -> [usize; 2] {
            let row = pos.0 / 3;
            let col = pos.0 % 3;
            [pos.1[0] + row * sector_size, pos.1[1] + col * sector_size]
        };
        let mut position = (1, [0, 0]);

        let mut direction = Direction::Right;
        for mov in movements {
            direction.movement(mov);
            if let Movement::Num(num) = mov {
                for _ in 0..num {
                    let mut try_position = position;
                    let mut try_direction = direction;
                    //println!("Current pos: {:?}", sector_pos_to_pos(current_pos));
                    match direction {
                        Direction::Right => {
                            if position.1[1] == sector_size - 1 {
                                /* println!("Current pos: {current_pos:?}, direction: {direction:?}"); */
                                let portal = portals[position.0][0];
                                try_position.0 = portal.0;
                                // Changing direction is [0]
                                match portal.1 {
                                    IP::Right => {
                                        try_position.1[0] = sector_size - position.1[0] - 1;
                                        try_position.1[1] = sector_size - 1;
                                    }
                                    IP::Bottom => {
                                        try_position.1[0] = sector_size - 1;
                                        try_position.1[1] = position.1[0];
                                    }
                                    IP::Left => {
                                        try_position.1[1] = 0;
                                    }
                                    IP::Top => {
                                        // Untested direction. Might not work.
                                        try_position.1[0] = 0;
                                        try_position.1[1] = position.1[0]
                                    }
                                }
                                try_direction.turn_right(portal.1 as usize + 2);

                                /* println!(
                                    "Going Right to {} with portal {portal:?}, checking position: {:?}, ",
                                    test_pos.0, test_pos.1
                                ); */
                            } else {
                                try_position.1[1] += 1;
                            }
                        }
                        Direction::Down => {
                            if position.1[0] == sector_size - 1 {
                                /* println!("Current pos: {current_pos:?}, direction: {direction:?}"); */
                                let portal = portals[position.0][1];
                                try_position.0 = portal.0;
                                // Changing direction is [1]
                                match portal.1 {
                                    IP::Right => {
                                        try_position.1[0] = position.1[1];
                                        try_position.1[1] = sector_size - 1;
                                    }
                                    IP::Bottom => {
                                        // Untested direction. Might not work.
                                        try_position.1[0] = sector_size - position.1[1] - 1;
                                        try_position.1[1] = sector_size - 1;
                                    }
                                    IP::Left => {
                                        // Untested direction. Might not work.
                                        try_position.1[0] = position.1[1];
                                        try_position.1[1] = 0;
                                    }
                                    IP::Top => {
                                        try_position.1[0] = 0;
                                    }
                                }
                                try_direction.turn_right(portal.1 as usize + 1);
                                /* println!(
                                    "Going Down to {} with portal {portal:?}, checking position: {:?}, ",
                                    test_pos.0, test_pos.1
                                ); */
                            } else {
                                try_position.1[0] += 1;
                            }
                        }
                        Direction::Left => {
                            if position.1[1] == 0 {
                                /* println!("Current pos: {current_pos:?}, direction: {direction:?}"); */
                                let portal = portals[position.0][2];
                                try_position.0 = portal.0;
                                // Changing direction is [0]
                                match portal.1 {
                                    IP::Right => {
                                        try_position.1[1] = sector_size - 1;
                                    }
                                    IP::Bottom => {
                                        try_position.1[0] = sector_size - 1;
                                        try_position.1[1] = position.1[0];
                                    }
                                    IP::Left => {
                                        try_position.1[0] = sector_size - position.1[0] - 1;
                                        try_position.1[1] = 0;
                                    }
                                    IP::Top => {
                                        try_position.1[0] = 0;
                                        try_position.1[1] = position.1[0]
                                    }
                                }
                                try_direction.turn_right(portal.1 as usize);
                                /* println!(
                                    "Going Left to {} with portal {portal:?}, checking position: {:?}, ",
                                    test_pos.0, test_pos.1
                                ); */
                            } else {
                                try_position.1[1] -= 1;
                            }
                        }
                        Direction::Up => {
                            if position.1[0] == 0 {
                                /* println!("Current pos: {current_pos:?}, direction: {direction:?}"); */
                                let portal = portals[position.0][3];
                                try_position.0 = portal.0;
                                // Changing direction is [1]
                                match portal.1 {
                                    IP::Right => {
                                        // Untested direction. Might not work.
                                        try_position.1[0] = sector_size - position.1[1] - 1;
                                        try_position.1[1] = sector_size - 1;
                                    }
                                    IP::Bottom => {
                                        try_position.1[0] = sector_size - 1;
                                    }
                                    IP::Left => {
                                        try_position.1[0] = position.1[1];
                                        try_position.1[1] = 0;
                                    }
                                    IP::Top => {
                                        // Untested direction. Might not work.
                                        try_position.1[0] = 0;
                                    }
                                }
                                try_direction.turn_right(portal.1 as usize + 3);
                                /* println!(
                                    "Going Up to {} with portal {portal:?}, checking position: {:?}, ",
                                    test_pos.0, test_pos.1
                                ); */
                            } else {
                                try_position.1[0] -= 1;
                            }
                        }
                    }
                    if try_position != position && array[sector_pos_to_pos(try_position)] == Point::Open {
                        draw_array[sector_pos_to_pos(position)] = match direction {
                            Direction::Right => Point::DrawRight,
                            Direction::Down => Point::DrawDown,
                            Direction::Left => Point::DrawLeft,
                            Direction::Up => Point::DrawUp,
                        };
                        //_w_frame(&mut encoder, &draw_array);
                        direction = try_direction;
                        position = try_position;
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

fn move_by(current_pos: &mut [usize; 2], direction: Direction, array: &Array2<Point>) {
    let (height, width) = array.dim();
    let mut look_pos = *current_pos;
    loop {
        match direction {
            Direction::Up => {
                look_pos[0] = look_pos[0].wrapping_sub(1).min(height - 1);
            }
            Direction::Right => {
                look_pos[1] = if look_pos[1] == width - 1 {
                    0
                } else {
                    look_pos[1] + 1
                };
            }
            Direction::Down => {
                look_pos[0] = if look_pos[0] == height - 1 {
                    0
                } else {
                    look_pos[0] + 1
                };
            }
            Direction::Left => {
                look_pos[1] = look_pos[1].wrapping_sub(1).min(width - 1);
            }
        }

        match array[look_pos] {
            Point::Nothing => {}
            Point::Open => {
                *current_pos = look_pos;
                break;
            }
            Point::Wall => {
                break;
            }
            _ => {}
        }
    }
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
