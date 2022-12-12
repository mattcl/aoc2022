use std::{collections::BinaryHeap, str::FromStr};

use anyhow::anyhow;
use aoc_helpers::generic::{
    pathing::{DNode, DefaultLocationCache},
    prelude::*,
    Grid, Location,
};
use aoc_plumbing::{bits::char_to_num, Problem};

#[derive(Debug, Clone)]
pub struct HillClimbingAlgorithm {
    grid: Grid<char>,
    end: Location,
}

impl HillClimbingAlgorithm {
    pub fn shortest_path(&self, begin: &Location, end: char) -> Option<usize> {
        let mut cache: DefaultLocationCache<usize> =
            DefaultLocationCache::new(self.grid.size(), self.grid.cols());
        let mut heap = BinaryHeap::new();

        let start = DNode {
            id: *begin,
            cost: 0,
        };
        cache.cache_set(&start.id, 0);
        heap.push(start);

        while let Some(DNode { id, cost }) = heap.pop() {
            // the unwrap is safe because we never insert anything not in the grid
            let cur_val = self.grid.get(&id).unwrap();

            if *cur_val == end {
                return Some(cost);
            }

            if cost > cache.cache_get(&id) {
                continue;
            }

            // the unwrap is safe because we never insert anything not in the grid
            let numeric_current = match self.grid.get(&id).unwrap() {
                'E' => char_to_num('z'),
                'S' => char_to_num('a'),
                x => char_to_num(*x),
            };

            for edge in id.orthogonal_neighbors() {
                if let Some(neighbor_value) = self.grid.get(&edge) {
                    let numeric_neighbor = match neighbor_value {
                        'E' => char_to_num('z'),
                        'S' => char_to_num('a'),
                        _ => char_to_num(*neighbor_value),
                    };

                    if numeric_neighbor >= numeric_current
                        || numeric_current - numeric_neighbor == 1
                    {
                        let next = DNode {
                            id: edge,
                            cost: cost + 1,
                        };

                        if next.cost < cache.cache_get(&next.id) {
                            cache.cache_set(&next.id, next.cost);
                            heap.push(next);
                        }
                    }
                }
            }
        }

        None
    }
}

impl FromStr for HillClimbingAlgorithm {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vals = s
            .trim()
            .lines()
            .map(|l| l.trim().chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut end = Location::default();
        'outer: for row in 0..vals.len() {
            for col in 0..vals[0].len() {
                let v = vals[row][col];
                if v == 'E' {
                    end.row = row;
                    end.col = col;
                    break 'outer;
                }
            }
        }

        Ok(Self {
            grid: Grid::new(vals),
            end,
        })
    }
}

impl Problem for HillClimbingAlgorithm {
    const DAY: usize = 12;
    const TITLE: &'static str = "hill climbing algorithm";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        self.shortest_path(&self.end, 'S')
            .ok_or_else(|| anyhow!("no path found"))
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        self.shortest_path(&self.end, 'a')
            .ok_or_else(|| anyhow!("no path found"))
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
        let solution = HillClimbingAlgorithm::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(484, 478));
    }

    #[test]
    fn example() {
        let input = "
            Sabqponm
            abcryxxl
            accszExk
            acctuvwj
            abdefghi
            ";
        let solution = HillClimbingAlgorithm::solve(input).unwrap();
        assert_eq!(solution, Solution::new(31, 29));
    }
}
