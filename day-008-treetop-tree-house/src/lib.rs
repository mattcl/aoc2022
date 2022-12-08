use std::str::FromStr;

use anyhow::{anyhow, bail};
use aoc_plumbing::Problem;
use rayon::prelude::*;
use rustc_hash::FxHashSet;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct VisualRange {
    left: usize,
    right: usize,
    up: usize,
    down: usize,
}

impl VisualRange {
    pub fn score(&self) -> usize {
        self.left * self.right * self.up * self.down
    }
}

impl Default for VisualRange {
    fn default() -> Self {
        Self {
            left: 1,
            right: 1,
            up: 1,
            down: 1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TreetopTreeHouse {
    grid: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl TreetopTreeHouse {
    fn compute_range(&self, row: usize, col: usize) -> VisualRange {
        let mut vr = VisualRange::default();

        if row > self.height || col > self.width {
            return vr;
        }

        let height = self.grid[row][col];

        // down
        let mut working = 0;
        for r in (row + 1)..self.height {
            working += 1;
            if self.grid[r][col] >= height {
                break;
            }
        }
        vr.down *= working;

        // up
        working = 0;
        for r in 1..=row {
            working += 1;
            if self.grid[row - r][col] >= height {
                break;
            }
        }
        vr.up *= working;

        // right
        working = 0;
        for c in (col + 1)..self.width {
            working += 1;
            if self.grid[row][c] >= height {
                break;
            }
        }
        vr.right *= working;

        //  left
        working = 0;
        for c in 1..=col {
            working += 1;
            if self.grid[row][col - c] >= height {
                break;
            }
        }
        vr.left *= working;

        vr
    }
}

impl FromStr for TreetopTreeHouse {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = Vec::default();

        for line in s.trim().lines() {
            grid.push(
                line.trim()
                    .chars()
                    .map(|ch| {
                        ch.to_digit(10)
                            .map(|v| v as u8)
                            .ok_or_else(|| anyhow!("Invalid char: {}", ch))
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            );
        }

        let height = grid.len();
        let width = grid[0].len();

        if grid.iter().any(|r| r.len() != width) {
            bail!("Grid has uneven rows");
        }

        Ok(Self {
            grid,
            width,
            height,
        })
    }
}

impl Problem for TreetopTreeHouse {
    const DAY: usize = 8;
    const TITLE: &'static str = "treetop tree house";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        // we can look _in_ from the edges
        let mut seen: FxHashSet<usize> = FxHashSet::default();

        for row in 1..(self.height - 1) {
            let left_edge = self.grid[row][0];
            let right_edge = self.grid[row][self.width - 1];
            let mut last_insert_col = 0;
            // look right
            let mut largest_seen = left_edge;
            for col in 1..(self.width - 1) {
                let idx = row * 10000 + col;
                if self.grid[row][col] > largest_seen {
                    largest_seen = self.grid[row][col];
                    seen.insert(idx);
                    last_insert_col = col;
                }
            }

            // look left
            let mut largest_seen = right_edge;
            for col in ((last_insert_col + 1)..(self.width - 1)).rev() {
                let idx = row * 10000 + col;

                if self.grid[row][col] > largest_seen {
                    largest_seen = self.grid[row][col];
                    seen.insert(idx);
                }
            }
        }

        for col in 1..(self.width - 1) {
            let top_edge = self.grid[0][col];
            let bot_edge = self.grid[self.height - 1][col];
            let mut last_insert_row = 0;
            // look down
            let mut largest_seen = top_edge;
            for row in 1..(self.height - 1) {
                let idx = row * 10000 + col;

                if self.grid[row][col] > largest_seen {
                    largest_seen = self.grid[row][col];
                    seen.insert(idx);
                    last_insert_row = row;
                }
            }

            // look up
            let mut largest_seen = bot_edge;
            for row in ((last_insert_row + 1)..(self.height - 1)).rev() {
                let idx = row * 10000 + col;

                if self.grid[row][col] > largest_seen {
                    largest_seen = self.grid[row][col];
                    seen.insert(idx);
                }
            }
        }

        // we know that the number of trees around the edge, which are visible
        // by default, is 2x the width + 2x the height minus 4 for the corners
        // which were each counted twice.
        Ok(seen.len() + self.width * 2 + self.height * 2 - 4)
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        // because we don't count edge cells, we know the minimum max is 4,
        // given a uniform grid of the same number
        (1..(self.height - 1))
            .into_par_iter()
            .filter_map(|row| {
                (1..(self.width - 1))
                    .map(|col| self.compute_range(row, col).score())
                    .max()
            })
            .max()
            .ok_or_else(|| anyhow!("Could not find max value"))
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
        let solution = TreetopTreeHouse::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(1859, 332640));
    }

    #[test]
    fn example() {
        let input = "
            30373
            25512
            65332
            33549
            35390
            ";
        let solution = TreetopTreeHouse::solve(input).unwrap();
        assert_eq!(solution, Solution::new(21, 8));
    }
}
