//! For this solution, we are making the assumption that the unknown value never
//! shows up on both sides of the root expression, as that would make it much
//! more difficult to solve. I verified with my input that this assumption holds
//! and I'm going to assume it has to hold for all inputs (or it may have ended
//! up being wildly unfair).
use std::{str::FromStr, ops::{Add, Sub, Mul, Div}};

use anyhow::{anyhow, bail};
use aoc_plumbing::Problem;
use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::{i64 as nom_i64, alpha1, newline},
    sequence::separated_pair, multi::separated_list1
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
            Self::Sum { left, right } => {
                match left {
                    Value::Var => {
                        match right {
                            Value::Num { value } => Ok(target - value),
                            _ => bail!("Invalid undo operation: {:?}", &self)
                        }
                    }
                    Value::Expr { op } => {
                        match right {
                            Value::Num { value } => op.undo(target - value),
                            _ => bail!("Invalid undo operation: {:?}", &self)
                        }
                    }
                    Value::Num { value } => {
                        match right {
                            Value::Var => Ok(target - value),
                            Value::Expr { op } => op.undo(target - value),
                            _ => bail!("Invalid undo operation: {:?}", &self)
                        }
                    }
                }
            }
            Self::Sub { left, right } => {
                match left {
                    Value::Var => {
                        match right {
                            Value::Num { value } => Ok(target + value),
                            _ => bail!("Invalid undo operation: {:?}", &self)
                        }
                    }
                    Value::Expr { op } => {
                        match right {
                            Value::Num { value } => op.undo(target + value),
                            _ => bail!("Invalid undo operation: {:?}", &self)
                        }
                    }
                    Value::Num { value } => {
                        match right {
                            Value::Var => Ok(value - target),
                            Value::Expr { op } => op.undo(value - target),
                            _ => bail!("Invalid undo operation: {:?}", &self)
                        }
                    }
                }
            }
            Self::Mul { left, right } => {
                match left {
                    Value::Var => {
                        match right {
                            Value::Num { value } => Ok(target / value),
                            _ => bail!("Invalid undo operation: {:?}", &self)
                        }
                    }
                    Value::Expr { op } => {
                        match right {
                            Value::Num { value } => op.undo(target / value),
                            _ => bail!("Invalid undo operation: {:?}", &self)
                        }
                    }
                    Value::Num { value } => {
                        match right {
                            Value::Var => Ok(target / value),
                            Value::Expr { op } => op.undo(target / value),
                            _ => bail!("Invalid undo operation: {:?}", &self)
                        }
                    }
                }
            }
            Self::Div { left, right } => {
                match left {
                    Value::Var => {
                        match right {
                            Value::Num { value } => Ok(target * value),
                            _ => bail!("Invalid undo operation: {:?}", &self)
                        }
                    }
                    Value::Expr { op } => {
                        match right {
                            Value::Num { value } => op.undo(target * value),
                            _ => bail!("Invalid undo operation: {:?}", &self)
                        }
                    }
                    Value::Num { value } => {
                        match right {
                            Value::Var => Ok(value / target),
                            Value::Expr { op } => op.undo(value / target),
                            _ => bail!("Invalid undo operation: {:?}", &self)
                        }
                    }
                }
            }
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
            Self::Num { value }  => Ok(*value),
            _ => bail!("help")
        }
    }

    pub fn solve(&self, target: i64) -> Result<i64, anyhow::Error> {
        match self {
            Self::Expr { op } => op.undo(target),
            _ => bail!("Cannot call solve on anything but an expression")
        }
    }
}

impl Add<Value> for Value {
    type Output = Value;

    fn add(self, rhs: Value) -> Self::Output {
        match self {
            Value::Num { value: mine } => {
                match rhs {
                    Value::Num { value: their } => Value::Num { value: mine + their },
                    Value::Var => Value::Expr { op: Box::new(Op::Sum { left: self, right: rhs }) },
                    Value::Expr { .. } => Value::Expr { op: Box::new(Op::Sum { left: self, right: rhs }) },
                }
            }
            Value::Var => {
                match rhs {
                    Value::Num { .. } => Value::Expr { op: Box::new(Op::Sum { left: self, right: rhs }) },
                    Value::Expr { .. } => unreachable!("Found expression and var on two sides of an operation"),
                    Value::Var => unreachable!("Found var on two sides of an operation")
                }
            },
            Value::Expr { .. } => {
                match rhs {
                    Value::Num { .. } => Value::Expr { op: Box::new(Op::Sum { left: self, right: rhs }) },
                    Value::Expr { .. } => unreachable!("Found expression on two sides of an operation"),
                    Value::Var => unreachable!("Found expression and var on two sides of an operation")
                }
            }
        }
    }
}

