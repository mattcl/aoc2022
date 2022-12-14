use std::{collections::BinaryHeap, str::FromStr};

use anyhow::anyhow;
use aoc_helpers::generic::{
    pathing::{DASTNode, DNode, DefaultLocationCache},
    prelude::*,
    Grid, Location,
};
use aoc_plumbing::{bits::char_to_num, Problem};

const E_MARKER: u8 = 30;
const S_MARKER: u8 = 44;

#[derive(Debug, Clone)]
pub struct HillClimbingAlgorithm {
    grid: Grid<u8>,
    end: Location,
}

impl HillClimbingAlgorithm {
    pub fn shortest_path(&self, begin: &Location, end: u8) -> Option<usize> {
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
            let numeric_current = match *cur_val {
                E_MARKER => char_to_num('z'),
                S_MARKER => char_to_num('a'),
                x => x,
            };

            for edge in id.orthogonal_neighbors() {
                if let Some(neighbor_value) = self.grid.get(&edge) {
                    let numeric_neighbor = match *neighbor_value {
                        E_MARKER => char_to_num('z'),
                        S_MARKER => char_to_num('a'),
                        x => x,
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

    pub fn shortest_path_known_destination(
        &self,
        begin: &Location,
        end: &Location,
    ) -> Option<usize> {
        let mut cache: DefaultLocationCache<usize> =
            DefaultLocationCache::new(self.grid.size(), self.grid.cols());
        let mut heap = BinaryHeap::new();

        let start = DASTNode {
            id: *begin,
            cost: 0,
            path: 0,
        };
        cache.cache_set(&start.id, 0);
        heap.push(start);

        while let Some(DASTNode { id, cost, path }) = heap.pop() {
            // the unwrap is safe because we never insert anything not in the grid
            let cur_val = self.grid.get(&id).unwrap();

            if id == *end {
                return Some(path);
            }

            if cost > cache.cache_get(&id) {
                continue;
            }

            // the unwrap is safe because we never insert anything not in the grid
            let numeric_current = match *cur_val {
                E_MARKER => char_to_num('z'),
                S_MARKER => char_to_num('a'),
                x => x,
            };

            for edge in id.orthogonal_neighbors() {
                if let Some(neighbor_value) = self.grid.get(&edge) {
                    let numeric_neighbor = match *neighbor_value {
                        E_MARKER => char_to_num('z'),
                        S_MARKER => char_to_num('a'),
                        x => x,
                    };

                    if numeric_neighbor >= numeric_current
                        || numeric_current - numeric_neighbor == 1
                    {
                        let next = DASTNode {
                            id: edge,
                            cost: cost + edge.manhattan_dist(end),
                            path: path + 1,
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
            .map(|l| {
                l.trim()
                    .chars()
                    .map(|ch| char_to_num(ch))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let mut end = Location::default();
        'outer: for row in 0..vals.len() {
            for col in 0..vals[0].len() {
                let v = vals[row][col];
                if v == E_MARKER {
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
        let mut end = Location::default();
        'outer: for row in 0..self.grid.rows() {
            for col in 0..self.grid.cols() {
                if self.grid.locations[row][col] == S_MARKER {
                    end.row = row;
                    end.col = col;
                    break 'outer;
                }
            }
        }

        self.shortest_path_known_destination(&self.end, &end)
            .ok_or_else(|| anyhow!("no path found"))
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        self.shortest_path(&self.end, char_to_num('a'))
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
