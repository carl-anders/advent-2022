use super::{day::Day, helpers::LongBitArr};
use ahash::AHashSet;
use anyhow::Result;
use itertools::{Itertools, MinMaxResult};

pub trait SandSolver: CloneSandSolver {
    fn first(&mut self) -> usize;
    fn second(&mut self) -> usize;
}
pub trait CloneSandSolver {
    fn clone_sand_solver(&self) -> Box<dyn SandSolver>;
}
impl<T> CloneSandSolver for T
where
    T: SandSolver + Clone + 'static,
{
    fn clone_sand_solver(&self) -> Box<dyn SandSolver> {
        Box::new(self.clone())
    }
}
impl Clone for Box<dyn SandSolver> {
    fn clone(&self) -> Self {
        self.clone_sand_solver()
    }
}
#[derive(Debug, Clone)]
pub struct ArraySandSolver<const XC: usize, const YC: usize> {
    map: [LongBitArr<XC>; YC],
    bottom: usize,
    sand_start: (usize, usize),
}
impl<const XC: usize, const YC: usize> ArraySandSolver<XC, YC> {
    fn parse(paths: &Vec<Vec<(usize, usize)>>, borders: ((usize, usize), (usize, usize))) -> Self {
        let left_remove = borders.0 .0 - borders.1 .1 - 2;

        let mut map = [LongBitArr::<XC>::new(); YC];
        for path in paths {
            for (a, b) in path.iter().tuple_windows() {
                (a.0.min(b.0)..=a.0.max(b.0))
                    .zip_longest(a.1.min(b.1)..=a.1.max(b.1))
                    .for_each(|xy| {
                        let xy = xy.or(a.0, a.1);
                        if !map[xy.1].get(xy.0 - left_remove) {
                            map[xy.1].set(xy.0 - left_remove);
                        }
                    });
            }
        }
        Self {
            map,
            bottom: borders.1 .1,
            sand_start: (500 - left_remove, 0),
        }
    }
    fn second_deep(map: &mut [LongBitArr<XC>; YC], sand: (usize, usize), bottom: usize) {
        if sand.1 > bottom + 1 {
            return;
        }
        map[sand.1].set(sand.0);
        if !map[sand.1 + 1].get(sand.0) {
            Self::second_deep(map, (sand.0, sand.1 + 1), bottom);
        }
        if !map[sand.1 + 1].get(sand.0 - 1) {
            Self::second_deep(map, (sand.0 - 1, sand.1 + 1), bottom);
        }
        if !map[sand.1 + 1].get(sand.0 + 1) {
            Self::second_deep(map, (sand.0 + 1, sand.1 + 1), bottom);
        }
    }
}
impl<const XC: usize, const YC: usize> SandSolver for ArraySandSolver<XC, YC> {
    fn first(&mut self) -> usize {
        let mut sand = self.sand_start;
        let mut total_sand = 0;
        loop {
            if sand.1 >= self.bottom {
                break;
            } else if !self.map[sand.1 + 1].get(sand.0) {
                sand.1 += 1;
            } else if !self.map[sand.1 + 1].get(sand.0 - 1) {
                sand = (sand.0 - 1, sand.1 + 1);
            } else if !self.map[sand.1 + 1].get(sand.0 + 1) {
                sand = (sand.0 + 1, sand.1 + 1);
            } else {
                self.map[sand.1].set(sand.0);
                total_sand += 1;
                sand = self.sand_start;
            }
        }
        total_sand
    }
    fn second(&mut self) -> usize {
        let init_blocks = self.map.iter().map(LongBitArr::count_ones).sum::<usize>();
        Self::second_deep(&mut self.map, self.sand_start, self.bottom);
        self.map.iter().map(LongBitArr::count_ones).sum::<usize>() - init_blocks
    }
}
#[derive(Debug, Clone)]
pub struct SetSandSolver {
    map: AHashSet<(usize, usize)>,
    bottom: usize,
}
impl SetSandSolver {
    fn parse(paths: &Vec<Vec<(usize, usize)>>, borders: ((usize, usize), (usize, usize))) -> Self {
        let mut map = AHashSet::default();
        for path in paths {
            for (a, b) in path.iter().tuple_windows() {
                (a.0.min(b.0)..=a.0.max(b.0))
                    .zip_longest(a.1.min(b.1)..=a.1.max(b.1))
                    .for_each(|xy| {
                        let xy = xy.or(a.0, a.1);
                        map.insert(xy);
                    });
            }
        }
        Self {
            map,
            bottom: borders.1 .1,
        }
    }
}
impl SandSolver for SetSandSolver {
    fn first(&mut self) -> usize {
        let mut sand = (500, 0);
        let mut total_sand = 0;
        loop {
            if sand.1 >= self.bottom {
                break;
            } else if !self.map.contains(&(sand.0, sand.1 + 1)) {
                sand.1 += 1;
            } else if !self.map.contains(&(sand.0 - 1, sand.1 + 1)) {
                sand = (sand.0 - 1, sand.1 + 1);
            } else if !self.map.contains(&(sand.0 + 1, sand.1 + 1)) {
                sand = (sand.0 + 1, sand.1 + 1);
            } else {
                self.map.insert(sand);
                total_sand += 1;
                sand = (500, 0);
            }
        }
        total_sand
    }
    fn second(&mut self) -> usize {
        fn deep(map: &mut AHashSet<(usize, usize)>, sand: (usize, usize), bottom: usize) {
            if sand.1 > bottom + 1 {
                return;
            }
            map.insert(sand);
            if !map.contains(&(sand.0, sand.1 + 1)) {
                deep(map, (sand.0, sand.1 + 1), bottom);
            }
            if !map.contains(&(sand.0 - 1, sand.1 + 1)) {
                deep(map, (sand.0 - 1, sand.1 + 1), bottom);
            }
            if !map.contains(&(sand.0 + 1, sand.1 + 1)) {
                deep(map, (sand.0 + 1, sand.1 + 1), bottom);
            }
        }
        let before = self.map.len();
        deep(&mut self.map, (500, 0), self.bottom);
        self.map.len() - before
    }
}

