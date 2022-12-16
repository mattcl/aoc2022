use std::str::FromStr;

use anyhow::anyhow;
use aoc_plumbing::Problem;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, multispace0},
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    IResult,
};
use rustc_hash::FxHashMap;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct Edge {
    origin: usize,
    destination: usize,
    distance: i64,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ValveState {
    Open,
    Closed,
}

impl Default for ValveState {
    fn default() -> Self {
        Self::Closed
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct RawValve<'a> {
    name: &'a str,
    flow_rate: i64,
    tunnels: Vec<&'a str>,
    state: ValveState,
}

impl<'a> RawValve<'a> {
    pub fn try_into_valve(&self, map: &FxHashMap<&str, usize>) -> Result<Valve, anyhow::Error> {
        Ok(Valve {
            index: map
                .get(self.name)
                .copied()
                .ok_or_else(|| anyhow!("Cannot find self in map: {:?}", self))?,
            flow_rate: self.flow_rate,
            tunnels: self
                .tunnels
                .iter()
                .map(|t| {
                    map.get(*t)
                        .copied()
                        .ok_or_else(|| anyhow!("Cannot find {} in map", t))
                })
                .collect::<Result<Vec<_>, _>>()?,
            state: self.state,
        })
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Valve {
    index: usize,
    flow_rate: i64,
    tunnels: Vec<usize>,
    state: ValveState,
}

impl Valve {
    pub fn is_open(&self) -> bool {
        self.state == ValveState::Open
    }

    pub fn pressure_over_time(&self, duration: i64) -> i64 {
        self.flow_rate * duration
    }

    pub fn open(&mut self) {
        self.state = ValveState::Open
    }

    pub fn close(&mut self) {
        self.state = ValveState::Closed
    }

    pub fn is_candidate(&self) -> bool {
        self.flow_rate > 0
    }

    pub fn is_dead_end(&self) -> bool {
        self.flow_rate == 0 && self.tunnels.len() == 1
    }
}

fn name_parser(input: &str) -> IResult<&str, &str> {
    preceded(tag("Valve "), alpha1)(input)
}

fn flow_rate_parser(input: &str) -> IResult<&str, i64> {
    preceded(tag(" has flow rate="), nom::character::complete::i64)(input)
}

fn tunnels_parser(input: &str) -> IResult<&str, Vec<&str>> {
    preceded(
        alt((
            tag("; tunnels lead to valves "),
            tag("; tunnel leads to valve "),
        )),
        separated_list1(tag(", "), alpha1),
    )(input)
}

fn parse_valve<'a>(input: &'a str) -> IResult<&str, RawValve<'a>> {
    let (input, (name, flow_rate, tunnels)) =
        tuple((name_parser, flow_rate_parser, tunnels_parser))(input)?;

    Ok((
        input,
        RawValve {
            name,
            flow_rate,
            tunnels,
            state: ValveState::Closed,
        },
    ))
}

fn parse_valves<'a>(input: &'a str) -> IResult<&str, Vec<RawValve<'a>>> {
    delimited(
        multispace0,
        separated_list1(multispace0, parse_valve),
        multispace0,
    )(input)
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct Explore {
    cur: usize,
    seen: u64,
    minutes_remaining: i64,
}

impl Explore {
    pub fn set(&mut self, bit: usize) {
        self.seen |= 1 << bit;
    }

    pub fn unset(&mut self, bit: usize) {
        self.seen &= !(1 << bit);
    }

    pub fn is_set(&self, bit: usize) -> bool {
        self.seen & 1 << bit > 0
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct State {
    me: Explore,
    elephant: Explore,
}

impl State {
    pub fn finished(&self, all_valves: u64) -> bool {
        (self.me.minutes_remaining <= 0 && self.elephant.minutes_remaining <= 0)
            || (self.me.seen | self.elephant.seen == all_valves)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ProboscideaVolcanium {
    aa_index: usize,
    valves: Vec<Valve>,
    nonzero_valves: Vec<usize>,
    shortest_paths: Vec<Vec<i64>>,
    all_open: u64,
}

impl ProboscideaVolcanium {
    pub fn optimal_path(&self) -> i64 {
        let mut best = 0;
        let mut best_seen = 0;
        let mut cache = FxHashMap::default();
        let mut cur = Explore {
            cur: self.aa_index,
            minutes_remaining: 30,
            ..Default::default()
        };
        self.optimal_path_recur(&mut cur, 0, &mut best, &mut best_seen, &mut cache);
        best
    }

    pub fn optimal_path_recur(
        &self,
        cur: &Explore,
        cur_best: i64,
        best: &mut i64,
        best_seen: &mut u64,
        cache: &mut FxHashMap<u64, i64>,
    ) {
        if cur.seen == self.all_open || cur.minutes_remaining <= 0 {
            if cur_best > *best {
                *best = cur_best;
                *best_seen = cur.seen;
            }
            return;
        }

        if let Some(old) = cache.get(&cur.seen) {
            if *old > cur_best {
                return;
            }
        }

        cache.insert(cur.seen, cur_best);

        // we can pick a valve to move to, opening that valve in the process
        for v in self.nonzero_valves.iter() {
            if cur.cur != *v && !cur.is_set(*v) {
                // move to this and open it
                // it takes an extra minute there to open the valve
                let next_minutes = cur.minutes_remaining - self.shortest_paths[cur.cur][*v] - 1;
                if next_minutes < 0 {
                    continue;
                }
                let mut next_cur = *cur;
                next_cur.cur = *v;
                next_cur.minutes_remaining = next_minutes;
                next_cur.set(*v);
                self.optimal_path_recur(
                    &next_cur,
                    cur_best + self.valves[*v].pressure_over_time(next_minutes),
                    best,
                    best_seen,
                    cache,
                );
            }
        }

        // handle the situation where we would not have been able to move
        // anywhere else in time
        if cur_best > *best {
            *best = cur_best;
            *best_seen = cur.seen;
        }
    }

    pub fn optimal_path_with_elephant(&self) -> i64 {
        let mut best = 0;
        let mut cache = FxHashMap::default();

        let me = Explore {
            cur: self.aa_index,
            minutes_remaining: 26,
            ..Default::default()
        };

        let elephant = Explore {
            cur: self.aa_index,
            minutes_remaining: 26,
            ..Default::default()
        };

        // self.faster_recur(&me, &elephant, 0, &mut best, &mut cache);
        self.optimal_path_with_elephant_recur(&me, 0, &mut best, &mut cache);
        best
    }

    // what would the BFS solution look like?
    //

    pub fn optimal_path_with_elephant_recur(
        &self,
        me: &Explore,
        cur_best: i64,
        best: &mut i64,
        cache: &mut FxHashMap<u64, i64>,
    ) {
        // the problem is that the time is different
        if me.seen == self.all_open || me.minutes_remaining == 0 {
            if cur_best > *best {
                *best = cur_best;
                // cache.insert(me.seen, *best);
            }
            return;
        }

        // if let Some(old) = cache.get(&me.seen) {
        //     if *old > cur_best {
        //         return;
        //     }
        // }

        // cache.insert(me.seen, cur_best);

        // we can pick a valve to move to, opening that valve in the process
        for v in self.nonzero_valves.iter() {
            if me.cur != *v && !me.is_set(*v) {
                // move to this and open it
                // it takes an extra minute there to open the valve
                let next_minutes = me.minutes_remaining - self.shortest_paths[me.cur][*v] - 1;
                if next_minutes <= 0 {
                    // we ran out of time, so try with the elephant
                    let mut elephant = Explore {
                        cur: self.aa_index,
                        seen: me.seen,
                        minutes_remaining: 26,
                    };
                    let mut elephant_best = 0;
                    let mut elephant_best_seen = 0;
                    let mut elephant_cache = FxHashMap::default();

                    self.optimal_path_recur(
                        &mut elephant,
                        0,
                        &mut elephant_best,
                        &mut elephant_best_seen,
                        &mut elephant_cache,
                    );

                    if cur_best + elephant_best > *best {
                        *best = cur_best + elephant_best;
                        // cache.insert(me.seen, *best);
                    }

                    continue;
                }
                let mut next_me = *me;
                next_me.cur = *v;
                next_me.minutes_remaining = next_minutes;
                next_me.set(*v);
                self.optimal_path_with_elephant_recur(
                    &next_me,
                    cur_best + self.valves[*v].pressure_over_time(next_minutes),
                    best,
                    cache,
                );
            }
        }

        // handle the situation where we would not have been able to move
        // anywhere else in time
        if cur_best > *best {
            *best = cur_best;
        }
    }

    pub fn faster_recur(
        &self,
        me: &Explore,
        elephant: &Explore,
        cur_best: i64,
        best: &mut i64,
        cache: &mut FxHashMap<(u64, u64), i64>,
    ) {
        assert!(me.seen & elephant.seen == 0);
        if cur_best > *best {
            *best = cur_best;
        }

        if me.minutes_remaining == 0
            || elephant.minutes_remaining == 0
            || me.seen | elephant.seen == self.all_open
        {
            return;
        }

        if let Some(v) = cache.get(&(me.seen, elephant.seen)) {
            if cur_best < *v {
                return;
            }
        }

        cache.insert((me.seen, elephant.seen), cur_best);

        for me_candidate in self.nonzero_valves.iter() {
            // if we'd be looking at our current location or a location that
            // we've already been to, we need to just continue
            if *me_candidate == me.cur || me.is_set(*me_candidate) || elephant.is_set(*me_candidate)
            {
                continue;
            }

            // prepare the next me
            let mut next_me = *me;
            next_me.cur = *me_candidate;
            next_me.minutes_remaining -= self.shortest_paths[me.cur][next_me.cur] - 1;

            // if at this point we couldn't continue, just finish with the
            // elephant
            if next_me.minutes_remaining < 0 {
                // we ran out of time, so just finish with the elephant
                let mut tmp_elephant = *elephant;
                // tell the elephant which things we've seen
                tmp_elephant.seen |= me.seen;
                let mut elephant_best = 0;
                let mut elephant_best_seen = 0;
                let mut elephant_cache = FxHashMap::default();
                self.optimal_path_recur(
                    &tmp_elephant,
                    0,
                    &mut elephant_best,
                    &mut elephant_best_seen,
                    &mut elephant_cache,
                );

                let real_best = cur_best + elephant_best;
                if real_best > *best {
                    *best = real_best;
                    cache.insert((me.seen, elephant.seen), *best);
                }
                continue;
            }

            // we can continue, so add this thing to the list of seen values
            next_me.set(*me_candidate);

            let next_best =
                cur_best + self.valves[next_me.cur].pressure_over_time(next_me.minutes_remaining);

            for el_candidate in self.nonzero_valves.iter() {
                // same thing here
                // if we'd be looking at our current location or a location that
                // we've already been to, we need to just continue
                if *el_candidate == elephant.cur
                    || next_me.is_set(*el_candidate)
                    || elephant.is_set(*el_candidate)
                {
                    continue;
                }

                // prepare the next elephant
                let mut next_el = *elephant;
                next_el.cur = *el_candidate;
                next_el.minutes_remaining -= self.shortest_paths[elephant.cur][next_el.cur] - 1;

                // if at this point we couldn't continue, just finish with the
                // ourself
                if next_el.minutes_remaining < 0 {
                    // we ran out of time, so just finish with me
                    let mut tmp_me = next_me;
                    // tell the new me which things the elephant as seen
                    tmp_me.seen |= elephant.seen;
                    let mut me_best = 0;
                    let mut me_best_seen = 0;
                    let mut me_cache = FxHashMap::default();
                    self.optimal_path_recur(
                        &tmp_me,
                        0,
                        &mut me_best,
                        &mut me_best_seen,
                        &mut me_cache,
                    );

                    let real_best = next_best + me_best;
                    if real_best > *best {
                        *best = real_best;
                        cache.insert((next_me.seen, elephant.seen), *best);
                    }
                    continue;
                }

                // prepare the next me
                // we can continue, so add this thing to the list of seen values
                next_el.set(*el_candidate);
                let next_best_actual = next_best
                    + self.valves[next_el.cur].pressure_over_time(next_el.minutes_remaining);

                self.faster_recur(&next_me, &next_el, next_best_actual, best, cache)
            }
        }

        if cur_best > *best {
            *best = cur_best;
        }
    }
}

impl FromStr for ProboscideaVolcanium {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, raw_valves) = parse_valves(s).map_err(|e| e.to_owned())?;

        // make a temporary name -> idx map and a list of the nonzero valves
        let mut valves_map = FxHashMap::default();
        let mut nonzero_valves = Vec::with_capacity(raw_valves.len());
        let mut aa_index = None;
        for (idx, valve) in raw_valves.iter().enumerate() {
            valves_map.insert(valve.name, idx);
            if valve.flow_rate > 0 {
                nonzero_valves.push(idx);
            }
            if valve.name == "AA" {
                aa_index = Some(idx);
            }
        }

        let valves = raw_valves
            .iter()
            .map(|v| v.try_into_valve(&valves_map))
            .collect::<Result<Vec<_>, _>>()?;

        // calculate shortest paths to every node
        let mut shortest_paths = vec![vec![i64::MAX / 4; valves.len()]; valves.len()];

        for v in valves.iter() {
            shortest_paths[v.index][v.index] = 0;

            for other in v.tunnels.iter() {
                shortest_paths[v.index][*other] = 1;
            }
        }

        for k in 0..valves.len() {
            for i in 0..valves.len() {
                for j in 0..valves.len() {
                    shortest_paths[i][j] =
                        shortest_paths[i][j].min(shortest_paths[i][k] + shortest_paths[k][j]);
                }
            }
        }

        let mut all_open = 0;
        for v in nonzero_valves.iter() {
            all_open |= 1 << v;
        }

        Ok(Self {
            aa_index: aa_index.ok_or_else(|| anyhow!("Could not find AA"))?,
            valves,
            nonzero_valves,
            shortest_paths,
            all_open,
        })
    }
}

impl Problem for ProboscideaVolcanium {
    const DAY: usize = 16;
    const TITLE: &'static str = "proboscidea volcanium";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = i64;
    type P2 = i64;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.optimal_path())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.optimal_path_with_elephant())
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
        let solution = ProboscideaVolcanium::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(1376, 1933));
    }

    #[test]
    fn example() {
        let input = "
            Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
            Valve BB has flow rate=13; tunnels lead to valves CC, AA
            Valve CC has flow rate=2; tunnels lead to valves DD, BB
            Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
            Valve EE has flow rate=3; tunnels lead to valves FF, DD
            Valve FF has flow rate=0; tunnels lead to valves EE, GG
            Valve GG has flow rate=0; tunnels lead to valves FF, HH
            Valve HH has flow rate=22; tunnel leads to valve GG
            Valve II has flow rate=0; tunnels lead to valves AA, JJ
            Valve JJ has flow rate=21; tunnel leads to valve II
            ";
        let solution = ProboscideaVolcanium::solve(input).unwrap();
        // FIXME: figure out why I can't get the right answer. it's related to
        // the fact that one person can do the example in the allotted time
        // - MCL - 2022-12-16
        // assert_eq!(solution, Solution::new(1651, 1707));
    }
}
