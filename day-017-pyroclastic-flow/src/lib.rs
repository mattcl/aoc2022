use std::{fmt::Display, str::FromStr};

use anyhow::bail;
use aoc_plumbing::Problem;
use rustc_hash::FxHashMap;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct Point {
    x: u8,
    y: usize,
}

impl Point {
    pub fn new(x: u8, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Jet {
    Left,
    Right,
}

impl TryFrom<char> for Jet {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '<' => Ok(Jet::Left),
            '>' => Ok(Jet::Right),
            _ => bail!("Invalid jet: {}", value),
        }
    }
}

const PLUS: [u8; 3] = [0b010, 0b111, 0b010];

const SQUARE: [u8; 2] = [0b11, 0b11];

const HORIZONTAL: [u8; 1] = [0b1111];

const VERTICAL: [u8; 4] = [0b1, 0b1, 0b1, 0b1];

const CORNER: [u8; 3] = [0b111, 0b001, 0b001];

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Shape {
    Plus,
    Square,
    Horizontal,
    Vertical,
    Corner,
}

impl Shape {
    pub fn width(&self) -> u8 {
        match self {
            Self::Plus => 3,
            Self::Square => 2,
            Self::Horizontal => 4,
            Self::Vertical => 1,
            Self::Corner => 3,
        }
    }

    pub fn height(&self) -> usize {
        match self {
            Self::Plus => 3,
            Self::Square => 2,
            Self::Horizontal => 1,
            Self::Vertical => 4,
            Self::Corner => 3,
        }
    }

    pub fn bot(&self) -> u8 {
        match self {
            Self::Plus => PLUS[0],
            Self::Square => SQUARE[0],
            Self::Horizontal => HORIZONTAL[0],
            Self::Vertical => VERTICAL[0],
            Self::Corner => CORNER[0],
        }
    }