pub struct Day14;
impl Day for Day14 {
    type Parsed = Box<dyn SandSolver>;
    type Output = usize;

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

        let xs = paths.iter().flat_map(|path| path.iter().map(|p| p.0));
        let ys = paths.iter().flat_map(|path| path.iter().map(|p| p.1));
        let borders = match (xs.minmax(), ys.minmax()) {
            (MinMaxResult::MinMax(x1, x2), MinMaxResult::MinMax(y1, y2)) => ((x1, x2), (y1, y2)),
            _ => panic!(),
        };

        let expanded_width = (borders.0 .1 - borders.0 .0) + borders.1 .1 * 2 + 6;
        let expanded_height = borders.1 .1 + 2;
        Ok(match (expanded_width, expanded_height) {
            (w, h) if w < 64 && h < 32 => {
                Box::new(ArraySandSolver::<1, 32>::parse(&paths, borders))
            }
            (w, h) if w < 128 && h < 64 => {
                Box::new(ArraySandSolver::<2, 64>::parse(&paths, borders))
            }
            (w, h) if w < 256 && h < 128 => {
                Box::new(ArraySandSolver::<4, 128>::parse(&paths, borders))
            }
            (w, h) if w < 512 && h < 256 => {
                Box::new(ArraySandSolver::<8, 256>::parse(&paths, borders))
            }
            _ => Box::new(SetSandSolver::parse(&paths, borders)),
        })
    }
    fn first(mut solver: Self::Parsed) -> Self::Output {
        solver.first()
    }
    fn second(mut solver: Self::Parsed) -> Self::Output {
        solver.second()
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
