use std::collections::HashMap;

use super::{
    day::Day,
    helpers::{BitArr, UsizeIter},
};
use ahash::AHashMap;
use anyhow::Result;
use itertools::iproduct;
use smallvec::SmallVec;

pub struct RoomSolver64 {
    rooms: Vec<Room>,
    start: usize,
    weights: [u32; 64 * 64],
    flows: [u32; 64],
}
impl RoomSolver64 {
    pub fn new(rooms: Vec<Room>, start: usize) -> Self {
        let weights = Self::make_weights(&rooms);
        let flows = Self::make_flows(&rooms);
        Self {
            rooms,
            start,
            weights,
            flows,
        }
    }
    fn make_weights(rooms: &[Room]) -> [u32; 64 * 64] {
        let mut weights = [9999; 64 * 64];
        for (i, room) in rooms.iter().enumerate() {
            for &tunnel in &room.tunnels {
                weights[i * 64 + tunnel] = 1;
            }
        }
        for (k, i, j) in iproduct!(0..rooms.len(), 0..rooms.len(), 0..rooms.len()) {
            let y = weights[i * 64 + k];
            let z = weights[k * 64 + j];
            weights[i * 64 + j] = weights[i * 64 + j].min(y + z);
        }
        weights
    }
    fn make_flows(rooms: &[Room]) -> [u32; 64] {
        let mut flows = [0; 64];
        for (i, room) in rooms.iter().enumerate() {
            flows[i] = room.flow;
        }
        flows
    }
    fn make_to_search(&self) -> usize {
        let mut to_search = 0;
        to_search.set(0);
        for (i, room) in self.rooms.iter().enumerate() {
            if room.flow > 0 {
                to_search.set(i);
            }
        }
        to_search
    }
    fn solve_first(&mut self) -> u32 {
        self.alone(30, self.start, self.make_to_search())
    }
    fn alone(&mut self, time: u32, key: usize, to_search: usize) -> u32 {
        if time == 0 {
            return 0;
        }
        let mut max = 0;
        for curr in UsizeIter::new(to_search) {
            let walk_time = unsafe { *self.weights.get_unchecked(key * 64 + curr) };
            if walk_time < time {
                let mut ts = to_search;
                ts.clear(curr);
                max = max.max(
                    self.flows[curr] * (time - walk_time - 1)
                        + self.alone(time - walk_time - 1, curr, ts),
                );
            }
        }
        max
    }
    fn solve_second(&mut self) -> u32 {
        let to_search = self.make_to_search();
        self.together(&mut AHashMap::new(), 26, self.start, to_search, true)
    }
    fn together(
        &mut self,
        memo: &mut AHashMap<(u32, usize, usize, bool), u32>,
        time: u32,
        key: usize,
        to_search: usize,
        helper: bool,
    ) -> u32 {
        if time == 0 {
            return 0;
        }
        if let Some(m) = memo.get(&(time, key, to_search, helper)) {
            return *m;
        }
        let mut max = 0;
        for curr in UsizeIter::new(to_search) {
            let walk_time = self.weights[key * 64 + curr];
            if walk_time < time {
                let mut ts = to_search;
                ts.clear(curr);
                let time_left = time - walk_time - 1;
                let deeper = self.together(memo, time_left, curr, ts, helper);
                max = max.max(deeper + self.flows[curr] * time_left);
            }
        }
        if helper {
            max = max.max(self.together(memo, 26, self.start, to_search, false));
        }
        memo.insert((time, key, to_search, helper), max);
        max
    }
}

#[derive(Debug, Clone)]
pub struct Room {
    flow: u32,
    tunnels: SmallVec<[usize; 8]>,
}
pub struct TempRoom<'a> {
    flow: u32,
    tunnels: SmallVec<[&'a str; 8]>,
}

pub struct Day16;
impl Day for Day16 {
    type Parsed = (Vec<Room>, usize);
    type Output = u32;

    fn parse(input: String) -> Result<Self::Parsed> {
        let mut names_to_i = HashMap::new();

        let temp_rooms: Vec<TempRoom> = input
            .lines()
            .enumerate()
            .map(|(i, line)| {
                let (flow, rest) = line[23..].split_once(';').unwrap();
                names_to_i.insert(&line[6..8], i);

                TempRoom {
                    flow: flow.parse::<u32>().unwrap(),
                    tunnels: rest[23..].trim_start().split(", ").collect(),
                }
            })
            .collect();

        Ok((
            temp_rooms
                .iter()
                .map(|r| Room {
                    flow: r.flow,
                    tunnels: r.tunnels.iter().map(|t| names_to_i[t]).collect(),
                })
                .collect(),
            *names_to_i.get(&"AA").unwrap(),
        ))
    }
    fn first((rooms, start): Self::Parsed) -> Self::Output {
        assert!(rooms.len() <= 64, "Input data too long");
        let mut solver = RoomSolver64::new(rooms, start);
        solver.solve_first()
    }
    fn second((rooms, start): Self::Parsed) -> Self::Output {
        assert!(rooms.len() <= 64, "Input data too long");
        let mut solver = RoomSolver64::new(rooms, start);
        solver.solve_second()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
    fn parsed() -> <Day16 as Day>::Parsed {
        Day16::parse(INPUT.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day16::first(parsed()), 1651);
    }
    #[test]
    fn part2() {
        assert_eq!(Day16::second(parsed()), 1707);
    }
}
