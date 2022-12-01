use std::str::FromStr;

use anyhow::anyhow;
use aoc_plumbing::Problem;
use itertools::Itertools;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Elf {
    food: Vec<usize>,
}

impl Elf {
    pub fn calories(&self) -> usize {
        self.food.iter().sum()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CalorieCounting {
    elves: Vec<Elf>,
}

impl FromStr for CalorieCounting {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.trim().lines().collect();

        let elves: Vec<_> = lines
            .split(|l| l.is_empty())
            .map(|items| {
                items
                    .iter()
                    .map(|item| item.trim().parse::<usize>())
                    .collect::<Result<Vec<_>, _>>()
            })
            .map_ok(|food| Elf { food })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { elves })
    }
}

impl Problem for CalorieCounting {
    const DAY: usize = 1;
    const TITLE: &'static str = "calorie counting";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        self.elves
            .iter()
            .map(|e| e.calories())
            .max()
            .ok_or_else(|| anyhow!("Could not get max value"))
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self
            .elves
            .iter()
            .map(|e| e.calories())
            .sorted_by(|a, b| b.cmp(&a))
            .take(3)
            .sum())
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
        let solution = CalorieCounting::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(69795, 208437));
    }

    #[test]
    fn example() {
        let input = "
            1000
            2000
            3000

            4000

            5000
            6000

            7000
            8000
            9000

            10000
            ";
        let solution = CalorieCounting::solve(input).unwrap();
        assert_eq!(solution, Solution::new(24000, 45000));
    }
}
