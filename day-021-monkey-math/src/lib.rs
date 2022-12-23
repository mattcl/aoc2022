//! For this solution, we are making the assumption that the unknown value never
//! shows up on both sides of the root expression, as that would make it much
//! more difficult to solve. I verified with my input that this assumption holds
//! and I'm going to assume it has to hold for all inputs (or it may have ended
//! up being wildly unfair).
use std::{
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};

use anyhow::{anyhow, bail};
use aoc_plumbing::Problem;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, i64 as nom_i64, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use rustc_hash::FxHashMap;

/// Used when using the `Value` representation to model expressions.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Op {
    Sum { left: Value, right: Value },
    Sub { left: Value, right: Value },
    Mul { left: Value, right: Value },
    Div { left: Value, right: Value },
}

impl Op {
    pub fn undo(&self, target: i64) -> Result<i64, anyhow::Error> {
        match self {
            Self::Sum { left, right } => match (left, right) {
                (Value::Var, Value::Num { value }) => Ok(target - value),
                (Value::Num { value }, Value::Var) => Ok(target - value),
                (Value::Expr { op }, Value::Num { value }) => op.undo(target - value),
                (Value::Num { value }, Value::Expr { op }) => op.undo(target - value),
                _ => bail!("Invalid undo operation {:?}", &self),
            },
            Self::Sub { left, right } => match (left, right) {
                (Value::Var, Value::Num { value }) => Ok(target + value),
                (Value::Num { value }, Value::Var) => Ok(value - target),
                (Value::Expr { op }, Value::Num { value }) => op.undo(target + value),
                (Value::Num { value }, Value::Expr { op }) => op.undo(value - target),
                _ => bail!("Invalid undo operation {:?}", &self),
            },
            Self::Mul { left, right } => match (left, right) {
                (Value::Var, Value::Num { value }) => Ok(target / value),
                (Value::Num { value }, Value::Var) => Ok(value / target),
                (Value::Expr { op }, Value::Num { value }) => op.undo(target / value),
                (Value::Num { value }, Value::Expr { op }) => op.undo(target / value),
                _ => bail!("Invalid undo operation {:?}", &self),
            },
            Self::Div { left, right } => match (left, right) {
                (Value::Var, Value::Num { value }) => Ok(target * value),
                (Value::Num { value }, Value::Var) => Ok(value / target),
                (Value::Expr { op }, Value::Num { value }) => op.undo(target * value),
                (Value::Num { value }, Value::Expr { op }) => op.undo(value / target),
                _ => bail!("Invalid undo operation {:?}", &self),
            },
        }
    }
}

/// Used only for part two to allow solving for a single variable.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Value {
    Num { value: i64 },
    Expr { op: Box<Op> },
    Var,
}

impl Value {
    pub fn get_value(&self) -> Result<i64, anyhow::Error> {
        match self {
            Self::Num { value } => Ok(*value),
            _ => bail!("help"),
        }
    }

    pub fn solve(&self, target: i64) -> Result<i64, anyhow::Error> {
        match self {
            Self::Expr { op } => op.undo(target),
            _ => bail!("Cannot call solve on anything but an expression"),
        }
    }
}

impl Add<Value> for Value {
    type Output = Value;

    fn add(self, rhs: Value) -> Self::Output {
        match (&self, &rhs) {
            (Value::Num { value: left }, Value::Num { value: right }) => Value::Num {
                value: left + right,
            },
            (Value::Expr { .. }, Value::Var)
            | (Value::Expr { .. }, Value::Expr { .. })
            | (Value::Var, Value::Var)
            | (Value::Var, Value::Expr { .. }) => {
                unreachable!("Found expression and or var on two sides of an operation")
            }
            _ => Value::Expr {
                op: Box::new(Op::Sum {
                    left: self,
                    right: rhs,
                }),
            },
        }
    }
}

impl Sub<Value> for Value {
    type Output = Value;

    fn sub(self, rhs: Value) -> Self::Output {
        match (&self, &rhs) {
            (Value::Num { value: left }, Value::Num { value: right }) => Value::Num {
                value: left - right,
            },
            (Value::Expr { .. }, Value::Var)
            | (Value::Expr { .. }, Value::Expr { .. })
            | (Value::Var, Value::Var)
            | (Value::Var, Value::Expr { .. }) => {
                unreachable!("Found expression and or var on two sides of an operation")
            }
            _ => Value::Expr {
                op: Box::new(Op::Sub {
                    left: self,
                    right: rhs,
                }),
            },
        }
    }
}

