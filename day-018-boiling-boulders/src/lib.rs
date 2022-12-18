use std::str::FromStr;

use aoc_plumbing::Problem;
use nom::{character::complete::multispace1, multi::separated_list1, sequence::tuple, IResult};
use rustc_hash::FxHashSet;

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

        bounds.min_x -= 1;
        bounds.min_y -= 1;
        bounds.min_z -= 1;
        bounds.max_x += 1;
        bounds.max_y += 1;
        bounds.max_z += 1;

        Ok(Self { cubes, bounds })
    }
}

impl BoilingBoulders {
    pub fn outer_surface(&self) -> usize {
        // pick a place on the bounds and bfs to the other corner
        let start = Cube {
            x: self.bounds.min_x,
            y: self.bounds.min_y,
            z: self.bounds.min_z,
        };

        let mut fringe = Vec::default();
        let mut seen = FxHashSet::default();
        seen.insert(start);
        fringe.push(start);

        self.surface_recur(fringe, &mut seen)
    }

    pub fn surface_recur(&self, fringe: Vec<Cube>, seen: &mut FxHashSet<Cube>) -> usize {
        let mut sum = 0;
        let mut next_fringe = Vec::with_capacity(fringe.len());

        for cube in fringe.iter() {
            for neighbor in cube.neighbors() {
                if self.bounds.does_not_contain(&neighbor) {
                    continue;
                }

                if seen.contains(&neighbor) {
                    continue;
                }

                // luckily we're counting surface area, or we'd have to record
                // this collision
                if self.cubes.contains(&neighbor) {
                    sum += 1;
                    continue;
                }

                seen.insert(neighbor);
                next_fringe.push(neighbor);
            }
        }

        if next_fringe.is_empty() {
            return sum;
        }

        sum + self.surface_recur(next_fringe, seen)
    }

    // this was a test, and it doesn't improve performance with the given input
    pub fn outer_surface_iterative(&self) -> usize {
        // pick a place on the bounds and bfs to the other corner
        let start = Cube {
            x: self.bounds.min_x,
            y: self.bounds.min_y,
            z: self.bounds.min_z,
        };

        let mut fringe = Vec::default();
        let mut seen = FxHashSet::default();
        seen.insert(start);
        fringe.push(start);

        let mut sum = 0;

        loop {
            let mut next_fringe = Vec::with_capacity(fringe.len());
            for cube in fringe.iter() {
                for neighbor in cube.neighbors() {
                    if self.bounds.does_not_contain(&neighbor) {
                        continue;
                    }
                    if seen.contains(&neighbor) {
                        continue;
                    }
                    if self.cubes.contains(&neighbor) {
                        sum += 1;
                        continue;
                    }
                    seen.insert(neighbor);

                    next_fringe.push(neighbor);
                }
            }

            if next_fringe.is_empty() {
                break;
            }

            fringe = next_fringe;
        }

        sum
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
        Ok(self.outer_surface())
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
