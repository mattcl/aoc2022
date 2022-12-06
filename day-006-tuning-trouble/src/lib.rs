use std::str::FromStr;

use anyhow::bail;
use aoc_plumbing::{bits::char_to_mask, Problem};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TuningTrouble {
    message: Vec<u64>,
}

impl TuningTrouble {
    pub fn find_unique(&self, size: usize) -> Result<usize, anyhow::Error> {
        let mut idx = size - 1;
        'outer: while idx < self.message.len() {
            let mut sum = self.message[idx];

            for i in 1..size {
                let cur = idx - i;
                let v = self.message[cur];
                if sum & v > 0 {
                    // we know the new index to start from is cur + 1
                    idx = cur + size;
                    continue 'outer;
                }

                sum |= v;
            }

            return Ok(idx + 1);
        }

        bail!("None found");
    }
}

impl FromStr for TuningTrouble {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            message: s.chars().map(char_to_mask).collect(),
        })
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
        self.find_unique(4)
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        self.find_unique(14)
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