impl Mul<Value> for Value {
    type Output = Value;

    fn mul(self, rhs: Value) -> Self::Output {
        match (&self, &rhs) {
            (Value::Num { value: left }, Value::Num { value: right }) => Value::Num {
                value: left * right,
            },
            (Value::Expr { .. }, Value::Var)
            | (Value::Expr { .. }, Value::Expr { .. })
            | (Value::Var, Value::Var)
            | (Value::Var, Value::Expr { .. }) => {
                unreachable!("Found expression and or var on two sides of an operation")
            }
            _ => Value::Expr {
                op: Box::new(Op::Mul {
                    left: self,
                    right: rhs,
                }),
            },
        }
    }
}

impl Div<Value> for Value {
    type Output = Value;

    fn div(self, rhs: Value) -> Self::Output {
        match (&self, &rhs) {
            (Value::Num { value: left }, Value::Num { value: right }) => Value::Num {
                value: left / right,
            },
            (Value::Expr { .. }, Value::Var)
            | (Value::Expr { .. }, Value::Expr { .. })
            | (Value::Var, Value::Var)
            | (Value::Var, Value::Expr { .. }) => {
                unreachable!("Found expression and or var on two sides of an operation")
            }
            _ => Value::Expr {
                op: Box::new(Op::Div {
                    left: self,
                    right: rhs,
                }),
            },
        }
    }
}

// So having these "raw" versions works around the limitation in the Problem
// trait not having lifetime support, by first storing &str then converting
// later. We're going to convert to int indicies of course, since that'll cut
// down on potential hash lookup time.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RawJob<'a> {
    Sum { left: &'a str, right: &'a str },
    Sub { left: &'a str, right: &'a str },
    Mul { left: &'a str, right: &'a str },
    Div { left: &'a str, right: &'a str },
    Yell { value: i64 },
}

