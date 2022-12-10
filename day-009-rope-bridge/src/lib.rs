use std::{hash::Hash, str::FromStr};

use anyhow::bail;
use aoc_plumbing::Problem;
use nom::{
    character::complete::multispace0,
    combinator::map_res,
    multi::many1,
    sequence::{preceded, separated_pair},
    IResult,
};
use rustc_hash::FxHashSet;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct Location {
    x: i64,
    y: i64,
}

impl Hash for Location {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_i64(self.x * 10_000_000 + self.y);
    }
}

impl Location {
    pub fn touching(&self, other: &Self) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Motion {
    Up(i64),
    Down(i64),
    Right(i64),
    Left(i64),
}

impl Motion {
    pub fn value(&self) -> i64 {
        *match self {
            Self::Up(v) => v,
            Self::Down(v) => v,
            Self::Right(v) => v,
            Self::Left(v) => v,
        }
    }
}

impl TryFrom<(char, i64)> for Motion {
    type Error = anyhow::Error;

    fn try_from((ch, val): (char, i64)) -> Result<Self, Self::Error> {
        match ch {
            'U' => Ok(Self::Up(val)),
            'D' => Ok(Self::Down(val)),
            'R' => Ok(Self::Right(val)),
            'L' => Ok(Self::Left(val)),
            _ => bail!("Invalid direction: {}", ch),
        }
    }
}

fn parse_motion(input: &str) -> IResult<&str, Motion> {
    map_res(
        separated_pair(
            nom::character::complete::anychar,
            nom::character::complete::multispace1,
            nom::character::complete::i64,
        ),
        Motion::try_from,
    )(input)
}

fn parse_motions(input: &str) -> IResult<&str, Vec<Motion>> {
    many1(preceded(multispace0, parse_motion))(input)
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Rope<const N: usize> {
    knots: [Location; N],
}

impl<const N: usize> Default for Rope<N> {
    fn default() -> Self {
        Self {
            knots: [Location::default(); N],
        }
    }
}

impl<const N: usize> Rope<N> {
    pub fn apply(&mut self, motion: &Motion, visited: &mut FxHashSet<Location>) {
        match motion {
            Motion::Up(v) => self.knots[0].y += v,
            Motion::Down(v) => self.knots[0].y -= v,
            Motion::Right(v) => self.knots[0].x += v,
            Motion::Left(v) => self.knots[0].x -= v,
        }

        'outer: loop {
            for cur in 1..N {
                let prev = cur - 1;

                if !self.knots[cur].touching(&self.knots[prev]) {
                    self.knots[cur].y += (self.knots[prev].y - self.knots[cur].y).signum();
                    self.knots[cur].x += (self.knots[prev].x - self.knots[cur].x).signum();
                } else if cur == 1 {
                    break 'outer;
                } else {
                    continue;
                }

                if cur == N - 1 {
                    visited.insert(self.knots[cur]);
                }
            }
        }
    }
}

#[derive(Debug, Clone, Eq, Default, PartialEq)]
pub struct RopeBridge {
    motions: Vec<Motion>,
}

impl FromStr for RopeBridge {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, motions) = parse_motions(s).map_err(|e| e.to_owned())?;

        Ok(Self { motions })
    }
}

impl Problem for RopeBridge {
    const DAY: usize = 9;
    const TITLE: &'static str = "rope bridge";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        let mut visited: FxHashSet<Location> = FxHashSet::default();

        let mut rope = Rope::<2>::default();
        visited.insert(rope.knots[0]);

        for motion in self.motions.iter() {
            rope.apply(motion, &mut visited);
        }

        Ok(visited.len())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        let mut visited: FxHashSet<Location> = FxHashSet::default();

        let mut rope = Rope::<10>::default();
        visited.insert(rope.knots[0]);

        for motion in self.motions.iter() {
            rope.apply(motion, &mut visited);
        }

        Ok(visited.len())
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
        let solution = RopeBridge::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(6057, 2514));
    }

    #[test]
    fn example() {
        let input = "
            R 5
            U 8
            L 8
            D 3
            R 17
            D 10
            L 25
            U 20
            ";
        let solution = RopeBridge::solve(input).unwrap();
        assert_eq!(solution, Solution::new(88, 36));
    }

    #[test]
    fn other() {
        let input = "
            R 2
            L 4
            ";
        let solution = RopeBridge::solve(input).unwrap();
        assert_eq!(solution, Solution::new(3, 1));
    }
}
