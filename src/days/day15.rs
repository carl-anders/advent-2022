#![allow(clippy::cast_possible_wrap, clippy::range_plus_one)]
use super::day::Day;
use crate::helpers::{MergedRange, RangeIntersect};
use anyhow::Result;
use itertools::Itertools;
use smallvec::{smallvec, SmallVec};
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
pub struct SensorData {
    pos: (i32, i32),
    beacon: (i32, i32),
    manhattan: i32,
    diagonal: i32,
}
impl SensorData {
    const fn new(pos: (i32, i32), beacon: (i32, i32)) -> Self {
        let x_diff = pos.0.abs_diff(beacon.0) as i32;
        let y_diff = pos.1.abs_diff(beacon.1) as i32;
        Self {
            pos,
            beacon,
            manhattan: x_diff + y_diff,
            diagonal: pos.0 + pos.1,
        }
    }
    const fn max_influences_y(&self, y: i32) -> bool {
        self.pos.1 + self.manhattan >= y && y >= self.pos.1 - self.manhattan
    }
}

#[cfg(test)]
const Y_TEST: i32 = 10;
#[cfg(not(test))]
const Y_TEST: i32 = 2_000_000;

#[cfg(test)]
const MAX_TEST: i32 = 20;
#[cfg(not(test))]
const MAX_TEST: i32 = 4_000_000;

pub struct Day15;
impl Day for Day15 {
    type Parsed = Vec<SensorData>;
    type Output = usize;

    fn parse(input: String) -> Result<Self::Parsed> {
        Ok(input
            .lines()
            .map(|line| {
                let nums: Vec<i32> = line
                    .split(&['=', ',', ':'])
                    .filter_map(|s| s.parse().ok())
                    .collect();
                SensorData::new((nums[0], nums[1]), (nums[2], nums[3]))
            })
            .collect())
    }
    fn first(sensors: Self::Parsed) -> Self::Output {
        let beacons_on_y: Vec<_> = sensors
            .iter()
            .filter(|sensor| sensor.beacon.1 == Y_TEST)
            .map(|sensor| sensor.beacon.0)
            .unique()
            .collect();

        let merged_ranges: MergedRange<_> = sensors
            .iter()
            .filter_map(|sensor| {
                if sensor.max_influences_y(Y_TEST) {
                    let distance = sensor.manhattan - (sensor.pos.1 - Y_TEST).abs();
                    Some((sensor.pos.0 - distance)..(sensor.pos.0 + distance + 1))
                } else {
                    None
                }
            })
            .collect();

        merged_ranges
            .ranges()
            .into_iter()
            .map(|r| {
                let beacons_in_range = beacons_on_y
                    .iter()
                    .filter(|beacon_x| r.contains(beacon_x))
                    .count();
                (r.end - r.start) as usize - beacons_in_range
            })
            .sum()
    }
    fn second(sensors: Self::Parsed) -> Self::Output {
        sensors
            .iter()
            .fold(VecDeque::from([0, MAX_TEST]), |mut acc, sensor| {
                acc.push_back(sensor.diagonal - sensor.manhattan - 1);
                acc.push_back(sensor.diagonal - sensor.manhattan - 2);
                acc.push_back(sensor.diagonal + sensor.manhattan + 1);
                acc.push_back(sensor.diagonal + sensor.manhattan + 2);
                acc
            })
            .into_iter()
            .filter_map(|diag| {
                if diag >= 0 && diag <= MAX_TEST * 2 {
                    let max_range = diag.min(2 * MAX_TEST - diag);
                    Some((diag, smallvec![-max_range..max_range]))
                } else {
                    None
                }
            })
            .unique()
            .filter_map(|(diag, mut range): (i32, SmallVec<[_; 2]>)| {
                for sensor in &sensors {
                    if !(sensor.diagonal - sensor.manhattan > diag
                        || sensor.diagonal + sensor.manhattan < diag)
                    {
                        let diag_test = sensor.pos.0 - sensor.pos.1;
                        range = range
                            .into_iter()
                            .flat_map(|r| {
                                [
                                    (0..(diag_test - sensor.manhattan - 1)),
                                    ((diag_test + sensor.manhattan + 1)..MAX_TEST),
                                ]
                                .iter()
                                .filter_map(|test_range| r.intersect(test_range))
                                .collect::<SmallVec<[_; 2]>>()
                            })
                            .collect();
                    }
                }
                if range.is_empty() {
                    None
                } else {
                    Some((diag, range[0].start))
                }
            })
            .map(|(diag, diff)| {
                let x = ((diag + diff) / 2) as usize;
                let y = ((diag - diff) / 2) as usize;
                4_000_000 * x + y
            })
            .next()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
    fn parsed() -> <Day15 as Day>::Parsed {
        Day15::parse(INPUT.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day15::first(parsed()), 26);
    }
    #[test]
    fn part2() {
        assert_eq!(Day15::second(parsed()), 56000011);
    }
}
