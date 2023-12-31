use crate::util::point::*;
use std::ops::{Index, IndexMut};

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Grid<T> {
    pub width: i32,
    pub height: i32,
    pub bytes: Vec<T>,
}

impl Grid<u8> {
    pub fn parse(input: &str) -> Self {
        let lines: Vec<_> = input.lines().map(str::as_bytes).collect();
        Grid {
            width: lines[0].len() as i32,
            height: lines.len() as i32,
            bytes: lines.into_iter().flatten().copied().collect(),
        }
    }
}

impl<T: Copy + PartialEq> Grid<T> {
    pub fn find(&self, needle: T) -> Option<Point> {
        self.bytes.iter().position(|&h| h == needle).map(|index| {
            let x = (index as i32) % self.width;
            let y = (index as i32) / self.width;
            Point::new(x, y)
        })
    }
    #[inline]
    pub fn contains(&self, point: Point) -> bool {
        point.x >= 0 && point.x < self.width && point.y >= 0 && point.y < self.height
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, point: Point) -> &Self::Output {
        &self.bytes[(self.width * point.y + point.x) as usize]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        &mut self.bytes[(self.width * point.y + point.x) as usize]
    }
}
