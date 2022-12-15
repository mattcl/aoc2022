use std::{fmt::Display, str::FromStr};

use aoc_helpers::generic::{prelude::GridLike, Bound2D, Grid, Location};
use aoc_plumbing::Problem;
use nom::{
    bytes::complete::tag, character::complete::multispace1, multi::separated_list1,
    sequence::separated_pair, IResult,
};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Tile {
    Source,
    Sand,
    FlowingSand,
    Rock,
    Air,
}

impl Tile {
    pub fn as_char(&self) -> char {
        match self {
            Self::Source => '+',
            Self::Sand => 'o',
            Self::FlowingSand => '~',
            Self::Rock => '#',
            Self::Air => '.',
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PathDesc {
    locations: Vec<Location>,
}

impl PathDesc {
    pub fn locations(&self) -> impl Iterator<Item = Location> + '_ {
        self.locations
            .windows(2)
            .map(|window| {
                let min_row = window[0].row.min(window[1].row);
                let max_row = window[0].row.max(window[1].row);
                let min_col = window[0].col.min(window[1].col);
                let max_col = window[0].col.max(window[1].col);

                (min_row..=max_row)
                    .map(move |row| (min_col..=max_col).map(move |col| Location::new(row, col)))
                    .flatten()
            })
            .flatten()
    }
}

fn location_parser(input: &str) -> IResult<&str, Location> {
    let (input, (x, y)) = separated_pair(
        nom::character::complete::u64,
        nom::character::complete::char(','),
        nom::character::complete::u64,
    )(input)?;
    Ok((input, Location::new(y as usize, x as usize)))
}

fn path_desc_parser(input: &str) -> IResult<&str, PathDesc> {
    let (input, locations) = separated_list1(tag(" -> "), location_parser)(input)?;
    Ok((input, PathDesc { locations }))
}

fn paths_parser(input: &str) -> IResult<&str, Vec<PathDesc>> {
    separated_list1(multispace1, path_desc_parser)(input)
}

#[derive(Debug, Clone)]
pub struct RegolithReservoir {
    grid: Grid<Tile>,
    source: Location,
    bounds: Bound2D<usize>,
    sand_count: usize,
}

impl Display for RegolithReservoir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.locations.iter() {
            writeln!(f, "{}", row.iter().map(|t| t.as_char()).collect::<String>())?;
        }

        Ok(())
    }
}

impl RegolithReservoir {
    pub fn fill(&mut self) {
        let cur = self.source;
        self.fill_recur(&cur);
    }

    fn fill_recur(&mut self, cur: &Location) -> Tile {
        // south first, by filling in beneath us if it's air
        match cur.south().and_then(|s| {
            self.grid.get(&s).copied().map(|t| {
                if t == Tile::Air {
                    self.fill_recur(&s)
                } else {
                    t
                }
            })
        }) {
            Some(Tile::FlowingSand) | None => {
                // if the thing immediately below me is flowing sand,
                // we're done and we're flowing sand
                self.grid.set(cur, Tile::FlowingSand);
                return Tile::FlowingSand;
            }
            Some(Tile::Sand) | Some(Tile::Rock) => {
                // if the thing immediately below me is sand or rock, we need
                // to check the left/right sides
            }
            _ => {}
        }

        // We check to the south west second
        if !self.process_diagonal(cur, |x| x.south_west()) {
            self.grid.set(cur, Tile::FlowingSand);
            return Tile::FlowingSand;
        }

        // then finally to the south east
        if !self.process_diagonal(cur, |x| x.south_east()) {
            self.grid.set(cur, Tile::FlowingSand);
            return Tile::FlowingSand;
        }

        self.sand_count += 1;

        match self.grid.get(cur) {
            Some(Tile::Source) => Tile::Source,
            _ => {
                self.grid.set(cur, Tile::Sand);
                Tile::Sand
            }
        }
    }

    /// returns false if we ended up as flowing sand
    fn process_diagonal(
        &mut self,
        cur: &Location,
        next_fn: impl Fn(Location) -> Option<Location>,
    ) -> bool {
        let mut origin = *cur;
        while let Some(loc) = next_fn(origin) {
            match self.grid.get(&loc) {
                // if the tile on the diagonal is air, recurse with the tile
                // under that to fill beneath that tile
                Some(Tile::Air) => {
                    match self.fill_recur(&loc) {
                        Tile::Sand => {}
                        Tile::FlowingSand => {
                            return false;
                        }
                        _ => {
                            unreachable!("Not possible")
                        }
                    }

                    origin = loc;
                }
                Some(Tile::Sand) | Some(Tile::Rock) | Some(Tile::Source) => {
                    return true;
                }
                _ => {
                    // we've hit a wall or the bottom or flowing sand while
                    // moving, so we're done
                    return false;
                }
            }
        }

        false
    }

    pub fn fill_infinite(&mut self) {
        let cur = self.source;
        self.fill_infinite_recur(&cur);
    }

