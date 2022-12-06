use std::{collections::VecDeque, str::FromStr};

use anyhow::anyhow;
use aoc_plumbing::Problem;
use rustc_hash::FxHashSet;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Window<const N: usize>(VecDeque<char>);

impl<const N: usize> Window<N> {
    pub fn unique(&self) -> bool {
        FxHashSet::from_iter(self.0.iter().copied()).len() == N
    }

    pub fn insert(&mut self, ch: char) {
        self.0.push_back(ch);

        if self.0.len() > N {
            self.0.pop_front();
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TuningTrouble {
    message: String,
}

impl TuningTrouble {
    pub fn first_marker(&self) -> Option<usize> {
        let mut window = Window::<4>::default();

        for (idx, ch) in self.message.chars().enumerate() {
            window.insert(ch);
            if window.unique() {
                return Some(idx + 1);
            }
        }

        None
    }

    pub fn first_message(&self) -> Option<usize> {
        let mut window = Window::<14>::default();

        for (idx, ch) in self.message.chars().enumerate() {
            window.insert(ch);
            if window.unique() {
                return Some(idx + 1);
            }
        }

        None
    }
}

impl FromStr for TuningTrouble {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { message: s.into() })
    }
}

impl Problem for TuningTrouble {
    const DAY: usize = 6;
    const TITLE: &'static str = "tuning trouble";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        self.first_marker().ok_or_else(|| anyhow!("None found"))
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        self.first_message().ok_or_else(|| anyhow!("None found"))
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
        let solution = TuningTrouble::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(1625, 2250));
    }

    #[test]
    fn example() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let solution = TuningTrouble::solve(input).unwrap();
        assert_eq!(solution, Solution::new(7, 19));
    }
}
