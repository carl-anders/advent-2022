use super::day::Day;
use ahash::HashSet;
use anyhow::Result;
use itertools::Itertools;
use pathfinding::prelude::dijkstra;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    x: i8,
    y: i8,
    z: i8,
}

pub struct Day18;
impl Day for Day18 {
    type Parsed = HashSet<Point>;
    type Output = usize;

    fn parse(input: String) -> Result<Self::Parsed> {
        let test = input
            .lines()
            .map(|line| {
                let (x, y, z) = line
                    .split(',')
                    .map(|i| i.parse().unwrap())
                    .next_tuple()
                    .unwrap();
                Point { x, y, z }
            })
            .collect();
        Ok(test)
    }
    fn first(points: Self::Parsed) -> Self::Output {
        points
            .iter()
            .map(|p| {
                [
                    [0, 0, 1],
                    [0, 0, -1],
                    [0, 1, 0],
                    [0, -1, 0],
                    [1, 0, 0],
                    [-1, 0, 0],
                ]
                .iter()
                .map(|d| {
                    usize::from(!points.contains(&Point {
                        x: p.x + d[0],
                        y: p.y + d[1],
                        z: p.z + d[2],
                    }))
                })
                .sum::<usize>()
            })
            .sum()
    }
    fn second(points: Self::Parsed) -> Self::Output {
        let max_x = points.iter().map(|p| p.x).max().unwrap();
        let max_y = points.iter().map(|p| p.y).max().unwrap();
        let max_z = points.iter().map(|p| p.z).max().unwrap();

        let trapped_air: HashSet<Point> = (0..max_x)
            .into_par_iter()
            .flat_map(|x| {
                let mut trapped_air: Vec<Point> = Vec::new();
                for y in 0..max_y {
                    for z in 0..max_z {
                        let self_point = Point { x, y, z };
                        if !points.contains(&self_point) {
                            let result = dijkstra(
                                &self_point,
                                |p| {
                                    let v: Vec<(Point, u8)> = [
                                        [0, 0, 1],
                                        [0, 0, -1],
                                        [0, 1, 0],
                                        [0, -1, 0],
                                        [1, 0, 0],
                                        [-1, 0, 0],
                                    ]
                                    .iter()
                                    .filter_map(|d| {
                                        let point = Point {
                                            x: p.x + d[0],
                                            y: p.y + d[1],
                                            z: p.z + d[2],
                                        };
                                        if points.contains(&point) {
                                            None
                                        } else {
                                            Some((point, 1))
                                        }
                                    })
                                    .collect();
                                    v
                                },
                                |p| {
                                    p.x > max_x
                                        || p.x < 0
                                        || p.y > max_y
                                        || p.y < 0
                                        || p.z > max_z
                                        || p.z < 0
                                },
                            );
                            if result.is_none() {
                                trapped_air.push(self_point);
                            }
                        }
                    }
                }
                trapped_air
            })
            .collect();

        let first_area = Self::first(points);
        let trapped_area = Self::first(trapped_air);
        first_area - trapped_area
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";
    fn parsed() -> <Day18 as Day>::Parsed {
        Day18::parse(INPUT.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day18::first(parsed()), 64);
    }
    #[test]
    fn part2() {
        assert_eq!(Day18::second(parsed()), 58);
    }
}