impl Sub<Value> for Value {
    type Output = Value;

    fn sub(self, rhs: Value) -> Self::Output {
        match self {
            Value::Num { value: mine } => {
                match rhs {
                    Value::Num { value: their } => Value::Num { value: mine - their },
                    Value::Var => Value::Expr { op: Box::new(Op::Sub { left: self, right: rhs }) },
                    Value::Expr { .. } => Value::Expr { op: Box::new(Op::Sub { left: self, right: rhs }) },
                }
            }
            Value::Var => {
                match rhs {
                    Value::Num { .. } => Value::Expr { op: Box::new(Op::Sub { left: self, right: rhs }) },
                    Value::Expr { .. } => unreachable!("Found expression and var on two sides of an operation"),
                    Value::Var => unreachable!("Found var on two sides of an operation")
                }
            },
            Value::Expr { .. } => {
                match rhs {
                    Value::Num { .. } => Value::Expr { op: Box::new(Op::Sub { left: self, right: rhs }) },
                    Value::Expr { .. } => unreachable!("Found expression on two sides of an operation"),
                    Value::Var => unreachable!("Found expression and var on two sides of an operation")
                }
            }
        }
    }
}

impl Mul<Value> for Value {
    type Output = Value;

    fn mul(self, rhs: Value) -> Self::Output {
        match self {
            Value::Num { value: mine } => {
                match rhs {
                    Value::Num { value: their } => Value::Num { value: mine * their },
                    Value::Var => Value::Expr { op: Box::new(Op::Mul { left: self, right: rhs }) },
                    Value::Expr { .. } => Value::Expr { op: Box::new(Op::Mul { left: self, right: rhs }) },
                }
            }
            Value::Var => {
                match rhs {
                    Value::Num { .. } => Value::Expr { op: Box::new(Op::Mul { left: self, right: rhs }) },
                    Value::Expr { .. } => unreachable!("Found expression and var on two sides of an operation"),
                    Value::Var => unreachable!("Found var on two sides of an operation")
                }
            },
            Value::Expr { .. } => {
                match rhs {
                    Value::Num { .. } => Value::Expr { op: Box::new(Op::Mul { left: self, right: rhs }) },
                    Value::Expr { .. } => unreachable!("Found expression on two sides of an operation"),
                    Value::Var => unreachable!("Found expression and var on two sides of an operation")
                }
            }
        }
    }
}

impl Div<Value> for Value {
    type Output = Value;

