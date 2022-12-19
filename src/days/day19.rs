use super::day::Day;
use ahash::{HashSet, HashSetExt};
use anyhow::Result;
use rayon::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Factory {
    id: u32,
    ore: Money,
    clay: Money,
    obsidian: Money,
    geode: Money,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Money {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}
impl Money {
    fn affords(&self, other: Self) -> bool {
        self.ore >= other.ore
            && self.clay >= other.clay
            && self.obsidian >= other.obsidian
            && self.geode >= other.geode
    }
}
impl std::ops::Add<Money> for Money {
    type Output = Money;
    fn add(self, other: Self) -> Money {
        Money {
            ore: self.ore + other.ore,
            clay: self.clay + other.clay,
            obsidian: self.obsidian + other.obsidian,
            geode: self.geode + other.geode,
        }
    }
}
impl std::ops::Sub<Money> for Money {
    type Output = Money;
    fn sub(self, other: Self) -> Money {
        Money {
            ore: self.ore - other.ore,
            clay: self.clay - other.clay,
            obsidian: self.obsidian - other.obsidian,
            geode: self.geode - other.geode,
        }
    }
}
impl std::ops::Mul<u32> for Money {
    type Output = Money;
    fn mul(self, other: u32) -> Money {
        Money {
            ore: self.ore * other,
            clay: self.clay * other,
            obsidian: self.obsidian * other,
            geode: self.geode * other,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Robots {
    ore: u8,
    clay: u8,
    obsidian: u8,
    geode: u8,
}
impl Robots {
    fn add_ore(self) -> Self {
        let mut new = self;
        new.ore += 1;
        new
    }
    fn add_clay(self) -> Self {
        let mut new = self;
        new.clay += 1;
        new
    }
    fn add_obsidian(self) -> Self {
        let mut new = self;
        new.obsidian += 1;
        new
    }
    fn add_geode(self) -> Self {
        let mut new = self;
        new.geode += 1;
        new
    }
    fn mine(self) -> Money {
        Money {
            ore: self.ore as u32,
            clay: self.clay as u32,
            obsidian: self.obsidian as u32,
            geode: self.geode as u32,
        }
    }
}

fn maxvals(time: u32, bots: u32) -> u32 {
    if time <= 1 {
        return bots * time;
    }
    let right = 2 * bots + (time - 1);
    if time % 2 == 0 {
        time / 2 * right
    } else {
        time / 2 * right + right / 2
    }
}

#[allow(unused)]
fn possible_max_vals(factory: Factory, time: u32, mut bots: Robots, money: Money) -> u32 {
    let mut ore_money = [money.ore; 4];
    let mut clay_money = money.clay;
    let mut obs_money = money.obsidian;

    let mut geodes = money.geode;
    for i in (0..time).rev() {
        ore_money.iter_mut().for_each(|m| *m += bots.ore as u32);
        clay_money += bots.clay as u32;
        obs_money += bots.obsidian as u32;
        geodes += bots.geode as u32;

        if ore_money[0] >= factory.ore.ore {
            ore_money[0] -= factory.ore.ore;
            bots.ore += 1;
        }

        if ore_money[1] >= factory.clay.ore {
            ore_money[1] -= factory.clay.ore;
            bots.clay += 1;
        }

        if ore_money[2] >= factory.obsidian.ore && clay_money >= factory.obsidian.clay {
            ore_money[2] -= factory.obsidian.ore;
            clay_money -= factory.obsidian.clay;
            bots.obsidian += 1;
        }

        if ore_money[3] >= factory.geode.ore && obs_money >= factory.geode.clay {
            ore_money[3] -= factory.geode.ore;
            obs_money -= factory.geode.obsidian;
            geodes += i;
        }
    }
    geodes
}

fn factory_loop(factory: Factory, max_time: u8) -> u32 {
    let buy_max_ore = factory.ore.ore.max(
        factory
            .clay
            .ore
            .max(factory.obsidian.ore.max(factory.geode.ore)),
    ) as u8;
    let buy_max_clay = factory.obsidian.clay as u8;
    let buy_max_obsidian = factory.geode.obsidian as u8;

    let mut max = 0;
    let mut stack = Vec::new();
    stack.push((
        max_time,
        Robots {
            ore: 1,
            clay: 0,
            obsidian: 0,
            geode: 0,
        },
        Money::default(),
    ));

    let mut cache = HashSet::new();

    while let Some((time, robots, money)) = stack.pop() {
        let after_money = money + robots.mine();
        if time == 1 {
            if after_money.geode > max {
                /* println!(
                    "  Factory {} found new max geode: {}",
                    factory.id, after_money.geode
                );
                println!("    robots: {robots:?}, money: {after_money:?}"); */
                max = after_money.geode;
            }
            continue;
        }

        if (maxvals(time as u32, robots.geode as u32) + money.geode) < max {
            continue;
        }

        if cache.contains(&(time, robots, money)) {
            continue;
        }
        cache.insert((time, robots, money));

        /*if possible_max_vals(factory, time as u32, robots, money) < max {
            continue;
        }*/
        stack.push((time - 1, robots, after_money));
        if robots.clay < buy_max_clay && money.affords(factory.clay) {
            stack.push((time - 1, robots.add_clay(), after_money - factory.clay));
        }
        if robots.obsidian < buy_max_obsidian && money.affords(factory.obsidian) {
            stack.push((
                time - 1,
                robots.add_obsidian(),
                after_money - factory.obsidian,
            ));
        }
        if robots.ore < buy_max_ore && money.affords(factory.ore) {
            stack.push((time - 1, robots.add_ore(), after_money - factory.ore));
        }
        if money.affords(factory.geode) {
            stack.push((time - 1, robots.add_geode(), after_money - factory.geode));
        }
    }
    max
}

pub struct Day19;
impl Day for Day19 {
    type Parsed = Vec<Factory>;
    type Output = u32;

    fn parse(input: String) -> Result<Self::Parsed> {
        Ok(input
            .lines()
            .map(|line| {
                let mut vals = line.split([' ', ':']).filter_map(|i| i.parse::<u32>().ok());
                Factory {
                    id: vals.next().unwrap(),
                    ore: Money {
                        ore: vals.next().unwrap(),
                        ..Default::default()
                    },
                    clay: Money {
                        ore: vals.next().unwrap(),
                        ..Default::default()
                    },
                    obsidian: Money {
                        ore: vals.next().unwrap(),
                        clay: vals.next().unwrap(),
                        ..Default::default()
                    },
                    geode: Money {
                        ore: vals.next().unwrap(),
                        obsidian: vals.next().unwrap(),
                        ..Default::default()
                    },
                }
            })
            .collect())
    }
    fn first(factories: Self::Parsed) -> Self::Output {
        factories
            .par_iter()
            .map(|factory| {
                let max = factory_loop(*factory, 24);
                //println!("Factory {} has max: {max}", factory.id);
                max * factory.id
            })
            .sum()
    }
    fn second(factories: Self::Parsed) -> Self::Output {
        factories
            .par_iter()
            .take(3)
            .map(|factory| {
                let max = factory_loop(*factory, 32);
                //println!("Factory {} has max: {max}", factory.id);
                max
            })
            .product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Blueprint 1:
  Each ore robot costs 4 ore.
  Each clay robot costs 2 ore.
  Each obsidian robot costs 3 ore and 14 clay.
  Each geode robot costs 2 ore and 7 obsidian.

Blueprint 2:
  Each ore robot costs 2 ore.
  Each clay robot costs 3 ore.
  Each obsidian robot costs 3 ore and 8 clay.
  Each geode robot costs 3 ore and 12 obsidian.";
    fn parsed() -> <Day19 as Day>::Parsed {
        Day19::parse(INPUT.replace("\n  ", " ").replace("\n\n", "\n").to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day19::first(parsed()), 33);
    }
    #[test]
    fn part2() {
        assert_eq!(Day19::second(parsed()), 3472);
    }
}
