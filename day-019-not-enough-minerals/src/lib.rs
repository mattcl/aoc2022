use std::{collections::BinaryHeap, hash::Hash, str::FromStr};

use aoc_plumbing::Problem;
use nom::{
    bytes::complete::tag,
    character::complete::{newline, space0},
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, tuple},
    IResult,
};
use rayon::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Mineral {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Robot {
    mineral: Mineral,
    costs: [i64; 4],
}

impl Robot {
    pub fn new(mineral: Mineral, costs: [i64; 4]) -> Self {
        Self { mineral, costs }
    }
}

fn parse_ore(input: &str) -> IResult<&str, Robot> {
    let (input, ore) = delimited(
        tag("Each ore robot costs "),
        nom::character::complete::i64,
        tag(" ore."),
    )(input)?;

    Ok((input, Robot::new(Mineral::Ore, [ore, 0, 0, 0])))
}

fn parse_clay(input: &str) -> IResult<&str, Robot> {
    let (input, ore) = delimited(
        tag("Each clay robot costs "),
        nom::character::complete::i64,
        tag(" ore."),
    )(input)?;

    Ok((input, Robot::new(Mineral::Clay, [ore, 0, 0, 0])))
}

fn parse_obsidian(input: &str) -> IResult<&str, Robot> {
    let (input, (ore, clay)) = delimited(
        tag("Each obsidian robot costs "),
        separated_pair(
            nom::character::complete::i64,
            tag(" ore and "),
            nom::character::complete::i64,
        ),
        tag(" clay."),
    )(input)?;

    Ok((input, Robot::new(Mineral::Obsidian, [ore, clay, 0, 0])))
}

fn parse_geode(input: &str) -> IResult<&str, Robot> {
    let (input, (ore, obsidian)) = delimited(
        tag("Each geode robot costs "),
        separated_pair(
            nom::character::complete::i64,
            tag(" ore and "),
            nom::character::complete::i64,
        ),
        tag(" obsidian."),
    )(input)?;

    Ok((input, Robot::new(Mineral::Geode, [ore, 0, obsidian, 0])))
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct OldState {
    time_remaining: i64,
    population: [i64; 4],
    inventory: [i64; 4],
}

impl Default for OldState {
    fn default() -> Self {
        Self {
            time_remaining: 0,
            population: [1, 0, 0, 0],
            inventory: [0; 4],
        }
    }
}

impl OldState {
    pub fn geodes_with_remaining_time(&self) -> i64 {
        self.inventory[3] + self.population[3] * self.time_remaining
    }

    /// the most geodes we could make from this point on, assuming we could also
    /// make a new geode robot every minute
    pub fn geode_limit(&self) -> i64 {
        self.mineral_limit(3)
    }

    /// the most obsidian we could make from this point on, assuming we could also
    /// make a new obsidian robot every minute
    pub fn obsidian_limit(&self) -> i64 {
        self.mineral_limit(2)
    }

    fn mineral_limit(&self, idx: usize) -> i64 {
        self.inventory[idx]
            + self.population[idx] * self.time_remaining
            + (self.time_remaining * (self.time_remaining - 1)) / 2
    }

    /// Return a placeholder next state with minutes remaining decremented and
    /// inventory increased with the current population
    pub fn next(&self) -> Self {
        let mut next = *self;
        next.time_remaining -= 1;
        for i in 0..4 {
            next.inventory[i] += next.population[i];
        }
        next
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct State {
    theoretical_best: i64,
    minutes_remaining: i64,
    inventory: [i64; 4],
    population: [i64; 4],
}

impl Default for State {
    fn default() -> Self {
        Self {
            theoretical_best: 0,
            minutes_remaining: 0,
            inventory: [0; 4],
            population: [1, 0, 0, 0],
        }
    }
}

impl State {
    pub fn best(&self) -> i64 {
        self.inventory[3] + self.population[3] * self.minutes_remaining
    }

    pub fn time_until_next(&self, robot: usize, blueprint: &Blueprint) -> i64 {
        (0..3)
            .map(|i| {
                if blueprint.robots[robot].costs[i] <= self.inventory[i] {
                    0
                } else if self.population[i] == 0 {
                    i64::MAX
                } else {
                    1 + (blueprint.robots[robot].costs[i] - self.inventory[i] - 1)
                        / self.population[i]
                }
            })
            .max()
            .unwrap()
    }

    pub fn next(&self, wait: i64, robot: usize, blueprint: &Blueprint) -> Self {
        let mut n = *self;
        for i in 0..4 {
            n.inventory[i] =
                n.inventory[i] + self.population[i] * (wait + 1) - blueprint.robots[robot].costs[i];
            if self.population[i] >= blueprint.limits[i] {
                n.inventory[i] = blueprint.limits[i];
            }
        }
        n.minutes_remaining -= wait + 1;
        n.population[robot] += 1;

        // pretend like we live in a world where we have seprate inventories
        // that we can use to buy each of the robot types. The most geode robots
        // we can produce in this world is the theoretical best we can do.
        n.theoretical_best = {
            // make copy of our current inventory for reach of the robots
            let mut inventories = [n.inventory; 4];

            // make a copy of the current robot inventory
            let mut population = n.population;

            // for the rest of the time we have left
            for _ in 0..n.minutes_remaining {
                let mut new_inventories = inventories;

                // for each of the inventory copies
                for i in 0..4 {
                    // adjust the mineral inventory based on the current
                    // theoretical best for each robot type
                    for mineral in 0..4 {
                        new_inventories[i][mineral] += population[mineral];
                    }
                }

                // for each of the inventory copies
                for i in 0..4 {
                    // if we can afford the robot this inventory copy correponds
                    // to, buy it and increment our theoretical best population
                    // of robots.
                    if (0..3).all(|mineral| {
                        inventories[i][mineral] >= blueprint.robots[i].costs[mineral]
                    }) {
                        (0..3).for_each(|mineral| {
                            new_inventories[i][mineral] -= blueprint.robots[i].costs[mineral]
                        });
                        population[i] += 1;
                    }
                }
                inventories = new_inventories;
            }

            // we could pick any of the inventories, but just pick 0. The value
            // here will be the theoretical best number of geodes we could have
            // produced
            inventories[0][3]
        };
        n
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // we want to sort the heap such that the largest theoretical bests
        // end up at the top of the heap. If there's a tie, use the minutes
        // remaining to break the tie, with _lower_ minutes remaining at the
        // top of the heap
        self.theoretical_best
            .cmp(&other.theoretical_best)
            .then_with(|| other.minutes_remaining.cmp(&self.minutes_remaining))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Blueprint {
    id: i64,
    robots: [Robot; 4],
    limits: [i64; 4],
}

impl Blueprint {
    pub fn most_geodes_in_time(&self, minutes: i64) -> i64 {
        let mut heap = BinaryHeap::new();

        heap.push(State {
            minutes_remaining: minutes,
            ..Default::default()
        });

        let mut best = i64::MIN;

        while let Some(state) = heap.pop() {
            if state.theoretical_best <= best {
                continue;
            }

            // this is the actual best we can do with this state if we didn't
            // build any more robots
            best = best.max(state.best());

            // simulate buying each kind of robot. We don't need to simulate
            // waiting because we force the purchase of the next robot
            for i in 0..4 {
                if state.population[i] == self.limits[i] {
                    continue;
                }

                // figure out how long to wait to build a robot of this type
                let wait = state.time_until_next(i, &self);

                // if we'd need to wait longer than the time we have left + 1,
                // skip this
                if wait == i64::MAX || wait + 1 >= state.minutes_remaining {
                    continue;
                }

                let next_state = state.next(wait, i, &self);

                if next_state.theoretical_best > best {
                    heap.push(next_state);
                }
            }
        }

        best
    }
}

fn parse_blueprint(input: &str) -> IResult<&str, Blueprint> {
    let (input, (id, ore, clay, obsidian, geode)) = tuple((
        delimited(
            tag("Blueprint "),
            nom::character::complete::i64,
            nom::character::complete::char(':'),
        ),
        preceded(space0, parse_ore),
        preceded(space0, parse_clay),
        preceded(space0, parse_obsidian),
        preceded(space0, parse_geode),
    ))(input)?;

    let robots = [ore, clay, obsidian, geode];
    let mut limits = [i64::MAX; 4];

    for robot in robots.iter() {
        for i in 0..3 {
            if robot.costs[i] > limits[i] {
                limits[i] = robot.costs[i];
            }
        }
    }

    Ok((input, Blueprint { id, robots, limits }))
}

fn parse_blueprints(input: &str) -> IResult<&str, Vec<Blueprint>> {
    separated_list1(newline, parse_blueprint)(input)
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NotEnoughMinerals {
    blueprints: Vec<Blueprint>,
}

impl FromStr for NotEnoughMinerals {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, blueprints) = parse_blueprints(s.trim()).map_err(|e| e.to_owned())?;
        Ok(Self { blueprints })
    }
}

impl Problem for NotEnoughMinerals {
    const DAY: usize = 19;
    const TITLE: &'static str = "not enough minerals";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = i64;
    type P2 = i64;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self
            .blueprints
            .par_iter()
            .map(|b| b.most_geodes_in_time(24) * b.id)
            .sum())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.blueprints[0..(3.min(self.blueprints.len()))]
            .par_iter()
            .map(|b| b.most_geodes_in_time(32))
            .product())
    }
}

#[cfg(test)]
mod tests {
    use aoc_plumbing::Solution;

    use super::*;

    #[test]
    #[ignore]
    fn full_dataset() {
        let input = std::fs::read_to_string("input.txt").expect("Unable to load input");
        let solution = NotEnoughMinerals::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(1624, 12628));
    }

    #[test]
    fn example() {
        let input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
        let solution = NotEnoughMinerals::solve(input).unwrap();
        assert_eq!(solution, Solution::new(33, 3472));
    }
}