    fn div(self, rhs: Value) -> Self::Output {
        match self {
            Value::Num { value: mine } => {
                match rhs {
                    Value::Num { value: their } => Value::Num { value: mine / their },
                    Value::Var => Value::Expr { op: Box::new(Op::Div { left: self, right: rhs }) },
                    Value::Expr { .. } => Value::Expr { op: Box::new(Op::Div { left: self, right: rhs }) },
                }
            }
            Value::Var => {
                match rhs {
                    Value::Num { .. } => Value::Expr { op: Box::new(Op::Div { left: self, right: rhs }) },
                    Value::Expr { .. } => unreachable!("Found expression and var on two sides of an operation"),
                    Value::Var => unreachable!("Found var on two sides of an operation")
                }
            },
            Value::Expr { .. } => {
                match rhs {
                    Value::Num { .. } => Value::Expr { op: Box::new(Op::Div { left: self, right: rhs }) },
                    Value::Expr { .. } => unreachable!("Found expression on two sides of an operation"),
                    Value::Var => unreachable!("Found expression and var on two sides of an operation")
                }
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Job {
    Sum { left: String, right: String },
    Sub { left: String, right: String },
    Mul { left: String, right: String },
    Div { left: String, right: String },
    Yell { value: i64 },
}

impl Job {
    /// So this is a direct solve without the extra `Value` overhead
    pub fn output(&self, monkeys: &FxHashMap<String, Monkey>, cache: &mut FxHashMap<String, i64>) -> Result<i64, anyhow::Error> {
        match self {
            Self::Sum { left, right } => {
                let l = monkeys.get(left).ok_or_else(|| anyhow!("Unknown monkey: {}", left))?;
                let r = monkeys.get(right).ok_or_else(|| anyhow!("Unknown monkey: {}", left))?;
                Ok(l.output(monkeys, cache)? + r.output(monkeys, cache)?)
            }
            Self::Sub { left, right } => {
                let l = monkeys.get(left).ok_or_else(|| anyhow!("Unknown monkey: {}", left))?;
                let r = monkeys.get(right).ok_or_else(|| anyhow!("Unknown monkey: {}", left))?;
                Ok(l.output(monkeys, cache)? - r.output(monkeys, cache)?)
            }
            Self::Mul { left, right } => {
                let l = monkeys.get(left).ok_or_else(|| anyhow!("Unknown monkey: {}", left))?;
                let r = monkeys.get(right).ok_or_else(|| anyhow!("Unknown monkey: {}", left))?;
                Ok(l.output(monkeys, cache)? * r.output(monkeys, cache)?)
            }
            Self::Div { left, right } => {
                let l = monkeys.get(left).ok_or_else(|| anyhow!("Unknown monkey: {}", left))?;
                let r = monkeys.get(right).ok_or_else(|| anyhow!("Unknown monkey: {}", left))?;
                Ok(l.output(monkeys, cache)? / r.output(monkeys, cache)?)
            }
            Self::Yell { value } => Ok(*value),
        }
    }

    /// So this pays the `Value` penalty to allow the solver to work.
    pub fn value_output(&self, monkeys: &FxHashMap<String, Monkey>, cache: &mut FxHashMap<String, Value>) -> Result<Value, anyhow::Error> {
        match self {
            Self::Sum { left, right } => {
                let l = monkeys.get(left).ok_or_else(|| anyhow!("Unknown monkey: {}", left))?;
                let r = monkeys.get(right).ok_or_else(|| anyhow!("Unknown monkey: {}", left))?;
                Ok(l.value_output(monkeys, cache)? + r.value_output(monkeys, cache)?)
            }
            Self::Sub { left, right } => {
                let l = monkeys.get(left).ok_or_else(|| anyhow!("Unknown monkey: {}", left))?;
                let r = monkeys.get(right).ok_or_else(|| anyhow!("Unknown monkey: {}", left))?;
                Ok(l.value_output(monkeys, cache)? - r.value_output(monkeys, cache)?)
            }
            Self::Mul { left, right } => {
                let l = monkeys.get(left).ok_or_else(|| anyhow!("Unknown monkey: {}", left))?;
                let r = monkeys.get(right).ok_or_else(|| anyhow!("Unknown monkey: {}", left))?;
                Ok(l.value_output(monkeys, cache)? * r.value_output(monkeys, cache)?)
            }
            Self::Div { left, right } => {
                let l = monkeys.get(left).ok_or_else(|| anyhow!("Unknown monkey: {}", left))?;
                let r = monkeys.get(right).ok_or_else(|| anyhow!("Unknown monkey: {}", left))?;
                Ok(l.value_output(monkeys, cache)? / r.value_output(monkeys, cache)?)
            }
            Self::Yell { value } => Ok(Value::Num { value: *value }),
        }
    }
}

fn parse_sum(input: &str) -> IResult<&str, Job> {
    let (input, (left, right)) = separated_pair(alpha1, tag(" + ") , alpha1)(input)?;
    Ok((input, Job::Sum { left: left.into(), right: right.into() }))
}

fn parse_sub(input: &str) -> IResult<&str, Job> {
    let (input, (left, right)) = separated_pair(alpha1, tag(" - ") , alpha1)(input)?;
    Ok((input, Job::Sub { left: left.into(), right: right.into() }))
}

fn parse_mul(input: &str) -> IResult<&str, Job> {
    let (input, (left, right)) = separated_pair(alpha1, tag(" * ") , alpha1)(input)?;
    Ok((input, Job::Mul { left: left.into(), right: right.into() }))
}

fn parse_div(input: &str) -> IResult<&str, Job> {
    let (input, (left, right)) = separated_pair(alpha1, tag(" / ") , alpha1)(input)?;
    Ok((input, Job::Div { left: left.into(), right: right.into() }))
}

fn parse_yell(input: &str) -> IResult<&str, Job> {
    let (input, value) = nom_i64(input)?;
    Ok((input, Job::Yell { value }))
}

fn parse_job(input: &str) -> IResult<&str, Job> {
    alt((parse_sum, parse_sub, parse_mul, parse_div, parse_yell))(input)
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Monkey {
    name: String,
    job: Job,
}

impl Monkey {
    pub fn output(&self, monkeys: &FxHashMap<String, Monkey>, cache: &mut FxHashMap<String, i64>) -> Result<i64, anyhow::Error> {
        if let Some(value) = cache.get(&self.name) {
            return Ok(*value);
        }

        let value = self.job.output(monkeys, cache)?;

        cache.insert(self.name.clone(), value);

        Ok(value)
    }

    pub fn value_output(&self, monkeys: &FxHashMap<String, Monkey>, cache: &mut FxHashMap<String, Value>) -> Result<Value, anyhow::Error> {
        if &self.name == "humn" {
            return Ok(Value::Var);
        }

        if let Some(value) = cache.get(&self.name) {
            return Ok(value.clone());
        }

        let value = self.job.value_output(monkeys, cache)?;

        cache.insert(self.name.clone(), value.clone());

        Ok(value)
    }

    pub fn depends_on_human(&self, monkeys: &FxHashMap<String, Monkey>) -> Result<bool, anyhow::Error> {
        if &self.name == "humn" {
            return Ok(true);
        }

        if let Ok((left, right)) = self.left_and_right(monkeys) {
            Ok(left.depends_on_human(monkeys)? || right.depends_on_human(monkeys)?)
        } else {
            Ok(false)
        }
    }

    pub fn left_and_right<'a>(&self, monkeys: &'a FxHashMap<String, Monkey>) -> Result<(&'a Monkey, &'a Monkey), anyhow::Error> {
        match &self.job {
            Job::Yell { .. } => bail!("cannot get left and right on Yell"),
            Job::Sum { left, right } | Job::Sub { left, right } | Job::Mul { left, right } | Job::Div { left, right } => {
                let l = monkeys.get(left).ok_or_else(|| anyhow!("unknown monkey: {}", left))?;
                let r = monkeys.get(right).ok_or_else(|| anyhow!("unknown monkey: {}", right))?;
                Ok((l, r))
            }
        }
    }
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, (name, job)) = separated_pair(alpha1, tag(": "), parse_job)(input)?;
    Ok((input, Monkey { name: name.into(), job }))
}

fn parse_monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(newline, parse_monkey)(input)
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MonkeyMath {
    monkeys: FxHashMap<String, Monkey>,
}

impl FromStr for MonkeyMath {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, raw_monkeys) = parse_monkeys(s).map_err(|e| e.to_owned())?;

        let mut monkeys = FxHashMap::with_capacity_and_hasher(raw_monkeys.len(), Default::default());

        for monkey in raw_monkeys {
            monkeys.insert(monkey.name.clone(), monkey);
        }

        Ok(
            Self {
                monkeys
            }
        )
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
        let mut cache = FxHashMap::with_capacity_and_hasher(self.monkeys.len(), Default::default());
        let root = self.monkeys.get("root").ok_or_else(|| anyhow!("no monkey named root"))?;
        root.output(&self.monkeys, &mut cache)
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        let root = self.monkeys.get("root").ok_or_else(|| anyhow!("no monkey named root"))?;

        let (left, right) = root.left_and_right(&self.monkeys)?;

        let mut cache = FxHashMap::with_capacity_and_hasher(self.monkeys.len(), Default::default());

        // we can compute each side independently without implementing == for
        // root
        let l = left.value_output(&self.monkeys, &mut cache)?;
        let r = right.value_output(&self.monkeys, &mut cache)?;

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
