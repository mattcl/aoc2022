use std::str::FromStr;

use aoc_plumbing::Problem;
use nom::{character::complete::multispace1, multi::separated_list1, sequence::tuple, IResult};
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct Bounds {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
    min_z: i64,
    max_z: i64,
}

impl Bounds {
    pub fn minmax() -> Self {
        Self {
            min_x: i64::MAX,
            max_x: i64::MIN,
            min_y: i64::MAX,
            max_y: i64::MIN,
            min_z: i64::MAX,
            max_z: i64::MIN,
        }
    }

    pub fn does_not_contain(&self, cube: &Cube) -> bool {
        cube.x > self.max_x
            || cube.x < self.min_x
            || cube.y > self.max_y
            || cube.y < self.min_y
            || cube.z > self.max_z
            || cube.z < self.min_z
    }
}

const NEIGHBORS: [(i64, i64, i64); 6] = [
    (0, 0, 1),
    (0, 1, 0),
    (1, 0, 0),
    (0, 0, -1),
    (0, -1, 0),
    (-1, 0, 0),
];

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct Cube {
    x: i64,
    y: i64,
    z: i64,
}

impl Cube {
    pub fn neighbors(&self) -> impl Iterator<Item = Cube> + '_ {
        NEIGHBORS.iter().map(|(dx, dy, dz)| Cube {
            x: self.x + dx,
            y: self.y + dy,
            z: self.z + dz,
        })
    }
}

fn parse_cube(input: &str) -> IResult<&str, Cube> {
    let (input, (x, _, y, _, z)) = tuple((
        nom::character::complete::i64,
        nom::character::complete::char(','),
        nom::character::complete::i64,
        nom::character::complete::char(','),
        nom::character::complete::i64,
    ))(input)?;

    Ok((input, Cube { x, y, z }))
}

fn parse_cubes(input: &str) -> IResult<&str, Vec<Cube>> {
    separated_list1(multispace1, parse_cube)(input)
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BoilingBoulders {
    // this is going to be slow
    cubes: FxHashSet<Cube>,
    bounds: Bounds,
}

impl FromStr for BoilingBoulders {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, raw_cubes) = parse_cubes(s).map_err(|e| e.to_owned())?;

        let mut bounds = Bounds::minmax();
        let mut cubes = FxHashSet::default();
        for cube in raw_cubes {
            if bounds.min_x > cube.x {
                bounds.min_x = cube.x;
            }

            if bounds.max_x < cube.x {
                bounds.max_x = cube.x;
            }

            if bounds.min_y > cube.y {
                bounds.min_y = cube.y;
            }

            if bounds.max_y < cube.y {
                bounds.max_y = cube.y;
            }

            if bounds.min_z > cube.z {
                bounds.min_z = cube.z;
            }

            if bounds.max_z < cube.z {
                bounds.max_z = cube.z;
            }

            cubes.insert(cube);
        }

        Ok(Self { cubes, bounds })
    }
}

impl BoilingBoulders {
    pub fn connects_to_outside(
        &self,
        loc: &Cube,
        outside_map: &mut FxHashMap<Cube, bool>,
        seen: &mut FxHashSet<Cube>,
    ) -> bool {
        if let Some(v) = outside_map.get(loc) {
            return *v;
        }

        seen.insert(*loc);

        for neighbor in loc.neighbors() {
            // we don't care about examining locations that are not air
            if self.cubes.contains(&neighbor) {
                continue;
            }

            if seen.contains(&neighbor) {
                continue;
            }

            // if we've already determined the status of this cube, everything
            // else must be this value
            if let Some(v) = outside_map.get(&neighbor).copied() {
                for cube in seen.iter() {
                    outside_map.insert(*cube, v);
                }
                return v;
            }

            // we know we've found an edge, so everytyhing we seen touches the
            // outside
            if self.bounds.does_not_contain(&neighbor) {
                for cube in seen.iter() {
                    outside_map.insert(*cube, true);
                }
                return true;
            }

            // otherwise, we need to recur
            if self.connects_to_outside(&neighbor, outside_map, seen) {
                return true;
            }
        }

        // if we get here, everything we've seen does not touch the outside
        for cube in seen.iter() {
            outside_map.insert(*cube, false);
        }

        false
    }
}

impl Problem for BoilingBoulders {
    const DAY: usize = 18;
    const TITLE: &'static str = "boiling boulders";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        let sum = self
            .cubes
            .iter()
            .map(|cube| cube.neighbors().filter(|n| !self.cubes.contains(n)).count())
            .sum::<usize>();

        Ok(sum)
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        // there are only ~9k cubes possible with the input, so we can probably
        // just fill them all in.
        let mut outside_map: FxHashMap<Cube, bool> = FxHashMap::default();
        let sum = self
            .cubes
            .iter()
            .map(|cube| {
                cube.neighbors()
                    .filter(|n| {
                        let mut seen = FxHashSet::default();
                        !self.cubes.contains(n)
                            && self.connects_to_outside(n, &mut outside_map, &mut seen)
                    })
                    .count()
            })
            .sum::<usize>();
        Ok(sum)
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
        let solution = BoilingBoulders::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(4536, 2606));
    }

    #[test]
    fn example() {
        let input = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";
        let solution = BoilingBoulders::solve(input).unwrap();
        assert_eq!(solution, Solution::new(64, 58));
    }
}
