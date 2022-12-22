use std::str::FromStr;

use anyhow::{anyhow, bail};
use aoc_helpers::generic::{Grid, Location};
use aoc_plumbing::Problem;
use nom::{branch::alt, multi::many1, IResult};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Tile {
    Void,
    Open,
    Wall,
}

/// Instead of up/down/whatever, let's just use compass directions to not get
/// confused.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Facing {
    East = 0,
    South,
    West,
    North,
}

impl Facing {
    fn left(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::South => Self::East,
            Self::East => Self::North,
            Self::West => Self::South,
        }
    }

    fn right(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::South => Self::West,
            Self::East => Self::South,
            Self::West => Self::North,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Turn {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Instruction {
    Turn(Turn),
    Dist(usize),
}

fn parse_turn(input: &str) -> IResult<&str, Instruction> {
    let (input, ch) = nom::character::complete::one_of("LR")(input)?;
    match ch {
        'L' => Ok((input, Instruction::Turn(Turn::Left))),
        'R' => Ok((input, Instruction::Turn(Turn::Right))),
        _ => unreachable!("should not be possible because of nom match"),
    }
}

fn parse_dist(input: &str) -> IResult<&str, Instruction> {
    let (input, dist) = nom::character::complete::u64(input)?;
    Ok((input, Instruction::Dist(dist as usize)))
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((parse_turn, parse_dist))(input)
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(parse_instruction)(input)
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct CubePerson<const N: usize> {
    location: Location,
    facing: Facing,
}

impl<const N: usize> CubePerson<N> {
    pub fn follow(&mut self, map: &MonkeyMap, instruction: &Instruction) {
        match instruction {
            Instruction::Dist(dist) => {
                for _ in 0..*dist {
                    let moved = match self.facing {
                        Facing::North => self.move_north(map),
                        Facing::South => self.move_south(map),
                        Facing::East => self.move_east(map),
                        Facing::West => self.move_west(map),
                    };

                    if !moved {
                        break;
                    }
                }
            }
            Instruction::Turn(turn) => self.turn(turn),
        }
    }

    pub fn turn(&mut self, turn: &Turn) {
        match turn {
            Turn::Left => self.facing = self.facing.left(),
            Turn::Right => self.facing = self.facing.right(),
        }
    }

    pub fn move_east(&mut self, map: &MonkeyMap) -> bool {
        // if our col is on an edge
        if self.location.col % N == N - 1 {
            return self.transition(&Facing::East, map);
        }

        let next_col = self.location.col + 1;
        // check for wall
        if map.grid.locations[self.location.row][next_col] == Tile::Wall {
            return false;
        }

        self.location.col = next_col;
        true
    }

    pub fn move_west(&mut self, map: &MonkeyMap) -> bool {
        // if our col is on an edge
        if self.location.col % N == 0 {
            return self.transition(&Facing::West, map);
        }

        let next_col = self.location.col - 1;
        // check for wall
        if map.grid.locations[self.location.row][next_col] == Tile::Wall {
            return false;
        }

        self.location.col = next_col;
        true
    }

    pub fn move_north(&mut self, map: &MonkeyMap) -> bool {
        // if our col is on an edge
        if self.location.row % N == 0 {
            return self.transition(&Facing::North, map);
        }

        let next_row = self.location.row - 1;
        // check for wall
        if map.grid.locations[next_row][self.location.col] == Tile::Wall {
            return false;
        }

        self.location.row = next_row;
        true
    }

    pub fn move_south(&mut self, map: &MonkeyMap) -> bool {
        // if our col is on an edge
        if self.location.row % N == N - 1 {
            return self.transition(&Facing::South, map);
        }

        let next_row = self.location.row + 1;
        // check for wall
        if map.grid.locations[next_row][self.location.col] == Tile::Wall {
            return false;
        }

        self.location.row = next_row;
        true
    }

    /// helper for doing the cube wrapping
    fn transition(&mut self, direction: &Facing, map: &MonkeyMap) -> bool {
        // we need to figure out where we would end up and all that jazz
        let (cur_region, cur_region_loc) = Region::<N>::make_region_location(&self.location);
        let (new_region, new_facing, new_region_loc) =
            cur_region.transition(direction, &cur_region_loc);
        let new_loc = new_region.make_global_location(&new_region_loc);

        if map.grid.locations[new_loc.row][new_loc.col] == Tile::Wall {
            return false;
        }

        self.location = new_loc;
        self.facing = new_facing;
        return true;
    }

    pub fn password(&self) -> usize {
        (self.location.row + 1) * 1000 + (self.location.col + 1) * 4 + self.facing as usize
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Region<const N: usize> {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

impl<const N: usize> Region<N> {
    /// Take a global grid location and figure out which region and region-space
    /// location it represents.
    pub fn make_region_location(location: &Location) -> (Self, Location) {
        if location.row < N {
            if location.col < 2 * N {
                // region two
                (Self::Two, (location.row, location.col - N).into())
            } else {
                // region one
                (Self::One, (location.row, location.col - 2 * N).into())
            }
        } else if location.row < 2 * N {
            // we can only be three
            (Self::Three, (location.row - N, location.col - N).into())
        } else if location.row < 3 * N {
            if location.col < N {
                // region five
                (Self::Five, (location.row - 2 * N, location.col).into())
            } else {
                // region four
                (Self::Four, (location.row - 2 * N, location.col - N).into())
            }
        } else {
            // six
            (Self::Six, (location.row - 3 * N, location.col).into())
        }
    }

    /// Take a region-space location and turn it into a global grid location
    pub fn make_global_location(&self, location: &Location) -> Location {
        match self {
            Self::One => (location.row, location.col + 2 * N).into(),
            Self::Two => (location.row, location.col + N).into(),
            Self::Three => (location.row + N, location.col + N).into(),
            Self::Four => (location.row + 2 * N, location.col + N).into(),
            Self::Five => (location.row + 2 * N, location.col).into(),
            Self::Six => (location.row + 3 * N, location.col).into(),
        }
    }

    /// Take a direction to move and a location in region-space and turn that
    /// into a destination region, new facing, and destination location in that
    /// region's space.
    pub fn transition(
        &self,
        dir: &Facing,
        in_region_location: &Location,
    ) -> (Self, Facing, Location) {
        match self {
            Self::One => match dir {
                Facing::North => (
                    Region::Six,
                    Facing::North,
                    Location::new(N - 1, in_region_location.col),
                ),
                Facing::South => (
                    Region::Three,
                    Facing::West,
                    Location::new(in_region_location.col, N - 1),
                ),
                Facing::East => (
                    Region::Four,
                    Facing::West,
                    Location::new(N - 1 - in_region_location.row, N - 1),
                ),
                Facing::West => (
                    Region::Two,
                    Facing::West,
                    Location::new(in_region_location.row, N - 1),
                ),
            },
            Self::Two => match dir {
                Facing::North => (
                    Region::Six,
                    Facing::East,
                    Location::new(in_region_location.col, 0),
                ),
                Facing::South => (
                    Region::Three,
                    Facing::South,
                    Location::new(0, in_region_location.col),
                ),
                Facing::East => (
                    Region::One,
                    Facing::East,
                    Location::new(in_region_location.row, 0),
                ),
                Facing::West => (
                    Region::Five,
                    Facing::East,
                    Location::new(N - 1 - in_region_location.row, 0),
                ),
            },
            Self::Three => match dir {
                Facing::North => (
                    Region::Two,
                    Facing::North,
                    Location::new(N - 1, in_region_location.col),
                ),
                Facing::South => (
                    Region::Four,
                    Facing::South,
                    Location::new(0, in_region_location.col),
                ),
                Facing::East => (
                    Region::One,
                    Facing::North,
                    Location::new(N - 1, in_region_location.row),
                ),
                Facing::West => (
                    Region::Five,
                    Facing::South,
                    Location::new(0, in_region_location.row),
                ),
            },
            Self::Four => match dir {
                Facing::North => (
                    Region::Three,
                    Facing::North,
                    Location::new(N - 1, in_region_location.col),
                ),
                Facing::South => (
                    Region::Six,
                    Facing::West,
                    Location::new(in_region_location.col, N - 1),
                ),
                Facing::East => (
                    Region::One,
                    Facing::West,
                    Location::new(N - 1 - in_region_location.row, N - 1),
                ),
                Facing::West => (
                    Region::Five,
                    Facing::West,
                    Location::new(in_region_location.row, N - 1),
                ),
            },
            Self::Five => match dir {
                Facing::North => (
                    Region::Three,
                    Facing::East,
                    Location::new(in_region_location.col, 0),
                ),
                Facing::South => (
                    Region::Six,
                    Facing::South,
                    Location::new(0, in_region_location.col),
                ),
                Facing::East => (
                    Region::Four,
                    Facing::East,
                    Location::new(in_region_location.row, 0),
                ),
                Facing::West => (
                    Region::Two,
                    Facing::East,
                    Location::new(N - 1 - in_region_location.row, 0),
                ),
            },
            Self::Six => match dir {
                Facing::North => (
                    Region::Five,
                    Facing::North,
                    Location::new(N - 1, in_region_location.col),
                ),
                Facing::South => (
                    Region::One,
                    Facing::South,
                    Location::new(0, in_region_location.col),
                ),
                Facing::East => (
                    Region::Four,
                    Facing::North,
                    Location::new(N - 1, in_region_location.row),
                ),
                Facing::West => (
                    Region::Two,
                    Facing::South,
                    Location::new(0, in_region_location.row),
                ),
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Person {
    location: Location,
    facing: Facing,
}

impl Person {
    pub fn follow(&mut self, map: &MonkeyMap, instruction: &Instruction) {
        match instruction {
            Instruction::Dist(dist) => {
                for _ in 0..*dist {
                    let moved = match self.facing {
                        Facing::North => self.move_north(map),
                        Facing::South => self.move_south(map),
                        Facing::East => self.move_east(map),
                        Facing::West => self.move_west(map),
                    };

                    if !moved {
                        break;
                    }
                }
            }
            Instruction::Turn(turn) => self.turn(turn),
        }
    }

    pub fn turn(&mut self, turn: &Turn) {
        match turn {
            Turn::Left => self.facing = self.facing.left(),
            Turn::Right => self.facing = self.facing.right(),
        }
    }

    pub fn move_east(&mut self, map: &MonkeyMap) -> bool {
        // get position
        let next_col = if self.location.col == map.lr_edges[self.location.row].1 {
            map.lr_edges[self.location.row].0
        } else {
            self.location.col + 1
        };

        // check for wall
        if map.grid.locations[self.location.row][next_col] == Tile::Wall {
            return false;
        }

        self.location.col = next_col;
        true
    }

    pub fn move_west(&mut self, map: &MonkeyMap) -> bool {
        // get position
        let next_col = if self.location.col == map.lr_edges[self.location.row].0 {
            map.lr_edges[self.location.row].1
        } else {
            self.location.col - 1
        };

        // check for wall
        if map.grid.locations[self.location.row][next_col] == Tile::Wall {
            return false;
        }

        self.location.col = next_col;
        true
    }

    pub fn move_north(&mut self, map: &MonkeyMap) -> bool {
        // get position
        let next_row = if self.location.row == map.tb_edges[self.location.col].0 {
            map.tb_edges[self.location.col].1
        } else {
            self.location.row - 1
        };

        // check for wall
        if map.grid.locations[next_row][self.location.col] == Tile::Wall {
            return false;
        }

        self.location.row = next_row;
        true
    }

    pub fn move_south(&mut self, map: &MonkeyMap) -> bool {
        // get position
        let next_row = if self.location.row == map.tb_edges[self.location.col].1 {
            map.tb_edges[self.location.col].0
        } else {
            self.location.row + 1
        };

        // check for wall
        if map.grid.locations[next_row][self.location.col] == Tile::Wall {
            return false;
        }

        self.location.row = next_row;
        true
    }

    pub fn password(&self) -> usize {
        (self.location.row + 1) * 1000 + (self.location.col + 1) * 4 + self.facing as usize
    }
}

#[derive(Debug, Clone)]
pub struct MonkeyMap {
    grid: Grid<Tile>,
    lr_edges: Vec<(usize, usize)>,
    tb_edges: Vec<(usize, usize)>,
    instructions: Vec<Instruction>,
}

impl MonkeyMap {
    pub fn password(&self) -> Result<usize, anyhow::Error> {
        // start facing right and in the first non-void open tile
        let mut start_col = self.lr_edges[0].0;

        // handle case where first tile is a wall
        while self.grid.locations[0][start_col] != Tile::Open {
            start_col += 1;
            if start_col > self.lr_edges[0].1 {
                bail!("First row does not have an open tile");
            }
        }

        let mut cur = Person {
            location: (0, start_col).into(),
            facing: Facing::East,
        };

        for inst in self.instructions.iter() {
            cur.follow(&self, inst);
        }

        Ok(cur.password())
    }

    pub fn cube_password(&self) -> Result<usize, anyhow::Error> {
        // start facing right and in the first non-void open tile
        let mut start_col = self.lr_edges[0].0;

        // handle case where first tile is a wall
        while self.grid.locations[0][start_col] != Tile::Open {
            start_col += 1;
            if start_col > self.lr_edges[0].1 {
                bail!("First row does not have an open tile");
            }
        }

        let mut cur: CubePerson<50> = CubePerson {
            location: (0, start_col).into(),
            facing: Facing::East,
        };

        for inst in self.instructions.iter() {
            cur.follow(&self, inst);
        }

        Ok(cur.password())
    }
}

impl FromStr for MonkeyMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("\n\n");
        let raw_map = parts.next().ok_or_else(|| anyhow!("Input missing map"))?;
        let lines = raw_map.lines().collect::<Vec<_>>();
        let width = lines
            .iter()
            .map(|line| line.len())
            .max()
            .ok_or_else(|| anyhow!("map is empty"))?;
        let height = lines.len();

        let mut raw_grid = vec![vec![Tile::Void; width]; height];
        let mut lr_edges = Vec::with_capacity(height);
        let mut tb_edges = Vec::with_capacity(width);

        for (row, line) in raw_map.lines().enumerate() {
            let mut min = width;
            let mut max = 0;
            for (col, ch) in line.chars().enumerate() {
                match ch {
                    '.' => raw_grid[row][col] = Tile::Open,
                    '#' => raw_grid[row][col] = Tile::Wall,
                    ' ' => continue,
                    _ => bail!("Invalid char: {}", ch),
                }

                if col < min {
                    min = col;
                }
                if col > max {
                    max = col;
                }
            }
            lr_edges.push((min, max));
        }

        for col in 0..width {
            let mut min = height;
            let mut max = 0;

            for row in 0..height {
                match raw_grid[row][col] {
                    Tile::Open | Tile::Wall => {
                        if row < min {
                            min = row;
                        }
                        if row > max {
                            max = row;
                        }
                    }
                    _ => {}
                }
            }

            tb_edges.push((min, max))
        }

        let (_, instructions) = parse_instructions(
            parts
                .next()
                .ok_or_else(|| anyhow!("missing instructions"))?,
        )
        .map_err(|e| e.to_owned())?;

        Ok(Self {
            grid: Grid::new(raw_grid),
            lr_edges,
            tb_edges,
            instructions,
        })
    }
}

impl Problem for MonkeyMap {
    const DAY: usize = 22;
    const TITLE: &'static str = "monkey map";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        self.password()
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        self.cube_password()
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
        let solution = MonkeyMap::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(75254, 108311));
    }

    // this is only a test for part one of the example input, on account of how
    // different the real input is laid out
    #[test]
    fn example() {
        let input = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
            ";
        let mut inst = MonkeyMap::instance(input).unwrap();
        assert_eq!(inst.part_one().unwrap(), 6032);
    }
}
