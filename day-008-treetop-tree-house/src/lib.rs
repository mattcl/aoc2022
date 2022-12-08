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

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct TreetopTreeHouse {
    grid: Vec<Vec<u8>>,
    // this ends up being roughly 4x the memory of storing the digits alone
    // because 16 digits could have fit in the u128, and we store 10 u128s per
    // row and 10 per col
    row_maps: Vec<Vec<u128>>,
    col_maps: Vec<Vec<u128>>,
    max_score: usize,
    width: usize,
    height: usize,
}

impl TreetopTreeHouse {
    /// Make a VisualRange for the given row/col.
    ///
    /// This solution came to me in a dream. There's a saner solution that
    /// lanjian pointed out that uses monotonic stacks, but I think this is
    /// faster and I actually understand this one.
    ///
    /// Recall that we have row_maps and col maps, which store an entry per row
    /// (or col), where each entry stores a mapping of height -> u128, where the
    /// 1's in the binary representation correspond to the locations of trees in
    /// that row or column that are either equal to or greater than the given
    /// height.
    ///
    /// Knowing this, and knowing our current row and column, we can fetch our
    /// height from the grid, fetch the appropriate row map and the appropriate
    /// column map, and using those values, calculate if we can see the edge and
    /// how far we can see in each direction without additional memory lookups
    /// or implicit looping.
    ///
    /// The theory here is that we can make a single left or right shift, then a
    /// call to `leading_zeros` or `trailing_zeros` to determine the distance to
    /// the nearest tree that would be greater than or equal to us. Most
    /// processor architectures have special instructions for trailing zeros,
    /// which will make that faster than if we were looping.
    fn compute_bin_range(&self, row: usize, col: usize) -> VisualRange {
        let extra_row_bits = 128 - self.height;
        let extra_col_bits = 128 - self.width;
        let mut vr = VisualRange::default();

        if row > self.height || col > self.width {
            return vr;
        }

        let digit = self.grid[row][col];

        if digit == 0 {
            vr.score = 4;
            return vr;
        }

        // grab this digit's map
        let row_map = self.row_maps[row][(digit - 1) as usize];

        // we want to shift by the current col + 1, which should leave us a
        // number representing the view to the _right_ (map is reversed)
        let shifted = row_map >> (col + 1);
        if shifted == 0 {
            // we can see the right edge
            vr.seen_edge = true;
            vr.score = self.width - col - 1;
        } else {
            // otherwise we know the number of zeros is how far we could see - 1
            vr.score = shifted.trailing_zeros() as usize + 1;
        }

        // we now want to know if we can see the edge to the _left_, which is
        // trickier because we're going to be shifting left
        let shifted = row_map << (self.width - col + extra_col_bits);
        if shifted == 0 {
            // we can see the left edge
            vr.seen_edge = true;
            vr.score *= col;
        } else {
            vr.score *= shifted.leading_zeros() as usize + 1;
        }

        // now we do the same for the columns
        let col_map = self.col_maps[col][(digit - 1) as usize];

        // we want to shift by the current row + 1, which should leave us a
        // number representing the view _down_ (map is reversed)
        let shifted = col_map >> (row + 1);
        if shifted == 0 {
            // we can see the right edge
            vr.seen_edge = true;
            vr.score *= self.height - row - 1;
        } else {
            // otherwise we know the number of zeros is how far we could see - 1
            vr.score *= shifted.trailing_zeros() as usize + 1;
        }

        // and, lastly, back up
        let shifted = col_map << (self.height - row + extra_row_bits);
        if shifted == 0 {
            // we can see the left edge
            vr.seen_edge = true;
            vr.score *= row;
        } else {
            vr.score *= shifted.leading_zeros() as usize + 1;
        }

        vr
    }
}

impl FromStr for TreetopTreeHouse {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dim = s.lines().count();
        if dim > 128 {
            bail!("Sorry, can only handle grids of at most 128x128");
        }
        let mut grid = Vec::with_capacity(dim);

        // so we're not going to allocate for the 0, because those can NEVER be
        // seen unless on the edge and they always have a score of at most 4
        let mut row_maps = vec![vec![0u128; 9]; dim];
        let mut col_maps = vec![vec![0u128; 9]; dim];

        let mut row_mask = 1u128;
        for (row, line) in s.trim().lines().enumerate() {
            let mut new_row = Vec::with_capacity(dim);
            let mut col_mask = 1u128;
            for (col, ch) in line.trim().chars().enumerate() {
                let digit =
                    ch.to_digit(10)
                        .ok_or_else(|| anyhow!("Invalid digit: {}", ch))? as u8;
                new_row.push(digit);
                if digit > 0 {
                    // confusing naming, I realize, but the col mask is which
                    // bit in the integer for the row this digit corresponds to
                    row_maps[row][(digit - 1) as usize] |= col_mask;
                    col_maps[col][(digit - 1) as usize] |= row_mask;
                }
                col_mask <<= 1;
            }

            grid.push(new_row);

            row_mask <<= 1;
        }

        // We're going to make each row's digit's map represent all the locations
        // where there's another digit that's either equal to or larger than it
        // In essence, starting from a particular bit position and looking to
        // the left would be the trees equal or larger to you to the right (or
        // down) and looking to the bits to the right would be the trees equal
        // or larger to the left (or up). This should allow us to take advantage
        // of not having to fetch from system memory, as we can probably hold
        // two u128 bit values in the processor cache.
        //
        // Additionally, we can make a single left or right shift, then a call
        // to leading or trailing zeros to determine the distance to the nearest
        // tree that would be greater than or equal to us. Most processors have
        // special instructions for trailing zeros, which will make that VERY
        // fast.
        for row in 0..row_maps.len() {
            for digit in (0..8).rev() {
                row_maps[row][digit] |= row_maps[row][digit + 1];
            }
        }

        // and the same for columns
        for col in 0..col_maps.len() {
            for digit in (0..8).rev() {
                col_maps[col][digit] |= col_maps[col][digit + 1];
            }
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
            row_maps,
            col_maps,
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
                let vr = self.compute_bin_range(row, col);
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
