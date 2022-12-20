use std::str::FromStr;

use anyhow::anyhow;
use aoc_plumbing::Problem;
use nom::{character::complete::newline, multi::separated_list1, IResult};

pub const DECRYPTION_KEY: i128 = 811589153;

fn parse_numbers(input: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(newline, nom::character::complete::i64)(input)
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GrovePositioningSystem {
    numbers: Vec<i64>,
}

impl GrovePositioningSystem {
    pub fn mix(&self, iterations: usize, decryption_key: i128) -> Result<i128, anyhow::Error> {
        let len = self.numbers.len() as i128;
        let mut min = i128::MAX;
        let mut working: Vec<_> = self
            .numbers
            .iter()
            .enumerate()
            .map(|(idx, v)| {
                let res = *v as i128 * decryption_key;
                if res < min {
                    min = res;
                }
                (idx as i128, res)
            })
            .collect();

        // we need to make sure we don't get negative indicies, so determine
        // how many additional len - 1 we have to add to ensure we can never be
        // negative. We're using len - 1 because that'll be the length of the
        // adjusted list while calculating indicies.
        let factor = min.abs() / (len - 1) + 1;

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
                let target = pos as i128 + old.1 + (len - 1) * factor;
                // we can never be zero
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
                zero = i as i128;
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
    type P1 = i128;
    type P2 = i128;

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
