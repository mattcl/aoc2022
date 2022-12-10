use std::str::FromStr;

use aoc_plumbing::Problem;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::multispace0, multi::many1,
    sequence::preceded, IResult,
};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Opcode {
    Addx { remaining: u8, val: i64 },
    NoOp { remaining: u8 },
}

impl Opcode {
    pub fn num_cycles(&self) -> i64 {
        match self {
            Self::Addx { .. } => 2,
            Self::NoOp { .. } => 1,
        }
    }

    pub fn done(&mut self) -> bool {
        match self {
            Self::Addx { remaining, .. } | Self::NoOp { remaining } => {
                *remaining -= 1;
                *remaining <= 0
            }
        }
    }
}

pub fn parse_addx(input: &str) -> IResult<&str, Opcode> {
    let (input, val) = preceded(tag("addx "), nom::character::complete::i64)(input)?;
    Ok((input, Opcode::Addx { remaining: 2, val }))
}

pub fn parse_noop(input: &str) -> IResult<&str, Opcode> {
    let (input, _) = tag("noop")(input)?;
    Ok((input, Opcode::NoOp { remaining: 1 }))
}

pub fn parse_opcode(input: &str) -> IResult<&str, Opcode> {
    alt((parse_addx, parse_noop))(input)
}

pub fn parse_opcodes(input: &str) -> IResult<&str, Vec<Opcode>> {
    many1(preceded(multispace0, parse_opcode))(input)
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CathodeRayTube {
    operations: Vec<Opcode>,
}

impl FromStr for CathodeRayTube {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, operations) = parse_opcodes(s).map_err(|e| e.to_owned())?;
        Ok(Self { operations })
    }
}

impl Problem for CathodeRayTube {
    const DAY: usize = 10;
    const TITLE: &'static str = "cathode ray tube";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = i64;
    type P2 = String;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        let mut cycle = 1;
        let mut breakpoint = 20;
        let mut register = 1_i64;
        let mut last_register = 1_i64;
        let mut out = 0;

        for op in self.operations.iter() {
            cycle += op.num_cycles();

            if let Opcode::Addx { val, .. } = op {
                last_register = register;
                register += val;
            }

            if cycle >= breakpoint {
                if cycle == breakpoint {
                    out += register * breakpoint;
                } else {
                    out += last_register * breakpoint;
                }
                breakpoint += 40;
            }
        }
        Ok(out)
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        let mut pixels = String::with_capacity(240 + 6);
        let mut program_counter = 0;
        let mut op = self.operations[0];
        let mut register = 1_i64;

        for pixel in 0..240_i64 {
            if pixel % 40 == 0 {
                // this results in a leading newline, which I actually want
                // because of the way I print the output with a leading 'part 2:'
                pixels.push('\n');
            }
            let pos = pixel % 40;
            if (register - pos).abs() <= 1 {
                pixels.push('#');
            } else {
                pixels.push('.');
            }

            if op.done() {
                if let Opcode::Addx { val, .. } = op {
                    register += val;
                }

                program_counter += 1;
                if program_counter < self.operations.len() {
                    op = self.operations[program_counter];
                }
            }
        }

        Ok(pixels)
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
        let solution = CathodeRayTube::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(11720, "\n####.###...##..###..####.###...##....##.\n#....#..#.#..#.#..#.#....#..#.#..#....#.\n###..#..#.#....#..#.###..#..#.#.......#.\n#....###..#....###..#....###..#.......#.\n#....#.#..#..#.#.#..#....#....#..#.#..#.\n####.#..#..##..#..#.####.#.....##...##..".into()));
    }

    #[test]
    fn example() {
        let input = "
            addx 15
            addx -11
            addx 6
            addx -3
            addx 5
            addx -1
            addx -8
            addx 13
            addx 4
            noop
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx -35
            addx 1
            addx 24
            addx -19
            addx 1
            addx 16
            addx -11
            noop
            noop
            addx 21
            addx -15
            noop
            noop
            addx -3
            addx 9
            addx 1
            addx -3
            addx 8
            addx 1
            addx 5
            noop
            noop
            noop
            noop
            noop
            addx -36
            noop
            addx 1
            addx 7
            noop
            noop
            noop
            addx 2
            addx 6
            noop
            noop
            noop
            noop
            noop
            addx 1
            noop
            noop
            addx 7
            addx 1
            noop
            addx -13
            addx 13
            addx 7
            noop
            addx 1
            addx -33
            noop
            noop
            noop
            addx 2
            noop
            noop
            noop
            addx 8
            noop
            addx -1
            addx 2
            addx 1
            noop
            addx 17
            addx -9
            addx 1
            addx 1
            addx -3
            addx 11
            noop
            noop
            addx 1
            noop
            addx 1
            noop
            noop
            addx -13
            addx -19
            addx 1
            addx 3
            addx 26
            addx -30
            addx 12
            addx -1
            addx 3
            addx 1
            noop
            noop
            noop
            addx -9
            addx 18
            addx 1
            addx 2
            noop
            noop
            addx 9
            noop
            noop
            noop
            addx -1
            addx 2
            addx -37
            addx 1
            addx 3
            noop
            addx 15
            addx -21
            addx 22
            addx -6
            addx 1
            noop
            addx 2
            addx 1
            noop
            addx -10
            noop
            noop
            addx 20
            addx 1
            addx 2
            addx 2
            addx -6
            addx -11
            noop
            noop
            noop
            ";
        let solution = CathodeRayTube::solve(input).unwrap();
        assert_eq!(solution, Solution::new(13140, "\n##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....".into()));
    }
}
