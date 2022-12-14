use std::str::FromStr;

use anyhow::bail;
use aoc_plumbing::{bits::char_to_mask, Problem};

#[inline]
fn priority_sum_from_bin(bin: u64) -> usize {
    let mut offset = bin.trailing_zeros() as usize;
    let mut shifted = bin;
    let mut total_shift = 0_usize;
    let mut sum = 0;

    while shifted > 0 {
        shifted = shifted >> (offset + 1);
        total_shift += offset + 1;
        sum += total_shift;
        offset = shifted.trailing_zeros() as usize;
    }

    sum
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Rucksack {
    one: u64,
    two: u64,
}

impl Rucksack {
    pub fn duplicate_priorities(&self) -> usize {
        priority_sum_from_bin(self.one & self.two)
    }

    pub fn union(&self) -> u64 {
        self.one | self.two
    }
}

impl FromStr for Rucksack {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_ascii() {
            bail!("invalid input: {}", s);
        }

        let mid = s.len() / 2;
        let one = s[0..mid].chars().fold(0, |acc, ch| acc | char_to_mask(ch));
        let two = s[mid..].chars().fold(0, |acc, ch| acc | char_to_mask(ch));

        Ok(Self { one, two })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RucksackReorganization {
    rucksacks: Vec<Rucksack>,
}

impl FromStr for RucksackReorganization {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rucksacks = s
            .trim()
            .lines()
            .map(|l| Rucksack::from_str(l.trim()))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { rucksacks })
    }
}

impl Problem for RucksackReorganization {
    const DAY: usize = 3;
    const TITLE: &'static str = "rucksack reorganization";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self
            .rucksacks
            .iter()
            .map(|r| r.duplicate_priorities())
            .sum())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        if self.rucksacks.len() % 3 != 0 {
            bail!("Num rucksacks is not a multiple of 3");
        }

        let total = self
            .rucksacks
            .chunks(3)
            .map(|chunk| {
                priority_sum_from_bin(
                    chunk
                        .iter()
                        .fold(chunk[0].union(), |acc, r| acc & r.union()),
                )
            })
            .sum();

        Ok(total)
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
        let solution = RucksackReorganization::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(7597, 2607));
    }

    #[test]
    fn example() {
        let input = "
            vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw
            ";
        let solution = RucksackReorganization::solve(input).unwrap();
        assert_eq!(solution, Solution::new(157, 70));
    }
}
