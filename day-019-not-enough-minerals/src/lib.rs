use std::{hash::Hash, marker::PhantomData, str::FromStr};

use aoc_plumbing::Problem;
use nom::{
    bytes::complete::tag,
    character::complete::{newline, space0},
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, tuple},
    IResult,
};
use rayon::prelude::*;

pub trait Mineral {
    const BIT: u8;
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct Ore;

impl Mineral for Ore {
    const BIT: u8 = 0b1;
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct Clay;

impl Mineral for Clay {
    const BIT: u8 = 0b10;
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct Obsidian;

impl Mineral for Obsidian {
    const BIT: u8 = 0b100;
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct Geode;

impl Mineral for Geode {
    const BIT: u8 = 0b1000;
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct Costs {
    ore: i64,
    clay: i64,
    obsidian: i64,
    geode: i64,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct Robot<T> {
    costs: Costs,
    _marker: PhantomData<T>,
}

impl<T> Robot<T>
where
    T: Mineral,
{
    pub fn ore_cost(&self) -> i64 {
        self.costs.ore
    }

    pub fn costs(&self) -> &Costs {
        &self.costs
    }

    pub fn bit(&self) -> u8 {
        T::BIT
    }
}

impl Robot<Ore> {
    pub fn new(ore: i64) -> Self {
        Self {
            costs: Costs {
                ore,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn can_afford(&self, inventory: &[i64; 4]) -> bool {
        self.costs.ore <= inventory[0]
    }

    pub fn pay_for(&self, inventory: &mut [i64; 4]) {
        inventory[0] -= self.costs.ore;
    }
}

impl Robot<Clay> {
    pub fn new(ore: i64) -> Self {
        Self {
            costs: Costs {
                ore,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn can_afford(&self, inventory: &[i64; 4]) -> bool {
        self.costs.ore <= inventory[0]
    }

    pub fn pay_for(&self, inventory: &mut [i64; 4]) {
        inventory[0] -= self.costs.ore;
    }
}

impl Robot<Obsidian> {
    pub fn new(ore: i64, clay: i64) -> Self {
        Self {
            costs: Costs {
                ore,
                clay,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn can_afford(&self, inventory: &[i64; 4]) -> bool {
        self.costs.ore <= inventory[0] && self.costs.clay <= inventory[1]
    }

    pub fn pay_for(&self, inventory: &mut [i64; 4]) {
        inventory[0] -= self.costs.ore;
        inventory[1] -= self.costs.clay;
    }
}

impl Robot<Geode> {
    pub fn new(ore: i64, obsidian: i64) -> Self {
        Self {
            costs: Costs {
                ore,
                obsidian,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn can_afford(&self, inventory: &[i64; 4]) -> bool {
        self.costs.ore <= inventory[0] && self.costs.obsidian <= inventory[2]
    }

    pub fn pay_for(&self, inventory: &mut [i64; 4]) {
        inventory[0] -= self.costs.ore;
        inventory[2] -= self.costs.obsidian;
    }
}

pub type OreRobot = Robot<Ore>;
pub type ClayRobot = Robot<Clay>;
pub type ObsidianRobot = Robot<Obsidian>;
pub type GeodeRobot = Robot<Geode>;

fn parse_ore(input: &str) -> IResult<&str, OreRobot> {
    let (input, ore) = delimited(
        tag("Each ore robot costs "),
        nom::character::complete::i64,
        tag(" ore."),
    )(input)?;

    Ok((input, OreRobot::new(ore)))
}

fn parse_clay(input: &str) -> IResult<&str, ClayRobot> {
    let (input, ore) = delimited(
        tag("Each clay robot costs "),
        nom::character::complete::i64,
        tag(" ore."),
    )(input)?;

    Ok((input, ClayRobot::new(ore)))
}

fn parse_obsidian(input: &str) -> IResult<&str, ObsidianRobot> {
    let (input, (ore, clay)) = delimited(
        tag("Each obsidian robot costs "),
        separated_pair(
            nom::character::complete::i64,
            tag(" ore and "),
            nom::character::complete::i64,
        ),
        tag(" clay."),
    )(input)?;

    Ok((input, ObsidianRobot::new(ore, clay)))
}

fn parse_geode(input: &str) -> IResult<&str, GeodeRobot> {
    let (input, (ore, obsidian)) = delimited(
        tag("Each geode robot costs "),
        separated_pair(
            nom::character::complete::i64,
            tag(" ore and "),
            nom::character::complete::i64,
        ),
        tag(" obsidian."),
    )(input)?;

    Ok((input, GeodeRobot::new(ore, obsidian)))
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct State {
    time_remaining: i64,
    population: [i64; 4],
    inventory: [i64; 4],
    skipped: u8,
}

impl Default for State {
    fn default() -> Self {
        Self {
            time_remaining: 0,
            population: [1, 0, 0, 0],
            inventory: [0; 4],
            skipped: 0,
        }
    }
}

impl State {
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
        next.skipped = 0;
        next.inventory[0] += next.population[0];
        next.inventory[1] += next.population[1];
        next.inventory[2] += next.population[2];
        next.inventory[3] += next.population[3];
        next
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct Limits {
    ore: i64,
    clay: i64,
    obsidian: i64,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct Blueprint {
    id: i64,
    ore: OreRobot,
    clay: ClayRobot,
    obsidian: ObsidianRobot,
    geode: GeodeRobot,
}

impl Blueprint {
    pub fn most_geodes_in_time(&self, minutes: i64) -> i64 {
        let limits = Limits {
            ore: self
                .ore
                .ore_cost()
                .max(self.clay.ore_cost())
                .max(self.obsidian.ore_cost())
                .max(self.geode.ore_cost()),
            clay: self.obsidian.costs().clay,
            obsidian: self.geode.costs().obsidian,
        };

        let state = State {
            time_remaining: minutes,
            ..Default::default()
        };

        let mut best = 0;

        self.search(&state, &limits, &mut best);
        best
    }

    pub fn search(&self, state: &State, limits: &Limits, best: &mut i64) {
        if state.time_remaining <= 1 {
            let geodes = state.geodes_with_remaining_time();
            if geodes > *best {
                *best = geodes;
            }
            return;
        }

        if state.geode_limit() < *best {
            return;
        }

        if state.obsidian_limit() < self.geode.costs().obsidian {
            let geodes = state.geodes_with_remaining_time();
            if geodes > *best {
                *best = geodes;
            }
            return;
        }

        let mut next_state = state.next();

        // build geode robots always
        if self.geode.can_afford(&state.inventory) {
            self.geode.pay_for(&mut next_state.inventory);
            next_state.population[3] += 1;
            return self.search(&next_state, &limits, best);
        }

        // try to build any other robots. We're going to try in reverse order
        // to maybe cut out some cycles
        let mut can_afford = 0u8;

        if state.skipped & Obsidian::BIT == 0
            && self.obsidian.can_afford(&state.inventory)
            && state.population[2] < limits.obsidian
        {
            can_afford |= Obsidian::BIT;
            let mut next_state = next_state;
            self.obsidian.pay_for(&mut next_state.inventory);
            next_state.population[2] += 1;
            self.search(&next_state, &limits, best);
        }

        if state.skipped & Clay::BIT == 0
            && self.clay.can_afford(&state.inventory)
            && state.population[1] < limits.clay
        {
            can_afford |= Clay::BIT;
            let mut next_state = next_state;
            self.clay.pay_for(&mut next_state.inventory);
            next_state.population[1] += 1;
            self.search(&next_state, &limits, best);
        }

        if state.skipped & Ore::BIT == 0
            && self.ore.can_afford(&state.inventory)
            && state.population[0] < limits.ore
        {
            can_afford |= Ore::BIT;
            let mut next_state = next_state;
            self.ore.pay_for(&mut next_state.inventory);
            next_state.population[0] += 1;
            self.search(&next_state, &limits, best);
        }

        // now simulate not buying, but set the ones we skipped to the ones we
        // could have bought
        next_state.skipped = can_afford;
        self.search(&next_state, &limits, best);
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

    Ok((
        input,
        Blueprint {
            id,
            ore,
            clay,
            obsidian,
            geode,
        },
    ))
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
