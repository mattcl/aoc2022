use std::{collections::VecDeque, hash::Hash, str::FromStr};

use aoc_helpers::generic::Bound2D;
use aoc_plumbing::Problem;
use rustc_hash::{FxHashMap, FxHashSet};

const N_NE_NW: usize = 0b10010100;
const S_SE_SW: usize = 0b00101001;
const W_NW_SW: usize = 0b00000111;
const E_NE_SE: usize = 0b11100000;
const NEIGHBORS: [(i16, i16); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct Point {
    x: i16,
    y: i16,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Choice {
    North,
    South,
    West,
    East,
}

impl Choice {
    pub fn propose(
        &self,
        elf: &Point,
        neighbors: usize,
        proposals: &mut FxHashMap<Point, Vec<Point>>,
    ) -> bool {
        match self {
            Self::North => {
                if neighbors & N_NE_NW == 0 {
                    let next = Point {
                        x: elf.x,
                        y: elf.y + 1,
                    };
                    let e = proposals.entry(next).or_default();
                    e.push(*elf);
                    true
                } else {
                    false
                }
            }
            Self::South => {
                if neighbors & S_SE_SW == 0 {
                    let next = Point {
                        x: elf.x,
                        y: elf.y - 1,
                    };
                    let e = proposals.entry(next).or_default();
                    e.push(*elf);
                    true
                } else {
                    false
                }
            }
            Self::West => {
                if neighbors & W_NW_SW == 0 {
                    let next = Point {
                        x: elf.x - 1,
                        y: elf.y,
                    };
                    let e = proposals.entry(next).or_default();
                    e.push(*elf);
                    true
                } else {
                    false
                }
            }
            Self::East => {
                if neighbors & E_NE_SE == 0 {
                    let next = Point {
                        x: elf.x + 1,
                        y: elf.y,
                    };
                    let e = proposals.entry(next).or_default();
                    e.push(*elf);
                    true
                } else {
                    false
                }
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnstableDiffusion {
    elves: FxHashSet<Point>,
}

impl UnstableDiffusion {
    pub fn rounds(&mut self, num: usize) -> i16 {
        let mut proposals: FxHashMap<Point, Vec<Point>> =
            FxHashMap::with_capacity_and_hasher(self.elves.len(), Default::default());

        let mut choices = VecDeque::with_capacity(4);
        choices.push_back(Choice::North);
        choices.push_back(Choice::South);
        choices.push_back(Choice::West);
        choices.push_back(Choice::East);

        for _ in 0..num {
            for elf in self.elves.iter() {
                let mut found_neighbors = 0;
                for (n_idx, (dx, dy)) in NEIGHBORS.iter().enumerate() {
                    let n = Point {
                        x: elf.x + dx,
                        y: elf.y + dy,
                    };
                    if self.elves.contains(&n) {
                        found_neighbors |= 1 << n_idx;
                    }
                }

                if found_neighbors == 0 {
                    continue;
                }

                for choice in choices.iter() {
                    if choice.propose(elf, found_neighbors, &mut proposals) {
                        break;
                    }
                }
            }

            for (dest, elves) in proposals.drain() {
                if elves.len() == 1 {
                    self.elves.remove(&elves[0]);
                    self.elves.insert(dest);
                }
            }

            let first = choices.pop_front().unwrap();
            choices.push_back(first);
        }

        let mut bounds: Bound2D<i16> = Bound2D::minmax();
        for elf in self.elves.iter() {
            if elf.x < bounds.min_x {
                bounds.min_x = elf.x;
            }

            if elf.x > bounds.max_x {
                bounds.max_x = elf.x;
            }

            if elf.y < bounds.min_y {
                bounds.min_y = elf.y;
            }

            if elf.y > bounds.max_y {
                bounds.max_y = elf.y;
            }
        }

        bounds.width() * bounds.height() - self.elves.len() as i16
    }

    pub fn rounds_until_no_moves(&mut self) -> usize {
        let mut proposals: FxHashMap<Point, Vec<Point>> =
            FxHashMap::with_capacity_and_hasher(self.elves.len(), Default::default());

        let mut choices = VecDeque::with_capacity(4);
        choices.push_back(Choice::North);
        choices.push_back(Choice::South);
        choices.push_back(Choice::West);
        choices.push_back(Choice::East);

        let mut count = 0;

        loop {
            count += 1;

            for elf in self.elves.iter() {
                let mut found_neighbors = 0;
                for (n_idx, (dx, dy)) in NEIGHBORS.iter().enumerate() {
                    let n = Point {
                        x: elf.x + dx,
                        y: elf.y + dy,
                    };
                    if self.elves.contains(&n) {
                        found_neighbors |= 1 << n_idx;
                    }
                }

                if found_neighbors == 0 {
                    continue;
                }

                for choice in choices.iter() {
                    if choice.propose(elf, found_neighbors, &mut proposals) {
                        break;
                    }
                }
            }

            let mut any_move = false;
            for (dest, elves) in proposals.drain() {
                if elves.len() == 1 {
                    self.elves.remove(&elves[0]);
                    self.elves.insert(dest);
                    any_move = true;
                }
            }

            if !any_move {
                break count;
            }

            let first = choices.pop_front().unwrap();
            choices.push_back(first);
        }
    }
}

impl FromStr for UnstableDiffusion {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut elves = FxHashSet::default();

        for (y, line) in s.trim().lines().rev().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch == '#' {
                    elves.insert(Point {
                        x: x as i16,
                        y: y as i16,
                    });
                }
            }
        }
        Ok(Self { elves })
    }
}

impl Problem for UnstableDiffusion {
    const DAY: usize = 23;
    const TITLE: &'static str = "unstable diffusion";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = i16;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        let mut working = self.clone();
        Ok(working.rounds(10))
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        let mut working = self.clone();
        Ok(working.rounds_until_no_moves())
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
        let solution = UnstableDiffusion::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(3788, 921));
    }

    #[test]
    fn example() {
        let input = "..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............";
        let solution = UnstableDiffusion::solve(input).unwrap();
        assert_eq!(solution, Solution::new(110, 20));
    }
}
