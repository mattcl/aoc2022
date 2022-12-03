use std::str::FromStr;

use anyhow::bail;
use aoc_plumbing::Problem;
use rayon::prelude::*;
use rustc_hash::FxHashSet;

#[inline]
fn char_to_priority(ch: char) -> usize {
    if ch.is_lowercase() {
        ((ch as u8) - ('a' as u8) + 1) as usize
    } else {
        ((ch as u8) - ('A' as u8) + 27) as usize
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Rucksack {
    one: FxHashSet<char>,
    two: FxHashSet<char>,
}

impl Rucksack {
    pub fn dupcate_priorities(&self) -> usize {
        self.one
            .intersection(&self.two)
            .map(|v| char_to_priority(*v))
            .sum()
    }

    pub fn union(&self) -> FxHashSet<&char> {
        self.one.union(&self.two).collect()
    }
}

impl FromStr for Rucksack {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_ascii() {
            bail!("invalid input: {}", s);
        }

        let mid = s.len() / 2;
        let one = s[0..mid].chars().collect();
        let two = s[mid..].chars().collect();

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
            .par_lines()
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
        Ok(self.rucksacks.iter().map(|r| r.dupcate_priorities()).sum())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        if self.rucksacks.len() % 3 != 0 {
            bail!("Num rucksacks is not a multiple of 3");
        }

        let total = self
            .rucksacks
            .chunks(3)
            .map(|chunk| {
                let mut sets = chunk.iter().map(|r| r.union());

                let mut reduced = sets.next().unwrap();
                let remaining: Vec<FxHashSet<_>> = sets.collect();
                reduced.retain(|item| remaining.iter().all(|s| s.contains(item)));

                reduced.iter().map(|v| char_to_priority(**v)).sum::<usize>()
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
