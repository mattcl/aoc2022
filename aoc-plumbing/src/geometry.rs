/// A 2D Point of (i64, i64)
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    /// Make a new point.
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    /// The manhattan_distance between this point and another point.
    ///
    /// # Examples
    /// ```
    /// use aoc_plumbing::geometry::Point;
    /// let p1 = Point::new(-5, 2);
    /// let p2 = Point::new(6, 3);
    /// let res1 = p1.manhattan_distance(&p2);
    /// let res2 = p2.manhattan_distance(&p1);
    ///
    /// assert_eq!(res1, 12);
    /// assert_eq!(res1, res2);
    /// ```
    pub fn manhattan_distance(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl From<(i64, i64)> for Point {
    fn from(value: (i64, i64)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}
