use std::str::FromStr;

use anyhow::anyhow;
use aoc_plumbing::Problem;
use nom::{character::complete::newline, multi::separated_list1, IResult};

pub const DECRYPTION_KEY: i64 = 811589153;

fn parse_numbers(input: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(newline, nom::character::complete::i64)(input)
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GrovePositioningSystem {
    numbers: Vec<i64>,
}

impl GrovePositioningSystem {
    pub fn mix(&self, iterations: usize, decryption_key: i64) -> Result<i64, anyhow::Error> {
        let len = self.numbers.len() as i64;
        let mut working: Vec<_> = self
            .numbers
            .iter()
            .enumerate()
            .map(|(idx, v)| {
                let res = *v * decryption_key;
                (idx as i64, res)
            })
            .collect();

        for _ in 0..iterations {
            for i in 0..len {
                let pos = working
                    .iter()
                    .position(|(idx, _)| *idx == i)
                    .ok_or_else(|| anyhow!("lost a value"))?;

                // this isn't much, but let's not manipulate the list here
                if working[pos].1 == 0 {
                    continue;
                }

                let old = working.remove(pos);
                let target = (pos as i64 + old.1).rem_euclid(len - 1);

                // this branch never executes, but it gains me 8% performance
                // for some dumb resaon so it's staying
                if target == len - 1 {
                    working.push(old);
                } else {
                    let idx = (target % (len - 1)) as usize;
                    working.insert(idx, old);
                }
            }
        }

        let mut zero = 0;
        for i in 0..working.len() {
            if working[i].1 == 0 {
                zero = i as i64;
                break;
            }
        }

        let one = working[((zero + 1000) % len) as usize].1;
        let two = working[((zero + 2000) % len) as usize].1;
        let three = working[((zero + 3000) % len) as usize].1;

        Ok(one + two + three)
    }
}

impl FromStr for GrovePositioningSystem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, numbers) = parse_numbers(s.trim()).map_err(|e| e.to_owned())?;
        Ok(Self { numbers })
    }
}

impl Problem for GrovePositioningSystem {
    const DAY: usize = 20;
    const TITLE: &'static str = "grove positioning system";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = i64;
    type P2 = i64;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        self.mix(1, 1)
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        self.mix(10, DECRYPTION_KEY)
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
        let solution = GrovePositioningSystem::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(13967, 1790365671518));
    }

    #[test]
    fn example() {
        let input = "1
2
-3
3
-2
0
4";
        let solution = GrovePositioningSystem::solve(input).unwrap();
        assert_eq!(solution, Solution::new(3, 1623178306));
    }
}
