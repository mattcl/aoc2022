use std::{collections::BinaryHeap, fmt::Display, str::FromStr};

use anyhow::{anyhow, bail};
use aoc_helpers::generic::{prelude::GridLike, Grid, Location};
use aoc_plumbing::Problem;
use rustc_hash::FxHashMap;

const NORTH: u8 = 0b1;
const SOUTH: u8 = 0b10;
const WEST: u8 = 0b100;
const EAST: u8 = 0b1000;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Tile {
    Wall,
    Empty,
    Blizzard(u8),
    Person,
}

/// What the grid looked like at this point in time.
#[derive(Debug, Clone)]
pub struct Snapshot {
    time: usize,
    grid: Grid<Tile>,
}

impl Display for Snapshot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.locations.iter() {
            let mut s = String::with_capacity(self.grid.cols);
            for tile in row.iter() {
                let ch = match tile {
                    Tile::Wall => '#',
                    Tile::Empty => '.',
                    Tile::Blizzard(_) => 'B',
                    _ => unreachable!(),
                };
                s.push(ch);
            }
            writeln!(f, "{}", s)?;
        }

        writeln!(f, "-----------")
    }
}

impl Snapshot {
    pub fn from_initial_grid(initial_state: &Grid<Tile>, template: &Grid<Tile>) -> Self {
        let mut next = template.clone();

        for row in 0..initial_state.rows {
            for col in 0..initial_state.cols {
                match initial_state.locations[row][col] {
                    Tile::Blizzard(v) => {
                        Snapshot::adjust_blizzard(&Location::new(row, col), v, &mut next);
                    }
                    _ => {}
                }
            }
        }

        Self {
            time: 1,
            grid: next,
        }
    }

    /// Basically whether or not this location would be a valid move
    pub fn is_open(&self, location: &Location) -> bool {
        self.grid
            .get(location)
            .map(|tile| matches!(tile, Tile::Empty))
            .unwrap_or_default()
    }

    /// Calculate the next snapshot using this one and the given template.
    pub fn next(&self, template: &Grid<Tile>) -> Self {
        let mut next = template.clone();

        for row in 0..self.grid.rows {
            for col in 0..self.grid.cols {
                match self.grid.locations[row][col] {
                    Tile::Blizzard(v) => {
                        Snapshot::adjust_blizzard(&Location::new(row, col), v, &mut next);
                    }
                    _ => {}
                }
            }
        }

        Self {
            time: self.time + 1,
            grid: next,
        }
    }

    /// For the blizzard tiles, they hold the information about which blizards
    /// exist at a given spot, and from this we can propagate to all the valid
    /// next locations.
    pub fn adjust_blizzard(location: &Location, blizzard_map: u8, grid: &mut Grid<Tile>) {
        if blizzard_map & NORTH > 0 {
            // this should always be able to find a north, since the top row is
            // wall
            let new_loc = if location.row == 1 {
                Location::new(grid.rows - 2, location.col)
            } else {
                location.north().unwrap()
            };

            if let Some(tile) = grid.get_mut(&new_loc) {
                match tile {
                    Tile::Blizzard(ref v) => *tile = Tile::Blizzard(v | NORTH),
                    Tile::Empty => *tile = Tile::Blizzard(NORTH),
                    _ => {}
                }
            }
        }

        if blizzard_map & SOUTH > 0 {
            // this should always be able to find a north, since the top row is
            // wall
            let new_loc = if location.row == grid.rows - 2 {
                Location::new(1, location.col)
            } else {
                location.south().unwrap()
            };

            if let Some(tile) = grid.get_mut(&new_loc) {
                match tile {
                    Tile::Blizzard(ref v) => *tile = Tile::Blizzard(v | SOUTH),
                    Tile::Empty => *tile = Tile::Blizzard(SOUTH),
                    _ => {}
                }
            }
        }

        if blizzard_map & WEST > 0 {
            // this should always be able to find a north, since the top row is
            // wall
            let new_loc = if location.col == 1 {
                Location::new(location.row, grid.cols - 2)
            } else {
                location.west().unwrap()
            };

            if let Some(tile) = grid.get_mut(&new_loc) {
                match tile {
                    Tile::Blizzard(ref v) => *tile = Tile::Blizzard(v | WEST),
                    Tile::Empty => *tile = Tile::Blizzard(WEST),
                    _ => {}
                }
            }
        }

        if blizzard_map & EAST > 0 {
            // this should always be able to find a north, since the top row is
            // wall
            let new_loc = if location.col == grid.cols - 2 {
                Location::new(location.row, 1)
            } else {
                location.east().unwrap()
            };

            if let Some(tile) = grid.get_mut(&new_loc) {
                match tile {
                    Tile::Blizzard(ref v) => *tile = Tile::Blizzard(v | EAST),
                    Tile::Empty => *tile = Tile::Blizzard(EAST),
                    _ => {}
                }
            }
        }
    }
}

/// The plan is to keep a timeline of grid states so that we don't have to
/// recalculate these as we're searching different possibilities for different
/// times.
#[derive(Debug, Clone)]
pub struct Timeline {
    snapshots: Vec<Snapshot>,
}

impl Timeline {
    pub fn new(initial_state: &Grid<Tile>, template: &Grid<Tile>) -> Self {
        let mut snapshots = Vec::new();
        snapshots.push(Snapshot::from_initial_grid(initial_state, template));

        Self { snapshots }
    }

