use std::{collections::VecDeque, hash::Hash, str::FromStr};

use aoc_helpers::generic::Bound2D;
use aoc_plumbing::Problem;
use rustc_hash::FxHashSet;

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

//                              | N     | S      | W | E
const NORTH_ORDER: [usize; 8] = [2, 4, 7, 0, 3, 5, 1, 6];
const NORTH_CHUNKS: [usize; 4] = [3, 3, 1, 1];
//                              | S     | W   | E   | N
const SOUTH_ORDER: [usize; 8] = [0, 3, 5, 1, 2, 6, 7, 4];
const SOUTH_CHUNKS: [usize; 4] = [3, 2, 2, 1];
//                             | W     | E      | N | S
const WEST_ORDER: [usize; 8] = [0, 1, 2, 5, 6, 7, 4, 3];
const WEST_CHUNKS: [usize; 4] = [3, 3, 1, 1];
//                             | E     | N   | S   | W
const EAST_ORDER: [usize; 8] = [5, 6, 7, 2, 4, 0, 3, 1];
const EAST_CHUNKS: [usize; 4] = [3, 2, 2, 1];

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
    pub fn order_when_first(&self) -> &'static [usize] {
        match self {
            Self::North => &NORTH_ORDER,
            Self::South => &SOUTH_ORDER,
            Self::East => &EAST_ORDER,
            Self::West => &WEST_ORDER,
        }
    }

    pub fn chunks_when_first(&self) -> &'static [usize] {
        match self {
            Self::North => &NORTH_CHUNKS,
            Self::South => &SOUTH_CHUNKS,
            Self::East => &EAST_CHUNKS,
            Self::West => &WEST_CHUNKS,
        }
    }

    pub fn corresponding_mask(&self) -> usize {
        match self {
            Self::North => N_NE_NW,
            Self::South => S_SE_SW,
            Self::East => E_NE_SE,
            Self::West => W_NW_SW,
        }
    }

    pub fn propose(&self, elf: &Point, neighbors: usize) -> Option<Point> {
        match self {
            Self::North => {
                if neighbors & N_NE_NW == 0 {
                    Some(Point {
                        x: elf.x,
                        y: elf.y + 1,
                    })
                } else {
                    None
                }
            }
            Self::South => {
                if neighbors & S_SE_SW == 0 {
                    Some(Point {
                        x: elf.x,
                        y: elf.y - 1,
                    })
                } else {
                    None
                }
            }
            Self::West => {
                if neighbors & W_NW_SW == 0 {
                    Some(Point {
                        x: elf.x - 1,
                        y: elf.y,
                    })
                } else {
                    None
                }
            }
            Self::East => {
                if neighbors & E_NE_SE == 0 {
                    Some(Point {
                        x: elf.x + 1,
                        y: elf.y,
                    })
                } else {
                    None
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
        let mut choices = VecDeque::with_capacity(4);
        choices.push_back(Choice::North);
        choices.push_back(Choice::South);
        choices.push_back(Choice::West);
        choices.push_back(Choice::East);

        for _ in 0..num {
            let mut next_elves =
                FxHashSet::with_capacity_and_hasher(self.elves.len(), Default::default());
            let order = choices[0].order_when_first();
            let chunks = choices[0].chunks_when_first();

            for elf in self.elves.iter() {
                let mut choice_idxs = order.iter();
                let mut chunks = chunks.iter();
                let mut prop: Option<Point> = None;
                let mut found_neighbors = 0;

                for choice in choices.iter() {
                    for _ in 0..*chunks.next().unwrap() {
                        let n_idx = *choice_idxs.next().unwrap();
                        let (dx, dy) = NEIGHBORS[n_idx];
                        let n = Point {
                            x: elf.x + dx,
                            y: elf.y + dy,
                        };
                        if self.elves.contains(&n) {
                            found_neighbors |= 1 << n_idx;
                        }
                    }

                    if prop.is_none() {
                        if let Some(dest) = choice.propose(elf, found_neighbors) {
                            prop = Some(dest);
                            // we can only break early when we make a choice if
                            // we've found at least one neighbor, because this
                            // might have been the first choice and we need
                            // to check for others
                            if found_neighbors > 0 {
                                break;
                            }
                        }
                    } else if found_neighbors > 0 {
                        // we have already made a choice in a previous iteration
                        // and we've found a neighbor, so break early
                        break;
                    }
                }

                // add the proposal
                if found_neighbors > 0 {
                    if let Some(dest) = prop {
                        if !next_elves.insert(dest) {
                            next_elves.remove(&dest);
                            next_elves.insert(*elf);
                            next_elves.insert(Point {
                                x: dest.x * 2 - elf.x,
                                y: dest.y * 2 - elf.y,
                            });
                        }
                        continue;
                    }
                }

                next_elves.insert(*elf);
            }

            let first = choices.pop_front().unwrap();
            choices.push_back(first);

            self.elves = next_elves;
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
        let mut choices = VecDeque::with_capacity(4);
        choices.push_back(Choice::North);
        choices.push_back(Choice::South);
        choices.push_back(Choice::West);
        choices.push_back(Choice::East);

        let mut count = 0;

        loop {
            count += 1;
            let mut moved = 0;
            let mut next_elves =
                FxHashSet::with_capacity_and_hasher(self.elves.len(), Default::default());
            let order = choices[0].order_when_first();
            let chunks = choices[0].chunks_when_first();

            for elf in self.elves.iter() {
                let mut choice_idxs = order.iter();
                let mut chunks = chunks.iter();
                let mut prop: Option<Point> = None;
                let mut found_neighbors = 0;

                for choice in choices.iter() {
                    for _ in 0..*chunks.next().unwrap() {
                        let n_idx = *choice_idxs.next().unwrap();
                        let (dx, dy) = NEIGHBORS[n_idx];
                        let n = Point {
                            x: elf.x + dx,
                            y: elf.y + dy,
                        };
                        if self.elves.contains(&n) {
                            found_neighbors |= 1 << n_idx;
                        }
                    }

                    if prop.is_none() {
                        if let Some(dest) = choice.propose(elf, found_neighbors) {
                            prop = Some(dest);
                            // we can only break early when we make a choice if
                            // we've found at least one neighbor, because this
                            // might have been the first choice and we need
                            // to check for others
                            if found_neighbors > 0 {
                                break;
                            }
                        }
                    } else if found_neighbors > 0 {
                        // we have already made a choice in a previous iteration
                        // and we've found a neighbor, so break early
                        break;
                    }
                }

                // add the proposal
                if found_neighbors > 0 {
                    if let Some(dest) = prop {
                        if !next_elves.insert(dest) {
                            next_elves.remove(&dest);
                            next_elves.insert(*elf);
                            next_elves.insert(Point {
                                x: dest.x * 2 - elf.x,
                                y: dest.y * 2 - elf.y,
                            });
                            moved -= 1;
                        } else {
                            moved += 1;
                        }
                        continue;
                    }
                }

                next_elves.insert(*elf);
            }

            if moved == 0 {
                break count;
            }

            self.elves = next_elves;

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
