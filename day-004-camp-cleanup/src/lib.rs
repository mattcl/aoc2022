use std::str::FromStr;

use aoc_plumbing::Problem;
use nom::{
    bytes::complete::tag,
    character::complete::{self, multispace0},
    multi::many1,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct Assignment {
    start: u64,
    end: u64,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct Pair {
    left: Assignment,
    right: Assignment,
}

impl Pair {
    pub fn complete_overlap(&self) -> bool {
        (self.left.start >= self.right.start && self.left.end <= self.right.end)
            || (self.right.start >= self.left.start && self.right.end <= self.left.end)
    }

    pub fn partial_overlap(&self) -> bool {
        !(self.left.end < self.right.start) && !(self.right.end < self.left.start)
    }
}

fn assignment_parser(input: &str) -> IResult<&str, Assignment> {
    let (input, (start, end)) = separated_pair(complete::u64, tag("-"), complete::u64)(input)?;
    Ok((input, Assignment { start, end }))
}

fn pair_parser(input: &str) -> IResult<&str, Pair> {
    let (input, (left, right)) =
        separated_pair(assignment_parser, tag(","), assignment_parser)(input)?;
    Ok((input, Pair { left, right }))
}

fn pairs_parser(input: &str) -> IResult<&str, Vec<Pair>> {
    many1(preceded(multispace0, pair_parser))(input)
}

impl FromStr for Pair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, pair) = pair_parser(s).map_err(|e| e.to_owned())?;
        Ok(pair)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CampCleanup {
    assignments: Vec<Pair>,
}

impl FromStr for CampCleanup {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, assignments) = pairs_parser(s).map_err(|e| e.to_owned())?;
        Ok(Self { assignments })
    }
}

impl Problem for CampCleanup {
    const DAY: usize = 4;
    const TITLE: &'static str = "camp cleanup";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self
            .assignments
            .iter()
            .filter(|a| a.complete_overlap())
            .count())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self
            .assignments
            .iter()
            .filter(|a| a.partial_overlap())
            .count())
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
        let solution = CampCleanup::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(494, 833));
    }

    #[test]
    fn example() {
        let input = " 2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8 ";
        dbg!(&input);
        let solution = CampCleanup::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(2, 4));
    }
}
