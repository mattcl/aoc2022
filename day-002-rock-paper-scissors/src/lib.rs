use std::str::FromStr;

use anyhow::{anyhow, bail};
use aoc_plumbing::Problem;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Outcome {
    Win,
    Lose,
    Draw,
}

impl FromStr for Outcome {
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

    pub fn match_desire(&self, desire: &Outcome) -> Self {
        match self {
            Self::Rock => match desire {
                Outcome::Win => Self::Paper,
                Outcome::Lose => Self::Scissors,
                Outcome::Draw => Self::Rock,
            },
            Self::Paper => match desire {
                Outcome::Win => Self::Scissors,
                Outcome::Lose => Self::Rock,
                Outcome::Draw => Self::Paper,
            },
            Self::Scissors => match desire {
                Outcome::Win => Self::Rock,
                Outcome::Lose => Self::Paper,
                Outcome::Draw => Self::Scissors,
            },
        }
    }

    pub fn evaluate(&self, other: &Self) -> Outcome {
        match self {
            Self::Rock => match other {
                Self::Rock => Outcome::Draw,
                Self::Paper => Outcome::Lose,
                Self::Scissors => Outcome::Win,
            },
            Self::Paper => match other {
                Self::Rock => Outcome::Win,
                Self::Paper => Outcome::Draw,
                Self::Scissors => Outcome::Lose,
            },
            Self::Scissors => match other {
                Self::Rock => Outcome::Lose,
                Self::Paper => Outcome::Win,
                Self::Scissors => Outcome::Draw,
            },
        }
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
    desire: Outcome,
}

impl Round {
    pub fn score(&self) -> usize {
        let score = self.you.score();

        match self.you.evaluate(&self.other) {
            Outcome::Win => 6 + score,
            Outcome::Draw => 3 + score,
            Outcome::Lose => score,
        }
    }

    pub fn score_desired(&self) -> usize {
        let score = self.other.match_desire(&self.desire).score();
        match self.desire {
            Outcome::Win => 6 + score,
            Outcome::Draw => 3 + score,
            Outcome::Lose => score,
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
        let desire = Outcome::from_str(second)?;

        Ok(Self { other, you, desire })
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

    #[test]
    fn choice_ordering() {
        assert_eq!(Choice::Rock.evaluate(&Choice::Rock), Outcome::Draw);
        assert_eq!(Choice::Rock.evaluate(&Choice::Paper), Outcome::Lose);
        assert_eq!(Choice::Rock.evaluate(&Choice::Scissors), Outcome::Win);

        assert_eq!(Choice::Paper.evaluate(&Choice::Rock), Outcome::Win);
        assert_eq!(Choice::Paper.evaluate(&Choice::Paper), Outcome::Draw);
        assert_eq!(Choice::Paper.evaluate(&Choice::Scissors), Outcome::Lose);

        assert_eq!(Choice::Scissors.evaluate(&Choice::Rock), Outcome::Lose);
        assert_eq!(Choice::Scissors.evaluate(&Choice::Paper), Outcome::Win);
        assert_eq!(Choice::Scissors.evaluate(&Choice::Scissors), Outcome::Draw);
    }

    #[test]
    fn desires() {
        assert_eq!(Choice::Rock.match_desire(&Outcome::Win), Choice::Paper);
        assert_eq!(Choice::Rock.match_desire(&Outcome::Draw), Choice::Rock);
        assert_eq!(Choice::Rock.match_desire(&Outcome::Lose), Choice::Scissors);

        assert_eq!(Choice::Paper.match_desire(&Outcome::Win), Choice::Scissors);
        assert_eq!(Choice::Paper.match_desire(&Outcome::Draw), Choice::Paper);
        assert_eq!(Choice::Paper.match_desire(&Outcome::Lose), Choice::Rock);

        assert_eq!(Choice::Scissors.match_desire(&Outcome::Win), Choice::Rock);
        assert_eq!(
            Choice::Scissors.match_desire(&Outcome::Draw),
            Choice::Scissors
        );
        assert_eq!(Choice::Scissors.match_desire(&Outcome::Lose), Choice::Paper);
    }
}
