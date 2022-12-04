use std::str::FromStr;

use anyhow::bail;
use aoc_plumbing::Problem;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct Assignment {
    start: usize,
    end: usize,
}

impl FromStr for Assignment {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vals = s
            .trim()
            .split('-')
            .map(|v| v.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?;
        if vals.len() != 2 {
            bail!("Invalid assignment: {}", s);
        }

        Ok(Self {
            start: vals[0],
            end: vals[1],
        })
    }
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
        !((self.left.end < self.right.start) || (self.right.end < self.left.start))
            || self.complete_overlap()
    }
}

impl FromStr for Pair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vals = s
            .trim()
            .split(',')
            .map(|v| v.parse())
            .collect::<Result<Vec<_>, _>>()?;
        if vals.len() != 2 {
            bail!("Invalid pair: {}", s);
        }

        Ok(Self {
            left: vals[0],
            right: vals[1],
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CampCleanup {
    assignments: Vec<Pair>,
}

impl FromStr for CampCleanup {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let assignments = s
            .trim()
            .lines()
            .map(|v| v.parse())
            .collect::<Result<Vec<_>, _>>()?;
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
        let input = "
            2-4,6-8
            2-3,4-5
            5-7,7-9
            2-8,3-7
            6-6,4-6
            2-6,4-8
            ";
        let solution = CampCleanup::solve(input).unwrap();
        assert_eq!(solution, Solution::new(2, 4));
    }
}