impl<'a> RawJob<'a> {
    pub fn to_job(&self, name_hash: &FxHashMap<&str, usize>) -> Result<Job, anyhow::Error> {
        let j = match self {
            Self::Sum { left, right } => Job::Sum {
                left: *name_hash
                    .get(left)
                    .ok_or_else(|| anyhow!("missing monkey: {}", left))?,
                right: *name_hash
                    .get(right)
                    .ok_or_else(|| anyhow!("missing monkey: {}", right))?,
            },
            Self::Sub { left, right } => Job::Sub {
                left: *name_hash
                    .get(left)
                    .ok_or_else(|| anyhow!("missing monkey: {}", left))?,
                right: *name_hash
                    .get(right)
                    .ok_or_else(|| anyhow!("missing monkey: {}", right))?,
            },
            Self::Mul { left, right } => Job::Mul {
                left: *name_hash
                    .get(left)
                    .ok_or_else(|| anyhow!("missing monkey: {}", left))?,
                right: *name_hash
                    .get(right)
                    .ok_or_else(|| anyhow!("missing monkey: {}", right))?,
            },
            Self::Div { left, right } => Job::Div {
                left: *name_hash
                    .get(left)
                    .ok_or_else(|| anyhow!("missing monkey: {}", left))?,
                right: *name_hash
                    .get(right)
                    .ok_or_else(|| anyhow!("missing monkey: {}", right))?,
            },
            Self::Yell { value } => Job::Yell { value: *value },
        };

        Ok(j)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Job {
    Sum { left: usize, right: usize },
    Sub { left: usize, right: usize },
    Mul { left: usize, right: usize },
    Div { left: usize, right: usize },
    Yell { value: i64 },
    Human,
}

impl Job {
    /// So this is a direct solve without the extra `Value` overhead
    pub fn output(&self, monkeys: &[Monkey]) -> Result<i64, anyhow::Error> {
        match self {
            Self::Sum { left, right } => {
                let l = monkeys
                    .get(*left)
                    .ok_or_else(|| anyhow!("Unknown monkey: {}", left))?;
                let r = monkeys
                    .get(*right)
                    .ok_or_else(|| anyhow!("Unknown monkey: {}", left))?;
                Ok(l.output(monkeys)? + r.output(monkeys)?)
            }
            Self::Sub { left, right } => {
                let l = monkeys
                    .get(*left)
                    .ok_or_else(|| anyhow!("Unknown monkey: {}", left))?;
                let r = monkeys
                    .get(*right)
                    .ok_or_else(|| anyhow!("Unknown monkey: {}", left))?;
                Ok(l.output(monkeys)? - r.output(monkeys)?)
            }
            Self::Mul { left, right } => {
                let l = monkeys
                    .get(*left)
                    .ok_or_else(|| anyhow!("Unknown monkey: {}", left))?;
                let r = monkeys
                    .get(*right)
                    .ok_or_else(|| anyhow!("Unknown monkey: {}", left))?;
                Ok(l.output(monkeys)? * r.output(monkeys)?)
            }
            Self::Div { left, right } => {
                let l = monkeys
                    .get(*left)
                    .ok_or_else(|| anyhow!("Unknown monkey: {}", left))?;
                let r = monkeys
                    .get(*right)
                    .ok_or_else(|| anyhow!("Unknown monkey: {}", left))?;
                Ok(l.output(monkeys)? / r.output(monkeys)?)
            }
            Self::Yell { value } => Ok(*value),
            Self::Human => bail!("Cannot solve with human unless using `value_output`"),
        }
    }

    /// This pays the `Value` penalty to allow the solver to work.
    pub fn value_output(&self, monkeys: &[Monkey]) -> Result<Value, anyhow::Error> {
        match self {
            Self::Sum { left, right } => {
                let l = monkeys
                    .get(*left)
                    .ok_or_else(|| anyhow!("Unknown monkey: {}", left))?;
                let r = monkeys
                    .get(*right)
                    .ok_or_else(|| anyhow!("Unknown monkey: {}", left))?;
                Ok(l.value_output(monkeys)? + r.value_output(monkeys)?)
            }
            Self::Sub { left, right } => {
                let l = monkeys
                    .get(*left)
                    .ok_or_else(|| anyhow!("Unknown monkey: {}", left))?;
                let r = monkeys
                    .get(*right)
                    .ok_or_else(|| anyhow!("Unknown monkey: {}", left))?;
                Ok(l.value_output(monkeys)? - r.value_output(monkeys)?)
            }
            Self::Mul { left, right } => {
                let l = monkeys
                    .get(*left)
                    .ok_or_else(|| anyhow!("Unknown monkey: {}", left))?;
                let r = monkeys
                    .get(*right)
                    .ok_or_else(|| anyhow!("Unknown monkey: {}", left))?;
                Ok(l.value_output(monkeys)? * r.value_output(monkeys)?)
            }
            Self::Div { left, right } => {
                let l = monkeys
                    .get(*left)
                    .ok_or_else(|| anyhow!("Unknown monkey: {}", left))?;
                let r = monkeys
                    .get(*right)
                    .ok_or_else(|| anyhow!("Unknown monkey: {}", left))?;
                Ok(l.value_output(monkeys)? / r.value_output(monkeys)?)
            }
            Self::Yell { value } => Ok(Value::Num { value: *value }),
            Self::Human => Ok(Value::Var),
        }
    }
}

fn parse_sum<'a>(input: &'a str) -> IResult<&'a str, RawJob<'a>> {
    let (input, (left, right)) = separated_pair(alpha1, tag(" + "), alpha1)(input)?;
    Ok((input, RawJob::Sum { left, right }))
}

fn parse_sub<'a>(input: &'a str) -> IResult<&'a str, RawJob<'a>> {
    let (input, (left, right)) = separated_pair(alpha1, tag(" - "), alpha1)(input)?;
    Ok((input, RawJob::Sub { left, right }))
}

fn parse_mul<'a>(input: &'a str) -> IResult<&'a str, RawJob<'a>> {
    let (input, (left, right)) = separated_pair(alpha1, tag(" * "), alpha1)(input)?;
    Ok((input, RawJob::Mul { left, right }))
}

fn parse_div<'a>(input: &'a str) -> IResult<&'a str, RawJob<'a>> {
    let (input, (left, right)) = separated_pair(alpha1, tag(" / "), alpha1)(input)?;
    Ok((input, RawJob::Div { left, right }))
}

fn parse_yell<'a>(input: &'a str) -> IResult<&'a str, RawJob<'a>> {
    let (input, value) = nom_i64(input)?;
    Ok((input, RawJob::Yell { value }))
}

fn parse_job<'a>(input: &'a str) -> IResult<&'a str, RawJob<'a>> {
    alt((parse_sum, parse_sub, parse_mul, parse_div, parse_yell))(input)
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct RawMonkey<'a> {
    name: &'a str,
    job: RawJob<'a>,
}

