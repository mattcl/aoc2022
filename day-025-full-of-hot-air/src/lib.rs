use std::{collections::VecDeque, fmt::Display, str::FromStr};

use anyhow::bail;
use aoc_plumbing::Problem;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Digit {
    Zero,
    One,
    Two,
    Minus,
    DoubleMinus,
}

impl Digit {
    pub fn val(&self) -> i64 {
        match self {
            Self::Zero => 0,
            Self::One => 1,
            Self::Two => 2,
            Self::Minus => -1,
            Self::DoubleMinus => -2,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Self::Zero => '0',
            Self::One => '1',
            Self::Two => '2',
            Self::Minus => '-',
            Self::DoubleMinus => '=',
        }
    }
}

impl TryFrom<char> for Digit {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '0' => Ok(Self::Zero),
            '1' => Ok(Self::One),
            '2' => Ok(Self::Two),
            '-' => Ok(Self::Minus),
            '=' => Ok(Self::DoubleMinus),
            _ => bail!("invalid digit char: {}", value),
        }
    }
}

const BASE: i64 = 5;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct State {
    snafu: Snafu,
    value: i64,
    len: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .len
            .cmp(&self.len)
            .then_with(|| self.value.cmp(&other.value))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Snafu {
    digits: VecDeque<Digit>,
}

impl Snafu {
    fn to_decimal(&self) -> i64 {
        let mut sum = 0;

        for (idx, digit) in self.digits.iter().rev().enumerate() {
            sum += BASE.pow(idx as u32) * digit.val();
        }

        sum
    }
}

impl From<i64> for Snafu {
    fn from(value: i64) -> Self {
        let mut snafu = Snafu::default();

        let mut working = value;
        while working > 0 {
            let digit = match working % 5 {
                0 => Digit::Zero,
                1 => Digit::One,
                2 => Digit::Two,
                3 => Digit::DoubleMinus,
                4 => Digit::Minus,
                _ => unreachable!(),
            };

            snafu.digits.push_front(digit);
            working -= digit.val();
            working /= 5;
        }

        snafu
    }
}

impl Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self.digits.iter().map(|d| d.to_char()).collect();
        s.fmt(f)
    }
}

impl FromStr for Snafu {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let digits = s
            .chars()
            .map(|ch| Digit::try_from(ch))
            .collect::<Result<VecDeque<_>, _>>()?;
        Ok(Self { digits })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FullOfHotAir {
    numbers: Vec<Snafu>,
}

impl FromStr for FullOfHotAir {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s
            .trim()
            .lines()
            .map(|l| Snafu::from_str(l.trim()))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { numbers })
    }
}

impl Problem for FullOfHotAir {
    const DAY: usize = 25;
    const TITLE: &'static str = "full of hot air";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = String;
    type P2 = i64;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        let sum: i64 = self.numbers.iter().map(|n| n.to_decimal()).sum();
        Ok(Snafu::from(sum).to_string())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        // no part two on day 25
        Ok(0)
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
        let solution = FullOfHotAir::solve(&input).unwrap();
        assert_eq!(solution, Solution::new("2=112--220-=-00=-=20".into(), 0));
    }

    #[test]
    fn example() {
        let input = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";
        let solution = FullOfHotAir::solve(input).unwrap();
        assert_eq!(solution, Solution::new("2=-1=0".into(), 0));
    }
}