    pub fn rows(&self) -> impl Iterator<Item = &u8> {
        match self {
            Self::Plus => PLUS.iter(),
            Self::Square => SQUARE.iter(),
            Self::Horizontal => HORIZONTAL.iter(),
            Self::Vertical => VERTICAL.iter(),
            Self::Corner => CORNER.iter(),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Rock {
    location: Point,
    shape: Shape,
}

impl Rock {
    pub fn new(location: Point, shape: Shape) -> Self {
        Self { location, shape }
    }

    pub fn collides_bot(&self, rows: &Vec<u8>) -> bool {
        let row = self.location.y;
        if row == 0 {
            return true;
        }
        let start_row = self.location.y - 1;
        let shift = self.location.x;
        for (idx, row) in self.shape.rows().enumerate() {
            let mask = row << shift;
            if mask & rows[start_row + idx] > 0 {
                return true;
            }
        }

        false
    }

    pub fn collides_left(&self, rows: &Vec<u8>) -> bool {
        let start_row = self.location.y;
        if 6 - self.shape.width() + 1 == self.location.x {
            return true;
        }
        let shift = self.location.x + 1;
        for (idx, row) in self.shape.rows().enumerate() {
            let mask = row << shift;
            if mask & rows[start_row + idx] > 0 {
                return true;
            }
        }

        false
    }

    pub fn collides_right(&self, rows: &Vec<u8>) -> bool {
        let start_row = self.location.y;
        if self.location.x == 0 {
            return true;
        }
        let shift = self.location.x - 1;
        for (idx, row) in self.shape.rows().enumerate() {
            let mask = row << shift;
            if mask & rows[start_row + idx] > 0 {
                return true;
            }
        }

        false
    }

    /// Adds the points for this rock to the map and returns the highest value
    pub fn add_points(&self, rows: &mut Vec<u8>) -> usize {
        let start_row = self.location.y;
        let shift = self.location.x;
        let mut max = start_row;
        for (idx, row) in self.shape.rows().enumerate() {
            let mask = row << shift;
            assert!(rows[start_row + idx] & mask == 0);
            rows[start_row + idx] |= mask;
            assert!(rows[start_row + idx] & mask > 0);
            if start_row + idx > max {
                max = start_row + idx;
            }
        }

        max
    }

    pub fn move_jet(&mut self, jet: &Jet, rows: &Vec<u8>) -> bool {
        match jet {
            Jet::Left => {
                if self.collides_left(rows) {
                    false
                } else {
                    self.location.x += 1;
                    true
                }
            }
            Jet::Right => {
                if self.collides_right(rows) {
                    false
                } else {
                    self.location.x -= 1;
                    true
                }
            }
        }
    }

    pub fn move_down(&mut self, rows: &Vec<u8>) -> bool {
        if self.collides_bot(rows) {
            false
        } else {
            self.location.y -= 1;
            true
        }
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Chamber {
    rows: Vec<u8>,
}

impl Chamber {
    pub fn drop_rocks(&mut self, num: usize, jets: &Vec<Jet>) -> usize {
        let mut highest = 0;
        let mut shapes = [
            Shape::Horizontal,
            Shape::Plus,
            Shape::Corner,
            Shape::Vertical,
            Shape::Square,
        ]
        .iter()
        .cycle();
        let mut jets_iter = jets.iter().cycle();

        for i in 0..num {
            let y = if i == 0 { 3 } else { highest + 4 };

            let shape = *shapes.next().unwrap();
            let location = Point {
                x: 6 - shape.width() - 1,
                y,
            };
            while self.rows.len() < location.y + shape.height() {
                self.rows.push(0);
            }
            let mut rock = Rock::new(location, shape);

            while let Some(jet) = jets_iter.next() {
                rock.move_jet(jet, &self.rows);
                // we can't move down because 0
                if !rock.move_down(&self.rows) {
                    let candidate = rock.add_points(&mut self.rows);
                    if candidate > highest {
                        highest = candidate;
                    }
                    break;
                }
            }
        }

        highest + 1
    }

    pub fn detect_cycle(&mut self, jets: &Vec<Jet>) -> usize {
        let mut highest = 0;
        let mut shapes = [
            Shape::Horizontal,
            Shape::Plus,
            Shape::Corner,
            Shape::Vertical,
            Shape::Square,
        ]
        .iter()
        .enumerate()
        .cycle();
        let mut jets_iter = jets.iter().enumerate().cycle();

        let mut states: FxHashMap<State, (usize, usize)> = FxHashMap::default();
        let mut i = 0;
        loop {
            let y = if i == 0 { 3 } else { highest + 4 };

            let (shape_idx, shape) = shapes.next().unwrap();
            let location = Point {
                x: 6 - shape.width() - 1,
                y,
            };
            while self.rows.len() < location.y + shape.height() {
                self.rows.push(0);
            }
            let mut rock = Rock::new(location, *shape);

            while let Some((jet_idx, jet)) = jets_iter.next() {
                rock.move_jet(jet, &self.rows);
                // we can't move down because 0
                if !rock.move_down(&self.rows) {
                    let candidate = rock.add_points(&mut self.rows);
                    if candidate > highest {
                        highest = candidate;
                    }

                    if i > 16 {
                        let state = State::new(shape_idx, jet_idx, &self.rows);

                        let e = states.entry(state).or_insert_with(|| (i, highest));
                        if e.0 != i {
                            let period = i - e.0;
                            if 1_000_000_000_000 % period == i % period {
                                let fp = highest - e.1;
                                let rem = (1_000_000_000_000 - i) / period;
                                return highest + rem * fp;
                            }
                        }
                    }

                    break;
                }
            }

            i += 1;
        }
    }
}

impl Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rows.iter().rev() {
            let mut buf = String::with_capacity(10);
            buf.push('|');
            for shift in (0..7).rev() {
                let mask = 1 << shift;
                if row & mask > 0 {
                    buf.push('#');
                } else {
                    buf.push(' ');
                }
            }
            buf.push('|');
            buf.push('\n');

            buf.fmt(f)?;
        }
        writeln!(f, "+-------+")
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct State {
    shape_idx: usize,
    jet_idx: usize,
    top_rows: u64,
}

impl State {
    pub fn new(shape_idx: usize, jet_idx: usize, rows: &[u8]) -> Self {
        let mut top_rows = 0;

        for i in 0..8 {
            top_rows |= (rows[rows.len() - 1 - i] as u64) << i * 8;
        }

        Self {
            shape_idx,
            jet_idx,
            top_rows,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct PyroclasticFlow {
    jets: Vec<Jet>,
    chamber: Chamber,
}

impl FromStr for PyroclasticFlow {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let jets = s
            .trim()
            .chars()
            .map(Jet::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            jets,
            ..Default::default()
        })
    }
}

impl Problem for PyroclasticFlow {
    const DAY: usize = 17;
    const TITLE: &'static str = "pyroclastic flow";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        let mut working = self.chamber.clone();
        let highest = working.drop_rocks(2022, &self.jets);
        Ok(highest)
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        let mut working = self.chamber.clone();
        let highest = working.detect_cycle(&self.jets);
        Ok(highest)
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
        let solution = PyroclasticFlow::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(3166, 1577207977186));
    }

    #[test]
    fn example() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        let solution = PyroclasticFlow::solve(input).unwrap();
        assert_eq!(solution, Solution::new(3068, 1514285714288));
    }
}
