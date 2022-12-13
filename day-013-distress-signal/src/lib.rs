use std::str::FromStr;

use aoc_plumbing::Problem;
use nom::{
    branch::alt,
    character::complete::{self, multispace0, newline, space0},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, preceded, separated_pair, tuple},
    IResult,
};
#[cfg(feature = "par")]
use rayon::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Value {
    Number(i64),
    List(Vec<Value>),
}

fn parse_number(input: &str) -> IResult<&str, Value> {
    let (input, v) = nom::character::complete::i64(input)?;
    Ok((input, Value::Number(v)))
}

fn parse_list(input: &str) -> IResult<&str, Value> {
    let (input, values) = delimited(
        complete::char('['),
        separated_list0(complete::char(','), alt((parse_number, parse_list))),
        complete::char(']'),
    )(input)?;

    Ok((input, Value::List(values)))
}

fn parse_value(input: &str) -> IResult<&str, Value> {
    alt((parse_number, parse_list))(input)
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            Self::Number(me) => match other {
                Self::Number(them) => me.cmp(them),
                Self::List(_) => Self::List(vec![self.clone()]).cmp(other),
            },
            Self::List(me) => match other {
                Self::Number(_) => self.cmp(&Self::List(vec![other.clone()])),
                // rust list ordering already implements the specified rules
                // from the problem
                Self::List(them) => me.cmp(them),
            },
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PacketPair {
    left: Value,
    right: Value,
}

impl PacketPair {
    pub fn in_order(&self) -> bool {
        self.left <= self.right
    }
}

fn parse_packet_pair(input: &str) -> IResult<&str, PacketPair> {
    let (input, (left, right)) = separated_pair(
        preceded(space0, parse_value),
        newline,
        preceded(space0, parse_value),
    )(input)?;
    Ok((input, PacketPair { left, right }))
}

#[allow(dead_code)]
fn parse_packet_pairs(input: &str) -> IResult<&str, Vec<PacketPair>> {
    preceded(
        multispace0,
        separated_list1(tuple((newline, newline)), parse_packet_pair),
    )(input)
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DistressSignal {
    packet_pairs: Vec<PacketPair>,
}

impl FromStr for DistressSignal {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        #[cfg(not(feature = "par"))]
        let (_, packet_pairs) = parse_packet_pairs(s).map_err(|e| e.to_owned())?;
        #[cfg(feature = "par")]
        // There's a limitation with par_split that it doesn't split on a full pattern
        let packet_pairs = s
            .trim()
            .replace("\n\n", ":")
            .par_split(':')
            .map(|g| parse_packet_pair(g).map(|(_, p)| p))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_owned())?;
        Ok(Self { packet_pairs })
    }
}

impl Problem for DistressSignal {
    const DAY: usize = 13;
    const TITLE: &'static str = "distress signal";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self
            .packet_pairs
            .iter()
            .enumerate()
            .filter(|(_, p)| p.in_order())
            .map(|(i, _)| i + 1)
            .sum())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        // We don't need to sort becuse we just care about the indicies of the
        // two divider packets, and we don't care where everything else is.
        //
        // Because we implemented Ord we _could_ sort, of course
        let mut div1_index = 1;
        let mut div2_index = 2; // this starts at two because div1 is smaller

        // let div1 = Value::List(vec![Value::Number(2)]);
        // let div2 = Value::List(vec![Value::Number(6)]);
        //
        // for some reason, this is _faster_ than constructing them directly
        let (_, div1) = parse_value("[[2]]").map_err(|e| e.to_owned())?;
        let (_, div2) = parse_value("[[6]]").map_err(|e| e.to_owned())?;

        for pair in self.packet_pairs.iter() {
            if div1 > pair.right {
                div1_index += 1;
            }

            if div2 > pair.right {
                div2_index += 1;
            }

            if div1 > pair.left {
                div1_index += 1;
            }

            if div2 > pair.left {
                div2_index += 1;
            }
        }

        Ok(div1_index * div2_index)
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
        let solution = DistressSignal::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(5684, 22932));
    }

    #[test]
    fn example() {
        let input = "[1,1,3,1,1]
            [1,1,5,1,1]

            [[1],[2,3,4]]
            [[1],4]

            [9]
            [[8,7,6]]

            [[4,4],4,4]
            [[4,4],4,4,4]

            [7,7,7,7]
            [7,7,7]

            []
            [3]

            [[[]]]
            [[]]

            [1,[2,[3,[4,[5,6,7]]]],8,9]
            [1,[2,[3,[4,[5,6,0]]]],8,9]
            ";
        let solution = DistressSignal::solve(input).unwrap();
        assert_eq!(solution, Solution::new(13, 140));
    }

    #[test]
    fn value_parsing() {
        let (_, _) = parse_value("[1,1,5,1,1]").unwrap();
        let (_, _) = parse_value("[1,[],5,1,1]").unwrap();
    }
}