fn parse_monkey<'a>(input: &'a str) -> IResult<&'a str, RawMonkey<'a>> {
    let (input, (name, job)) = separated_pair(alpha1, tag(": "), parse_job)(input)?;
    Ok((input, RawMonkey { name, job }))
}

fn parse_monkeys<'a>(input: &'a str) -> IResult<&'a str, Vec<RawMonkey<'a>>> {
    separated_list1(newline, parse_monkey)(input)
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Monkey {
    id: usize,
    job: Job,
}

impl Monkey {
    // So these recursive functions did have caches, before, but it turns out
    // that my input never had cache hits. The caches were removed for
    // performance reasons
    pub fn output(&self, monkeys: &[Monkey]) -> Result<i64, anyhow::Error> {
        self.job.output(monkeys)
    }

    pub fn value_output(&self, monkeys: &[Monkey]) -> Result<Value, anyhow::Error> {
        self.job.value_output(monkeys)
    }

    pub fn left_and_right<'a>(
        &self,
        monkeys: &'a [Monkey],
    ) -> Result<(&'a Monkey, &'a Monkey), anyhow::Error> {
        match &self.job {
            Job::Yell { .. } | Job::Human => bail!("cannot get left and right on Yell"),
            Job::Sum { left, right }
            | Job::Sub { left, right }
            | Job::Mul { left, right }
            | Job::Div { left, right } => {
                let l = monkeys
                    .get(*left)
                    .ok_or_else(|| anyhow!("unknown monkey: {}", left))?;
                let r = monkeys
                    .get(*right)
                    .ok_or_else(|| anyhow!("unknown monkey: {}", right))?;
                Ok((l, r))
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MonkeyMath {
    monkeys: Vec<Monkey>,
    root_id: usize,
    human_id: usize,
}

impl FromStr for MonkeyMath {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, raw_monkeys) = parse_monkeys(s).map_err(|e| e.to_owned())?;

        // this seems like some nonsense, but it's a huge later on savings to
        // not have to deal with strings and looking those up from hashes
        let mut monkeys = Vec::with_capacity(raw_monkeys.len());
        let mut monkey_name_hash: FxHashMap<&str, usize> =
            FxHashMap::with_capacity_and_hasher(raw_monkeys.len(), Default::default());

        let mut count = 0;
        for m in raw_monkeys.iter() {
            monkey_name_hash.insert(m.name, count);
            count += 1;
        }

        let root_id = *monkey_name_hash
            .get("root")
            .ok_or_else(|| anyhow!("no root monkey"))?;
        let human_id = *monkey_name_hash
            .get("humn")
            .ok_or_else(|| anyhow!("no human"))?;

        for m in raw_monkeys {
            let monkey = Monkey {
                id: monkeys.len(),
                job: m.job.to_job(&monkey_name_hash)?,
            };
            monkeys.push(monkey);
        }

        Ok(Self {
            monkeys,
            root_id,
            human_id,
        })
    }
}

impl Problem for MonkeyMath {
    const DAY: usize = 21;
    const TITLE: &'static str = "monkey math";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = i64;
    type P2 = i64;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        let root = self
            .monkeys
            .get(self.root_id)
            .ok_or_else(|| anyhow!("no monkey named root"))?;
        root.output(&self.monkeys)
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        self.monkeys[self.human_id].job = Job::Human;
        let root = self
            .monkeys
            .get(self.root_id)
            .ok_or_else(|| anyhow!("no monkey named root"))?;

        let (left, right) = root.left_and_right(&self.monkeys)?;

        // we can compute each side independently without implementing == for
        // root
        let l = left.value_output(&self.monkeys)?;
        let r = right.value_output(&self.monkeys)?;

        // figure out which side is us and the actual value of the other side
        let (us, them) = {
            if let Value::Num { value } = l {
                (r, value)
            } else if let Value::Num { value } = r {
                (l, value)
            } else {
                unreachable!("we should not have had two values")
            }
        };

        // then just figure out what we needed to be
        us.solve(them)
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
        let solution = MonkeyMath::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(49288254556480, 3558714869436));
    }

    #[test]
    fn example() {
        let input = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";
        let solution = MonkeyMath::solve(input).unwrap();
        assert_eq!(solution, Solution::new(152, 301));
    }
}