    /// If we don't already have a snapshot for the specified minute, simulate
    /// from the snapshot we have to the specified minute.
    pub fn simulate_to(&mut self, minute: usize, template: &Grid<Tile>) {
        for _ in self.snapshots.len()..minute {
            let last = self.snapshots.last().unwrap();
            self.snapshots.push(last.next(template));
        }
    }

    /// Get the snapshot for a given minute.
    pub fn get(&self, minute: usize) -> Option<&Snapshot> {
        self.snapshots.get(minute - 1)
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct State {
    location: Location,
    minute: usize,
    cost: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| other.minute.cmp(&self.minute))
            .then_with(|| self.location.cmp(&other.location))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
pub struct BlizzardBasin {
    grid: Grid<Tile>,
    next_template: Grid<Tile>,
    start: Location,
    end: Location,
}

impl BlizzardBasin {
    // pretty starndard dijkstra, haven't decided on a cost fn yet to make it A*
    pub fn best_time(
        &self,
        start_time: usize,
        start: &Location,
        end: &Location,
        timeline: &mut Timeline,
    ) -> Result<usize, anyhow::Error> {
        let mut cache: FxHashMap<(Location, usize), usize> = FxHashMap::default();

        let mut heap = BinaryHeap::new();

        let start = State {
            location: *start,
            minute: start_time,
            cost: 0,
        };

        cache.insert((self.start, start_time), 0);
        heap.push(start);

        while let Some(State {
            location,
            minute,
            cost,
        }) = heap.pop()
        {
            if location == *end {
                return Ok(minute);
            }

            if cost > *cache.get(&(location, minute)).unwrap_or(&usize::MAX) {
                continue;
            }

            // let's see what it would look like on the next step
            timeline.simulate_to(minute + 1, &self.next_template);
            // we know this exists now if it didn't before
            let snapshot = timeline.get(minute + 1).unwrap();

            if let Some(loc) = location.north() {
                self.check_location(loc, minute, cost, snapshot, &mut heap, &mut cache);
            }

            if let Some(loc) = location.south() {
                self.check_location(loc, minute, cost, snapshot, &mut heap, &mut cache);
            }

            if let Some(loc) = location.east() {
                self.check_location(loc, minute, cost, snapshot, &mut heap, &mut cache);
            }

            if let Some(loc) = location.west() {
                self.check_location(loc, minute, cost, snapshot, &mut heap, &mut cache);
            }

            // we can only wait if our current location would be open for the
            // next minute
            if snapshot.is_open(&location) {
                self.check_location(location, minute, cost, snapshot, &mut heap, &mut cache);
            }
        }

        bail!("Could not find a path")
    }

    fn check_location(
        &self,
        location: Location,
        minute: usize,
        cost: usize,
        snapshot: &Snapshot,
        heap: &mut BinaryHeap<State>,
        cache: &mut FxHashMap<(Location, usize), usize>,
    ) {
        if snapshot.is_open(&location) {
            let next = State {
                location,
                minute: minute + 1,
                cost: cost + 1,
            };

            if next.cost
                < *cache
                    .get(&(next.location, next.minute))
                    .unwrap_or(&usize::MAX)
            {
                cache.insert((location, next.minute), next.cost);
                heap.push(next);
            }
        }
    }
}

impl FromStr for BlizzardBasin {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.trim().lines().collect();
        let height = lines.len();
        let width = lines
            .get(0)
            .ok_or_else(|| anyhow!("Input has no lines"))?
            .chars()
            .count();

        let mut tiles = vec![vec![Tile::Empty; width]; height];
        let mut template = vec![vec![Tile::Empty; width]; height];

        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                let t = match ch {
                    '#' => Tile::Wall,
                    '.' => Tile::Empty,
                    '>' => Tile::Blizzard(EAST),
                    '<' => Tile::Blizzard(WEST),
                    '^' => Tile::Blizzard(NORTH),
                    'v' => Tile::Blizzard(SOUTH),
                    _ => bail!("Invalid char: {}", ch),
                };

                tiles[row][col] = t;

                if t == Tile::Wall {
                    template[row][col] = t;
                }
            }
        }

        let mut start = Location::default();
        let mut end = Location::default();

        for (idx, tile) in tiles[0].iter().enumerate() {
            if tile == &Tile::Empty {
                start.col = idx;
                break;
            }
        }

        for (idx, tile) in tiles[tiles.len() - 1].iter().enumerate() {
            if tile == &Tile::Empty {
                end.row = tiles.len() - 1;
                end.col = idx;
                break;
            }
        }

        let grid = Grid::new(tiles);
        let next_template = Grid::new(template);

        Ok(Self {
            grid,
            next_template,
            start,
            end,
        })
    }
}

impl Problem for BlizzardBasin {
    const DAY: usize = 24;
    const TITLE: &'static str = "blizzard basin";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        let mut timeline = Timeline::new(&self.grid, &self.next_template);
        self.best_time(0, &self.start, &self.end, &mut timeline)
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        let mut timeline = Timeline::new(&self.grid, &self.next_template);
        let t = self.best_time(0, &self.start, &self.end, &mut timeline)?;
        let t2 = self.best_time(t, &self.end, &self.start, &mut timeline)?;
        self.best_time(t2, &self.start, &self.end, &mut timeline)
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
        let solution = BlizzardBasin::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(343, 960));
    }

    #[test]
    fn example() {
        let input = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";
        let solution = BlizzardBasin::solve(input).unwrap();
        assert_eq!(solution, Solution::new(18, 54));
    }
}
