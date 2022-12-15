use std::{collections::VecDeque, str::FromStr};

use anyhow::bail;
use aoc_helpers::generic::Bound2D;
use aoc_plumbing::Problem;
use nom::{
    bytes::complete::tag,
    character::complete::newline,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};
use rustc_hash::FxHashMap;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct Point {
    x: i64,
    y: i64,
}

impl Point {
    pub fn manhattan_distance(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct Line {
    slope: i64,
    y_intersect: i64,
}

impl Line {
    pub fn intersection(&self, other: &Self) -> Option<Point> {
        if self != other && self.slope != other.slope {
            if self.slope > 0 {
                let delta = other.y_intersect - self.y_intersect;
                if delta % 2 == 0 {
                    let x = delta / 2;
                    let y = x + self.y_intersect;
                    return Some(Point { x, y });
                }
            } else {
                let delta = self.y_intersect - other.y_intersect;
                if delta % 2 == 0 {
                    let x = delta / 2;
                    let y = x + other.y_intersect;
                    return Some(Point { x, y });
                }
            }
        }

        None
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct Sensor {
    location: Point,
    closest_beacon: Point,
    dist_to_closest: i64,
}

impl Sensor {
    pub fn segment_for(&self, y: i64) -> Option<Segment> {
        let delta = (self.location.y - y).abs();
        if delta > self.dist_to_closest {
            // we can't say anything about this y coordinate
            return None;
        }

        let spillover = self.dist_to_closest - delta;

        Some(Segment {
            start: self.location.x - spillover,
            end: self.location.x + spillover,
        })
    }

    /// Generate lines parallel to our sensor range but one unit outside of range
    pub fn lines(&self) -> Vec<Line> {
        let mut res = Vec::with_capacity(4);
        let p1_x = self.location.x + self.dist_to_closest + 1;
        let p1_y = self.location.y;
        let a = p1_y - p1_x;
        res.push(Line {
            slope: 1,
            y_intersect: a,
        });
        res.push(Line {
            slope: 1,
            y_intersect: a + 2 * (self.dist_to_closest + 1),
        });

        let p1_x = self.location.x - self.dist_to_closest - 1;
        let p1_y = self.location.y;
        let a = p1_y + p1_x;
        res.push(Line {
            slope: -1,
            y_intersect: a,
        });
        res.push(Line {
            slope: -1,
            y_intersect: a + 2 * (self.dist_to_closest + 1),
        });

        res
    }
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    let (input, (x, y)) = separated_pair(
        preceded(tag("x="), nom::character::complete::i64),
        tag(", "),
        preceded(tag("y="), nom::character::complete::i64),
    )(input)?;

    Ok((input, Point { x, y }))
}

fn parse_sensor(input: &str) -> IResult<&str, Sensor> {
    let (input, (location, closest_beacon)) = preceded(
        tag("Sensor at "),
        separated_pair(parse_point, tag(": closest beacon is at "), parse_point),
    )(input)?;
    let dist_to_closest = location.manhattan_distance(&closest_beacon);
    Ok((
        input,
        Sensor {
            location,
            closest_beacon,
            dist_to_closest,
        },
    ))
}

fn parse_sensors(input: &str) -> IResult<&str, Vec<Sensor>> {
    separated_list1(newline, parse_sensor)(input)
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct Segment {
    start: i64,
    end: i64,
}

impl Segment {
    pub fn overlaps(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.start
            || other.start <= self.start && other.end >= self.start
            || self.start >= other.start && self.end <= other.end
            || other.start >= self.start && other.end <= self.end
    }

    pub fn merge(&self, other: &Self) -> Option<Self> {
        if !self.overlaps(other) {
            None
        } else {
            Some(Self {
                start: self.start.min(other.start),
                end: self.end.max(other.end),
            })
        }
    }

    pub fn len(&self) -> i64 {
        (self.end - self.start).abs()
    }
}

/// Generic over N and M so that we can run the example tests.
///
/// N is the target Y row for part 1, and M is the upper bound for part 2
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BeaconExclusionZoneGen<const N: i64, const M: i64> {
    sensors: Vec<Sensor>,
    bounds: Bound2D<i64>,
}

impl<const N: i64, const M: i64> FromStr for BeaconExclusionZoneGen<N, M> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, mut sensors) = parse_sensors(s.trim()).map_err(|e| e.to_owned())?;

        let mut bounds = Bound2D::minmax();

        for s in sensors.iter() {
            let adjusted_min_x = (s.location.x - s.dist_to_closest).min(s.closest_beacon.x);
            let adjusted_max_x = (s.location.x + s.dist_to_closest).max(s.closest_beacon.x);
            let adjusted_min_y = (s.location.y - s.dist_to_closest).min(s.closest_beacon.y);
            let adjusted_max_y = (s.location.y + s.dist_to_closest).max(s.closest_beacon.y);
            if adjusted_max_x > bounds.max_x {
                bounds.max_x = adjusted_max_x;
            }

            if adjusted_min_x < bounds.min_x {
                bounds.min_x = adjusted_min_x;
            }

            if adjusted_max_y > bounds.max_y {
                bounds.max_y = adjusted_max_y;
            }

            if adjusted_min_y < bounds.min_y {
                bounds.min_y = adjusted_min_y;
            }
        }

        sensors.sort_by(|a, b| a.location.x.cmp(&b.location.x));

        Ok(Self { sensors, bounds })
    }
}

impl<const N: i64, const M: i64> Problem for BeaconExclusionZoneGen<N, M> {
    const DAY: usize = 15;
    const TITLE: &'static str = "beacon exclusion zone";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = i64;
    type P2 = i64;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        let mut segments = self
            .sensors
            .iter()
            .filter_map(|s| s.segment_for(N))
            .collect::<VecDeque<_>>();

        'reducer: loop {
            if let Some(cur) = segments.pop_front() {
                for i in 0..segments.len() {
                    if let Some(merged) = cur.merge(&segments[i]) {
                        segments[i] = merged;
                        continue 'reducer;
                    }
                }
                // we didn't find anything and we
                // we need to put ourselves back
                segments.push_back(cur);

                if segments.len() > 2 {
                    continue 'reducer;
                }
            }
            break;
        }

        let sum: i64 = segments.iter().map(|s| s.len()).sum();
        Ok(sum)
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        // there are 4 lines that lie just beyond the borders of every diamond
        // region the sensors can see. If we calculate the intersection of all
        // of these lines, then we know the only possible points that could be
        // candidates for the beacon that satisfies the search criteria.
        //
        // We know the beacon must lie one the lines because if it were possible
        // for the beacon to not be on one of these lines, there would be
        // multiple solutions instead of a unique one.
        let mut lines = Vec::with_capacity(self.sensors.len() * 4);
        for sensor in self.sensors.iter() {
            let mut sensor_lines = sensor.lines();
            lines.append(&mut sensor_lines);
        }

        let mut intersections: FxHashMap<Point, usize> = FxHashMap::default();

        while let Some(line) = lines.pop() {
            for other in lines.iter() {
                if let Some(pt) = line.intersection(other) {
                    let e = intersections.entry(pt).or_default();
                    *e += 1;
                }
            }
        }

        // there should only be one valid point, and we can prune by checking
        // points against all the sensors
        'searcher: for (pt, count) in intersections.iter() {
            if *count >= 4 && pt.x >= 0 && pt.x < M && pt.y >= 0 && pt.y < M {
                for sensor in self.sensors.iter() {
                    if sensor.location.manhattan_distance(pt) <= sensor.dist_to_closest {
                        continue 'searcher;
                    }
                }

                // if we're here, we passed all the sensors
                return Ok(pt.x * 4_000_000 + pt.y);
            }
        }

        bail!("No beacon found");
    }
}

/// We expose this type for the actual solver and such.
pub type BeaconExclusionZone = BeaconExclusionZoneGen<2_000_000, 4_000_000>;

#[cfg(test)]
mod tests {
    use aoc_plumbing::Solution;

    use super::*;

    #[test]
    #[ignore]
    fn full_dataset() {
        let input = std::fs::read_to_string("input.txt").expect("Unable to load input");
        let solution = BeaconExclusionZone::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(4873353, 11600823139120));
    }

    #[test]
    fn example() {
        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
        let solution = BeaconExclusionZoneGen::<10, 20>::solve(input).unwrap();
        assert_eq!(solution, Solution::new(26, 56000011));
    }
}
