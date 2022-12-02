use std::str::FromStr;

use anyhow::{anyhow, bail};
use aoc_plumbing::Problem;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    pub fn score(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    pub fn match_desire(&self, desire: &Desire) -> Self {
        match self {
            Self::Rock => match desire {
                Desire::Win => Self::Paper,
                Desire::Lose => Self::Scissors,
                Desire::Draw => Self::Rock,
            },
            Self::Paper => match desire {
                Desire::Win => Self::Scissors,
                Desire::Lose => Self::Rock,
                Desire::Draw => Self::Paper,
            },
            Self::Scissors => match desire {
                Desire::Win => Self::Rock,
                Desire::Lose => Self::Paper,
                Desire::Draw => Self::Scissors,
            },
        }
    }
}

impl Ord for Choice {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            Self::Rock => match other {
                Self::Rock => std::cmp::Ordering::Equal,
                Self::Paper => std::cmp::Ordering::Less,
                Self::Scissors => std::cmp::Ordering::Greater,
            },
            Self::Paper => match other {
                Self::Rock => std::cmp::Ordering::Greater,
                Self::Paper => std::cmp::Ordering::Equal,
                Self::Scissors => std::cmp::Ordering::Less,
            },
            Self::Scissors => match other {
                Self::Rock => std::cmp::Ordering::Less,
                Self::Paper => std::cmp::Ordering::Greater,
                Self::Scissors => std::cmp::Ordering::Equal,
            },
        }
    }
}

impl PartialOrd for Choice {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Choice {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => bail!("Invalid choice {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Round {
    other: Choice,
    you: Choice,
    desire: Desire,
}

impl Round {
    pub fn score(&self) -> usize {
        let score = self.you.score();

        match self.you.cmp(&self.other) {
            std::cmp::Ordering::Greater => 6 + score,
            std::cmp::Ordering::Equal => 3 + score,
            std::cmp::Ordering::Less => score,
        }
    }

    pub fn score_desired(&self) -> usize {
        let score = self.other.match_desire(&self.desire).score();
        match self.desire {
            Desire::Win => 6 + score,
            Desire::Draw => 3 + score,
            Desire::Lose => score,
        }
    }
}

impl FromStr for Round {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.trim().split(' ');
        let other = Choice::from_str(iter.next().ok_or_else(|| anyhow!("invalid input: {}", s))?)?;
        let second = iter.next().ok_or_else(|| anyhow!("invalid input: {}", s))?;
        let you = Choice::from_str(second)?;
        let desire = Desire::from_str(second)?;

        Ok(Self { other, you, desire })
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Desire {
    Win,
    Lose,
    Draw,
}

impl FromStr for Desire {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => bail!("Invalid input for desire: {}", s),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RockPaperScissors {
    rounds: Vec<Round>,
}

impl FromStr for RockPaperScissors {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rounds = s
            .trim()
            .lines()
            .map(|l| Round::from_str(l))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { rounds })
    }
}

impl Problem for RockPaperScissors {
    const DAY: usize = 2;
    const TITLE: &'static str = "rock paper scissors";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.rounds.iter().map(|r| r.score()).sum())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.rounds.iter().map(|r| r.score_desired()).sum())
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
        let solution = RockPaperScissors::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(10404, 10334));
    }

    #[test]
    fn example() {
        let input = "
            A Y
            B X
            C Z
            ";
        let solution = RockPaperScissors::solve(input).unwrap();
        assert_eq!(solution, Solution::new(15, 12));
    }
}
