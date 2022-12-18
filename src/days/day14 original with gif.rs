use std::collections::HashSet;

use super::day::Day;
use anyhow::Result;
use itertools::Itertools;


#[allow(dead_code)]
fn print_map(map: &HashSet<(usize, usize)>, left: usize, right: usize, top: usize, bottom: usize) {
    for y in top..=bottom {
        for x in left..=right {
            if map.contains(&(x, y)) {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!()
    }
    println!("-----------------------")
}

pub struct Day14;
impl Day for Day14 {
    type Parsed = (HashSet<(usize, usize)>, usize);
    type Output = i32;

    fn parse(input: String) -> Result<Self::Parsed> {
        let paths: Vec<Vec<(usize, usize)>> = input
            .lines()
            .map(|line| {
                line.split(" -> ")
                    .map(|xy| {
                        xy.split(',')
                            .map(|v| v.parse().unwrap())
                            .collect_tuple()
                            .unwrap()
                    })
                    .collect()
            })
            .collect();
        let mut map = HashSet::new();
        let mut bottom = 0;
        for path in paths {
            for (a, b) in path.iter().tuple_windows() {
                if a.0 == b.0 {
                    let moves = if a.1 > b.1 { b.1..=a.1 } else { a.1..=b.1 };
                    for y in moves {
                        map.insert((a.0, y));
                        bottom = bottom.max(y);
                    }
                } else {
                    let moves = if a.0 > b.0 { b.0..=a.0 } else { a.0..=b.0 };
                    for x in moves {
                        map.insert((x, a.1));
                        bottom = bottom.max(a.1);
                    }
                }
            }
        }

        Ok((map, bottom))
    }
    fn first((mut map, bottom): Self::Parsed) -> Self::Output {
        //print_map(&map, 400, 520, 0, bottom);
        let mut sand = (500, 0);
        let mut total_sand = 0;
        loop {
            if sand.1 >= bottom {
                break;
            } else if !map.contains(&(sand.0, sand.1 + 1)) {
                sand.1 += 1;
            } else if !map.contains(&(sand.0 - 1, sand.1 + 1)) {
                sand = (sand.0 - 1, sand.1 + 1);
            } else if !map.contains(&(sand.0 + 1, sand.1 + 1)) {
                sand = (sand.0 + 1, sand.1 + 1);
            } else {
                // Can't sand no more
                map.insert(sand);
                total_sand += 1;
                sand = (500, 0);
            }
        }
        //print_map(&map, 400, 520, 0, bottom);

        total_sand
    }
    fn second((mut map, bottom): Self::Parsed) -> Self::Output {
        /*
        let original_map = map.clone();
        use gif::{Frame, Encoder, Repeat};
        use std::fs::File;
        use std::borrow::Cow;

        let color_map = &[0xA9, 0xB9, 0xCB, 229, 157, 111, 0x52, 0x2A, 0x2B];
        let mut image = File::create("day14_anim.gif").unwrap();
        let mut encoder = Encoder::new(&mut image, 360, 180, color_map).unwrap();
        encoder.set_repeat(Repeat::Infinite).unwrap();

        fn w_frame(encoder: &mut Encoder<&mut File>,  map: &HashSet<(usize, usize)>, original_map: &HashSet<(usize, usize)>) {
            let (width, height) = (360, 180);
            let mut state = [0;360*180];
            let mut frame = Frame::default();
            frame.width = width;
            frame.height = height;
            for y in 0..height {
                let left = 500-(width/2);
                for x in left..(500+(width/2)) {
                    if original_map.contains(&(x as usize, y as usize)) {
                        state[(y*width + x - left) as usize] = 2;
                    } else if map.contains(&(x as usize, y as usize)) {
                        state[(y*width + x - left) as usize] = 1;
                    }
                }
            }
            frame.buffer = Cow::Borrowed(&state);
            encoder.write_frame(&frame).unwrap();
        } */

        let mut sand = (500, 0);
        let mut total_sand = 0;
        loop {
            if sand.1 == bottom + 1 {
                // w_frame(&mut encoder, &map, &original_map);
                map.insert(sand);
                total_sand += 1;
                sand = (500, 0);
            } else if !map.contains(&(sand.0, sand.1 + 1)) {
                sand.1 += 1;
            } else if !map.contains(&(sand.0 - 1, sand.1 + 1)) {
                sand = (sand.0 - 1, sand.1 + 1);
            } else if !map.contains(&(sand.0 + 1, sand.1 + 1)) {
                sand = (sand.0 + 1, sand.1 + 1);
            } else if map.contains(&sand) {
                break;
            } else {
                // w_frame(&mut encoder, &map, &original_map);
                map.insert(sand);
                total_sand += 1;
                sand = (500, 0);
            }
        }
        //print_map(&map, 400, 520, 0, bottom + 2);

        total_sand
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
    fn parsed() -> <Day14 as Day>::Parsed {
        Day14::parse(INPUT.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day14::first(parsed()), 24);
    }
    #[test]
    fn part2() {
        assert_eq!(Day14::second(parsed()), 93);
    }
}
