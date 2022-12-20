use std::str::FromStr;

use anyhow::anyhow;
use aoc_plumbing::Problem;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    multi::separated_list1,
    sequence::{preceded, tuple},
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
    separated_list1(newline, parse_valve)(input)
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
        self.seen & (1 << bit) > 0
    }
}

#[derive(Debug, Clone, Default)]
pub struct ProboscideaVolcanium {
    aa_index: usize,
    valves: Vec<Valve>,
    nonzero_valves: Vec<usize>,
    shortest_paths: Vec<Vec<i64>>,
    all_open: u64,
}

impl ProboscideaVolcanium {
    pub fn optimal_path(&self, minutes: i64, cache: &mut FxHashMap<(usize, u64), i64>) -> i64 {
        let mut best = 0;
        let mut cur = Explore {
            cur: self.aa_index,
            minutes_remaining: minutes,
            ..Default::default()
        };
        self.optimal_path_recur(&mut cur, 0, &mut best, cache);
        best
    }

    pub fn optimal_path_recur(
        &self,
        cur: &Explore,
        cur_best: i64,
        best: &mut i64,
        cache: &mut FxHashMap<(usize, u64), i64>,
    ) {
        if cur_best > *best {
            *best = cur_best;
        }

        if cur.seen == self.all_open {
            cache.insert((cur.cur, cur.seen), cur_best);
            return;
        }

        if let Some(old) = cache.get(&(cur.cur, cur.seen)) {
            if *old > cur_best {
                return;
            }
        }

        cache.insert((cur.cur, cur.seen), cur_best);

        // we can pick a valve to move to, opening that valve in the process
        // we have to iterate this way to avoid the mutable/immutable borrow
        for v in self.nonzero_valves.iter() {
            let v = *v;
            if cur.cur != v && !cur.is_set(v) {
                // move to this and open it
                // it takes an extra minute there to open the valve
                let next_minutes = cur.minutes_remaining - self.shortest_paths[cur.cur][v] - 1;
                if next_minutes < 0 {
                    continue;
                }
                let mut next_cur = *cur;
                next_cur.cur = v;
                next_cur.minutes_remaining = next_minutes;
                next_cur.set(v);
                self.optimal_path_recur(
                    &next_cur,
                    cur_best
                        + self.valves[next_cur.cur].pressure_over_time(next_cur.minutes_remaining),
                    best,
                    cache,
                );
            }
        }
    }

    pub fn find_best_disjoint_pair(&self, path_cache: &FxHashMap<(usize, u64), i64>) -> i64 {
        let mut best = i64::MIN;

        // there's a special case where we were able to open all the valves
        // ourself, so we need to remove that from the list
        let mut ordered = path_cache
            .iter()
            .filter(|((_, m), _)| *m != self.all_open)
            .collect::<Vec<_>>();
        ordered.sort_by(|a, b| a.1.cmp(&b.1));

        while let Some(((_, valve_map), total)) = ordered.pop() {
            // we know the list is sorted, so the total we have is the largest
            // total remaining, so if we (x2) can't beat the best score so far,
            // there is no point looking at the rest of the list.
            if total * 2 < best {
                break;
            }
            for ((_, other_map), other_total) in ordered.iter().rev() {
                if *other_map & valve_map != 0 {
                    continue;
                }

                if total + *other_total <= best {
                    break;
                } else {
                    best = total + *other_total;
                }
            }
        }

        best
    }
}

impl FromStr for ProboscideaVolcanium {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, raw_valves) = parse_valves(s.trim()).map_err(|e| e.to_owned())?;

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
        let mut cache = FxHashMap::default();
        Ok(self.optimal_path(30, &mut cache))
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        let mut cache = FxHashMap::default();
        self.optimal_path(26, &mut cache);
        Ok(self.find_best_disjoint_pair(&cache))
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
        let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
        let solution = ProboscideaVolcanium::solve(input).unwrap();
        assert_eq!(solution, Solution::new(1651, 1707));
    }

    #[test]
    fn explore() {
        let mut e = Explore {
            cur: 0,
            seen: 0,
            minutes_remaining: 0,
        };

        e.set(3);
        assert!(e.is_set(3));
        assert_eq!(e.seen, 0b1000);

        e.set(4);
        assert!(e.is_set(3));
        assert!(e.is_set(4));
        assert_eq!(e.seen, 0b11000);

        e.unset(3);
        assert!(!e.is_set(3));
        assert!(e.is_set(4));
        assert_eq!(e.seen, 0b10000);
    }
}
