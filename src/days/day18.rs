use super::day::Day;
use ahash::{HashSet, HashSetExt};
use anyhow::Result;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    x: i8,
    y: i8,
    z: i8,
}
impl Point {
    const fn new(pos: (i8, i8, i8)) -> Self {
        Self {
            x: pos.0,
            y: pos.1,
            z: pos.2,
        }
    }
    const fn add(self, other: (i8, i8, i8)) -> Self {
        Self {
            x: self.x + other.0,
            y: self.y + other.1,
            z: self.z + other.2,
        }
    }
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
                    (0, 0, 1),
                    (0, 0, -1),
                    (0, 1, 0),
                    (0, -1, 0),
                    (1, 0, 0),
                    (-1, 0, 0),
                ]
                .iter()
                .map(|&d| usize::from(!points.contains(&p.add(d))))
                .sum::<usize>()
            })
            .sum()
    }
    fn second(points: Self::Parsed) -> Self::Output {
        let mut xs = points.iter().map(|p| p.x).minmax().into_option().unwrap();
        let mut ys = points.iter().map(|p| p.y).minmax().into_option().unwrap();
        let mut zs = points.iter().map(|p| p.z).minmax().into_option().unwrap();

        xs = (xs.0 - 1, xs.1 + 1);
        ys = (ys.0 - 1, ys.1 + 1);
        zs = (zs.0 - 1, zs.1 + 1);

        let mut to_visit = vec![Point::new((xs.0, ys.0, zs.0))];
        let mut visited = HashSet::new();
        visited.insert(to_visit[0]);

        let mut touched_walls = 0;

        while let Some(visit) = to_visit.pop() {
            for offset in [
                (0, 0, 1),
                (0, 0, -1),
                (0, 1, 0),
                (0, -1, 0),
                (1, 0, 0),
                (-1, 0, 0),
            ] {
                let next_visit = visit.add(offset);
                if xs.0 <= next_visit.x
                    && next_visit.x <= xs.1
                    && ys.0 <= next_visit.y
                    && next_visit.y <= ys.1
                    && zs.0 <= next_visit.z
                    && next_visit.z <= zs.1
                {
                    if points.contains(&next_visit) {
                        touched_walls += 1;
                    } else if !visited.contains(&next_visit) {
                        to_visit.push(next_visit);
                        visited.insert(next_visit);
                    }
                }
            }
        }
        touched_walls
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