    fn fill_infinite_recur(&mut self, cur: &Location) -> Tile {
        // south first
        match cur.south().and_then(|s| {
            self.grid.get(&s).copied().map(|t| {
                if t == Tile::Air {
                    self.fill_infinite_recur(&s)
                } else {
                    t
                }
            })
        }) {
            Some(Tile::FlowingSand) => {
                // if the thing immediately below me is flowing sand,
                // we're done and we're flowing sand
                self.grid.set(cur, Tile::FlowingSand);
                return Tile::FlowingSand;
            }
            Some(Tile::Sand) | Some(Tile::Rock) | None => {
                // if the thing immediately below me is sand or rock, we need
                // to check the left/right sides
            }
            _ => {}
        }

        // We check to the south west next
        if !self.process_infinite_diagonal(cur, |x| x.south_west()) {
            self.grid.set(cur, Tile::FlowingSand);
            return Tile::FlowingSand;
        }

        // then finally to the south east
        if !self.process_infinite_diagonal(cur, |x| x.south_east()) {
            self.grid.set(cur, Tile::FlowingSand);
            return Tile::FlowingSand;
        }

        self.sand_count += 1;

        match self.grid.get(cur) {
            Some(Tile::Source) => Tile::Source,
            _ => {
                self.grid.set(cur, Tile::Sand);
                Tile::Sand
            }
        }
    }

    /// returns false if we ended up as flowing sand
    fn process_infinite_diagonal(
        &mut self,
        cur: &Location,
        next_fn: impl Fn(Location) -> Option<Location>,
    ) -> bool {
        let mut origin = *cur;
        while let Some(loc) = next_fn(origin) {
            match self.grid.get(&loc) {
                // if the tile on the diagonal is air, recurse with the tile
                // under that to fill beneath that tile
                Some(Tile::Air) => {
                    match self.fill_infinite_recur(&loc) {
                        Tile::Sand => { /* nothing */ }
                        Tile::FlowingSand => {
                            return false;
                        }
                        _ => {
                            unreachable!("Not possible")
                        }
                    }

                    origin = loc;
                }
                // we include None for this, since we assume the edges are
                // infinite and we will fill them with sand
                Some(Tile::Sand) | Some(Tile::Rock) | Some(Tile::Source) | None => {
                    return true;
                }
                _ => {
                    // we've hit flowing sand while moving, so we're done
                    return false;
                }
            }
        }

        // true here because the edges of our grid are considered eventaully
        // to be sand
        true
    }
}

impl FromStr for RegolithReservoir {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, paths) = paths_parser(s.trim()).map_err(|e| e.to_owned())?;

        // calculate our actual bounds
        let mut bounds: Bound2D<usize> = Bound2D::minmax();
        for desc in paths.iter() {
            for loc in desc.locations.iter() {
                if loc.row < bounds.min_y {
                    bounds.min_y = loc.row;
                }

                if loc.row > bounds.max_y {
                    bounds.max_y = loc.row;
                }

                if loc.col < bounds.min_x {
                    bounds.min_x = loc.col;
                }

                if loc.col > bounds.max_x {
                    bounds.max_x = loc.col;
                }
            }
        }

        // reshape the bounds to accomodate the source
        let bounds = Bound2D::new(
            500.min(bounds.min_x) - 1, // we have to include the source
            500.max(bounds.max_x) + 1, // we have to include the source
            0,                         // we have to include the source
            bounds.max_y + 1,          // this ended up being very fortunate
                                       // as part 2 wanted an extra row
        );

        let mut grid = Grid::new(vec![vec![Tile::Air; bounds.width()]; bounds.height()]);

        // translate all of the locations via the bounds
        for desc in paths {
            for loc in desc.locations() {
                grid.set(&bounds.translate(&loc), Tile::Rock);
            }
        }

        // insert the source
        let source = bounds.translate(&Location::new(0, 500));
        grid.set(&source, Tile::Source);

        Ok(Self {
            grid,
            source,
            bounds,
            sand_count: 0,
        })
    }
}

impl Problem for RegolithReservoir {
    const DAY: usize = 14;
    const TITLE: &'static str = "regolith reservoir";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        let mut working = self.clone();
        working.fill();

        Ok(working.sand_count)
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        let mut working = self.clone();
        working.fill_infinite();

        // if we know our max y depth, we know the triangle should be max y in
        // each direction, so we can math our way into the quanity of sand
        // beyond what we can see. We are making the assumption that we have
        // some space on either side of the source, which is fixed at 500
        //
        // west side:
        let offset = 500 - self.bounds.min_x;
        if offset < self.bounds.max_y {
            let delta = self.bounds.max_y - offset;
            working.sand_count += (delta * (delta + 1)) / 2;
        }

        // and the east side:
        let offset = self.bounds.max_x - 500;
        if offset < self.bounds.max_y {
            let delta = self.bounds.max_y - offset;
            working.sand_count += (delta * (delta + 1)) / 2;
        }

        Ok(working.sand_count)
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
        let solution = RegolithReservoir::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(1001, 27976));
    }

    #[test]
    fn example() {
        let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
        let solution = RegolithReservoir::solve(input).unwrap();
        assert_eq!(solution, Solution::new(24, 93));
    }
}
