use std::str::FromStr;

use anyhow::{anyhow, bail};
use aoc_plumbing::Problem;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct VisualRange {
    score: usize,
    seen_edge: bool,
}

impl VisualRange {
    pub fn score(&self) -> usize {
        self.score
    }

    pub fn can_see_edge(&self) -> bool {
        self.seen_edge
    }
}

#[derive(Debug, Clone)]
pub struct TreetopTreeHouse {
    grid: Vec<Vec<u8>>,
    max_score: usize,
    width: usize,
    height: usize,
}

impl TreetopTreeHouse {
    fn compute_range(&self, row: usize, col: usize) -> VisualRange {
        let mut vr = VisualRange::default();

        if row > self.height || col > self.width {
            return vr;
        }

        // these are the max values
        let mut left = col;
        let mut right = self.width - col - 1;
        let mut up = row;
        let mut down = self.height - row - 1;

        let height = self.grid[row][col];

        // down
        for r in (row + 1)..self.height {
            if self.grid[r][col] >= height {
                down = r - row;
                break;
            }

            if r == self.height - 1 {
                vr.seen_edge = true;
            }
        }

        // up
        for r in 1..=row {
            if self.grid[row - r][col] >= height {
                up = r;
                break;
            }

            if r == row {
                vr.seen_edge = true;
            }
        }

        // right
        for c in (col + 1)..self.width {
            if self.grid[row][c] >= height {
                right = c - col;
                break;
            }

            if c == self.width - 1 {
                vr.seen_edge = true;
            }
        }

        //  left
        for c in 1..=col {
            if self.grid[row][col - c] >= height {
                left = c;
                break;
            }

            if c == col {
                vr.seen_edge = true;
            }
        }

        vr.score = left * right * up * down;

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
            max_score: 0,
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

    // see the comment for part two about why this is a combined day
    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        // initial count is everytihng on the edge
        let mut visible = self.width * 2 + self.height * 2 - 4;

        for row in 1..(self.height - 1) {
            for col in 1..(self.width - 1) {
                let vr = self.compute_range(row, col);
                if vr.seen_edge {
                    visible += 1;
                }
                if vr.score > self.max_score {
                    self.max_score = vr.score;
                }
            }
        }

        Ok(visible)
    }

    // Part 1 _could_ be O(n^2), The best part 2 could be is probably also
    // O(n^2), but my implementation is O(n^3). Instead of 2 x O(n^2), or worse,
    // O(n^2) + O(n^3), let's just solve both in one pass
    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.max_score)
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
