use std::{collections::VecDeque, str::FromStr};

use anyhow::anyhow;
use aoc_plumbing::Problem;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0, multispace1},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, tuple},
    IResult,
};
use rustc_hash::FxHashSet;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Operation {
    Add(u64),
    Mul(u64),
    Square,
    Double,
}

impl Operation {
    pub fn eval(&self, other: u64) -> u64 {
        match self {
            Self::Add(v) => other + v,
            Self::Mul(v) => other * v,
            Self::Double => other + other,
            Self::Square => other * other,
        }
    }
}

fn parse_add(input: &str) -> IResult<&str, Operation> {
    let (input, val) = preceded(tag("old + "), nom::character::complete::u64)(input)?;
    Ok((input, Operation::Add(val)))
}

fn parse_mul(input: &str) -> IResult<&str, Operation> {
    let (input, val) = preceded(tag("old * "), nom::character::complete::u64)(input)?;
    Ok((input, Operation::Mul(val)))
}

fn parse_double(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("old + old")(input)?;
    Ok((input, Operation::Double))
}

fn parse_square(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("old * old")(input)?;
    Ok((input, Operation::Square))
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    preceded(
        tag("Operation: new = "),
        alt((parse_add, parse_mul, parse_double, parse_square)),
    )(input)
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct Test {
    denominator: u64,
    target_true: usize,
    target_false: usize,
}

impl Test {
    pub fn eval(&self, item: u64) -> usize {
        if item % self.denominator == 0 {
            self.target_true
        } else {
            self.target_false
        }
    }
}

fn parse_test(input: &str) -> IResult<&str, Test> {
    let (input, denominator) =
        preceded(tag("Test: divisible by "), nom::character::complete::u64)(input)?;
    let (input, target_true) = preceded(
        tuple((multispace1, tag("If true: throw to monkey "))),
        nom::character::complete::u64,
    )(input)?;
    let (input, target_false) = preceded(
        tuple((multispace1, tag("If false: throw to monkey "))),
        nom::character::complete::u64,
    )(input)?;
    Ok((
        input,
        Test {
            denominator,
            target_true: target_true as usize,
            target_false: target_false as usize,
        },
    ))
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Monkey {
    id: usize,
    items_inspected: usize,
    items: VecDeque<u64>,
    operation: Operation,
    test: Test,
}

impl Monkey {
    pub fn inspect(&self, item: u64) -> u64 {
        self.operation.eval(item)
    }

    pub fn target(&self, item: u64) -> usize {
        self.test.eval(item)
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn throw_item(
        &mut self,
        adjustment: impl Fn(u64) -> u64,
    ) -> Result<(usize, u64), anyhow::Error> {
        let mut worry = self
            .items
            .pop_front()
            .ok_or_else(|| anyhow!("Attempted to throw from empty monkey: {}", self.id))?;

        worry = adjustment(self.inspect(worry));
        self.items_inspected += 1;

        let target = self.target(worry);

        Ok((target, worry))
    }

    pub fn receive_item(&mut self, item: u64) {
        self.items.push_back(item);
    }
}

fn parse_items(input: &str) -> IResult<&str, VecDeque<u64>> {
    let (input, items) = preceded(
        tag("Starting items: "),
        separated_list1(tag(", "), nom::character::complete::u64),
    )(input)?;
    Ok((input, VecDeque::from(items)))
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, id) = delimited(
        tuple((multispace0, tag("Monkey "))),
        nom::character::complete::u64,
        tag(":"),
    )(input)?;
    let (input, items) = preceded(multispace1, parse_items)(input)?;
    let (input, operation) = preceded(multispace1, parse_operation)(input)?;
    let (input, test) = preceded(multispace1, parse_test)(input)?;

    Ok((
        input,
        Monkey {
            id: id as usize,
            items_inspected: 0,
            items,
            operation,
            test,
        },
    ))
}

fn parse_monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    preceded(multispace0, many1(parse_monkey))(input)
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MonkeyInTheMiddle {
    monkeys: Vec<Monkey>,
}

impl MonkeyInTheMiddle {
    pub fn round(&mut self, adjustment: impl Fn(u64) -> u64) -> Result<(), anyhow::Error> {
        for i in 0..self.monkeys.len() {
            while !self.monkeys[i].is_empty() {
                let (target, item) = self.monkeys[i].throw_item(&adjustment)?;
                self.monkeys[target].receive_item(item);
            }
        }

        Ok(())
    }
}

impl FromStr for MonkeyInTheMiddle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, monkeys) = parse_monkeys(s).map_err(|e| e.to_owned())?;
        Ok(Self { monkeys })
    }
}

impl Problem for MonkeyInTheMiddle {
    const DAY: usize = 11;
    const TITLE: &'static str = "monkey in the middle";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        let mut working = self.clone();
        for _ in 0..20 {
            working.round(|x| x / 3)?;
        }
        let mut inspected = working
            .monkeys
            .iter()
            .map(|m| m.items_inspected)
            .collect::<Vec<_>>();
        inspected.sort();
        Ok(inspected.pop().unwrap_or(0) * inspected.pop().unwrap_or(0))
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        let unique: FxHashSet<u64> = self.monkeys.iter().map(|m| m.test.denominator).collect();

        let divisor: u64 = unique.iter().product();

        let mut working = self.clone();
        for _ in 0..10_000 {
            working.round(|x| x % divisor)?;
        }
        let mut inspected = working
            .monkeys
            .iter()
            .map(|m| m.items_inspected)
            .collect::<Vec<_>>();
        inspected.sort();
        Ok(inspected.pop().unwrap_or(0) * inspected.pop().unwrap_or(0))
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
        let solution = MonkeyInTheMiddle::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(95472, 17926061332));
    }

    #[test]
    fn example() {
        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
        let solution = MonkeyInTheMiddle::solve(input).unwrap();
        assert_eq!(solution, Solution::new(10605, 2713310158));
    }
}
