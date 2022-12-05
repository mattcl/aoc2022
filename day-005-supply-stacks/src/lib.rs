use std::{collections::VecDeque, str::FromStr};

use anyhow::{anyhow, bail};
use aoc_plumbing::Problem;
use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, recognize},
    sequence::{preceded, tuple},
    AsChar, IResult,
};

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Column {
    crates: Vec<char>,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct Instruction {
    quantity: usize,
    start: usize,
    end: usize,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, inst) =
            parse_instruction(s.trim()).map_err(|_| anyhow!("Failed to parse: {}", s))?;
        Ok(inst)
    }
}

fn parse_num(input: &str) -> IResult<&str, usize> {
    map_res(recognize(digit1), usize::from_str)(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, (quantity, start, end)) = tuple((
        preceded(tag("move "), parse_num),
        preceded(tag(" from "), parse_num),
        preceded(tag(" to "), parse_num),
    ))(input)?;

    Ok((
        input,
        Instruction {
            quantity,
            start,
            end,
        },
    ))
}

// Use an intermediate object for indirection so I can clone this and not the
// problem
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Dock {
    columns: Vec<Column>,
}

impl Dock {
    pub fn carry_out(&mut self, instruction: &Instruction) -> Result<(), anyhow::Error> {
        if self.columns.len() < instruction.start || self.columns.len() < instruction.end {
            bail!("Invalid instruction: {:?}", instruction);
        }

        for _ in 0..instruction.quantity {
            let k = self.columns[instruction.start - 1]
                .crates
                .pop()
                .ok_or_else(|| anyhow!("attempted to remove from empty stack"))?;
            self.columns[instruction.end - 1].crates.push(k);
        }

        Ok(())
    }

    pub fn carry_out_advanced(&mut self, instruction: &Instruction) -> Result<(), anyhow::Error> {
        if self.columns.len() < instruction.start || self.columns.len() < instruction.end {
            bail!("Invalid instruction: {:?}", instruction);
        }

        let mut acc = VecDeque::with_capacity(instruction.quantity);
        for _ in 0..instruction.quantity {
            acc.push_front(
                self.columns[instruction.start - 1]
                    .crates
                    .pop()
                    .ok_or_else(|| anyhow!("attempted to remove from empty stack"))?,
            );
        }

        for k in acc {
            self.columns[instruction.end - 1].crates.push(k);
        }

        Ok(())
    }

    pub fn top_values(&self) -> String {
        self.columns
            .iter()
            .filter_map(|c| c.crates.last())
            .collect()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SupplyStacks {
    dock: Dock,
    instructions: Vec<Instruction>,
}

impl FromStr for SupplyStacks {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (picture, insts) = s
            .split_once("\n\n")
            .ok_or_else(|| anyhow!("Invalid input, no separating newline"))?;

        // let's get the last line of the picture and find every char index
        // corresponding to a numeric character
        let mut iter = picture.lines().rev();
        let index_line = iter
            .next()
            .ok_or_else(|| anyhow!("Invalid input missing index line"))?;

        let indicies: Vec<_> = index_line
            .chars()
            .enumerate()
            .filter(|(_, ch)| ch.is_digit(10))
            .collect();

        // Now, if we found more than 9, we have a problem because our strategy
        // relies on column alignment, so I'm going to bail here
        if indicies.len() > 9 {
            bail!("I am only allowing for up to 9 stacks")
        }

        // with the remaining lines, we're going to find every alpha char in a
        // column that matches an index we discovered
        let picture_lines: Vec<Vec<char>> = iter.map(|l| l.chars().collect::<Vec<_>>()).collect();

        if picture_lines.is_empty() {
            bail!("Empty picture");
        }

        let mut columns: Vec<_> = (0..indicies.len()).map(|_| Column::default()).collect();
        for (col, (idx, _)) in indicies.iter().enumerate() {
            for line_idx in 0..picture_lines.len() {
                // if we have uneven lines, the get will guard against that
                if let Some(v) = picture_lines[line_idx]
                    .get(*idx)
                    .filter(|v| v.is_alphanum())
                {
                    columns[col].crates.push(*v);
                }
            }
        }

        // for each additional line, parse as instructions
        let instructions = insts
            .trim()
            .lines()
            .map(Instruction::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            dock: Dock { columns },
            instructions,
        })
    }
}

impl Problem for SupplyStacks {
    const DAY: usize = 5;
    const TITLE: &'static str = "supply stacks";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = String;
    type P2 = String;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        // we need to clone here so we don't mess with part two (and the bench)
        let mut dock = self.dock.clone();
        for inst in self.instructions.iter() {
            dock.carry_out(inst)?;
        }

        Ok(dock.top_values())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        // we need to clone here so we don't mess with the bench
        let mut dock = self.dock.clone();
        for inst in self.instructions.iter() {
            dock.carry_out_advanced(inst)?;
        }

        Ok(dock.top_values())
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
        let solution = SupplyStacks::solve(&input).unwrap();
        assert_eq!(
            solution,
            Solution::new("VQZNJMWTR".into(), "NLCDCLVMQ".into())
        );
    }

    #[test]
    fn example() {
        let input = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let solution = SupplyStacks::solve(input).unwrap();
        assert_eq!(solution, Solution::new("CMZ".into(), "MCD".into()));
    }

    #[test]
    fn instruction_parsing() {
        let res = Instruction::from_str("move 10 from 2 to 999").unwrap();
        assert_eq!(
            res,
            Instruction {
                quantity: 10,
                start: 2,
                end: 999
            }
        );

        let res = Instruction::from_str("move 1 from 2 to 3").unwrap();
        assert_eq!(
            res,
            Instruction {
                quantity: 1,
                start: 2,
                end: 3
            }
        );
    }
}
