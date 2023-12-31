use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

pub const ORIGIN: Point = Point::new(0, 0);
pub const UP: Point = Point::new(0, -1);
pub const DOWN: Point = Point::new(0, 1);
pub const LEFT: Point = Point::new(-1, 0);
pub const RIGHT: Point = Point::new(1, 0);
pub const ORTHOGONAL: [Point; 4] = [UP, DOWN, LEFT, RIGHT];
pub const DIAGONAL: [Point; 8] = [
    Point::new(-1, -1),
    UP,
    Point::new(1, -1),
    LEFT,
    RIGHT,
    Point::new(-1, 1),
    DOWN,
    Point::new(1, 1),
];
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    #[inline]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    #[inline]
    pub fn clockwise(self) -> Self {
        Point::new(-self.y, self.x)
    }
    #[inline]
    pub fn counter_clockwise(self) -> Self {
        Point::new(self.y, -self.x)
    }
    #[inline]
    pub fn manhattan(self, other: Self) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl From<u8> for Point {
    #[inline]
    fn from(value: u8) -> Self {
        match value {
            b'^' | b'U' => UP,
            b'>' | b'R' => RIGHT,
            b'v' | b'D' => DOWN,
            b'<' | b'L' => LEFT,
            _ => unreachable!("malformed input"),
        }
    }
}
impl Mul<i32> for Point {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: i32) -> Self::Output {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl MulAssign<i32> for Point {
    #[inline]
    fn mul_assign(&mut self, rhs: i32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Add for Point {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Point {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Point {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Point {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
