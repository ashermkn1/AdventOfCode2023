use itertools::Itertools;
use std::ops::{Add, Mul};
#[derive(Debug, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn origin() -> Self {
        Self::new(0, 0)
    }
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
    fn from_direction(dir: char) -> Self {
        match dir {
            '0' | 'R' => Self::new(1, 0),
            '1' | 'D' => Self::new(0, -1),
            '2' | 'L' => Self::new(-1, 0),
            '3' | 'U' => Self::new(0, 1),
            _ => unreachable!("Malformed input"),
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Mul<i64> for Point {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
#[aoc_generator(day18)]
fn parse_input(input: &str) -> Vec<(char, i64, String)> {
    input
        .lines()
        .map(|line| {
            let (dir, size, color) = line
                .split_ascii_whitespace()
                .collect_tuple()
                .expect("malformed input");

            let dir = dir.chars().next().unwrap();
            let size = size.parse::<i64>().unwrap();
            let color = color.strip_prefix("(#").unwrap().strip_suffix(')').unwrap();

            (dir, size, color.to_owned())
        })
        .collect()
}

fn get_capacity(dig_plan: Vec<Point>) -> i64 {
    let corners: Vec<_> = dig_plan
        .iter()
        .scan(Point::origin(), |state, &point| {
            let res = Some(state.add(point));
            *state = state.add(point);
            res
        })
        .collect();

    let perimeter = dig_plan
        .into_iter()
        .fold(0, |acc, point| acc + point.x.abs() + point.y.abs());

    let area = corners.into_iter().tuple_windows().fold(
        0,
        |acc, (Point { x: x0, y: y0 }, Point { x: x1, y: y1 })| acc + (x0 * y1 - x1 * y0),
    );

    let area = (area / 2).abs();
    area + (perimeter / 2) + 1
}
#[aoc(day18, part1)]
fn part1(input: &[(char, i64, String)]) -> i64 {
    let dig_plan: Vec<_> = input
        .iter()
        .map(|&(dir, size, _)| Point::from_direction(dir) * size)
        .collect();
    get_capacity(dig_plan)
}

#[aoc(day18, part2)]
fn part2(input: &[(char, i64, String)]) -> i64 {
    let dig_plan: Vec<_> = input
        .iter()
        .map(|(_, _, color)| {
            let color = color.clone();
            let (size, dir) = color.split_at(color.len() - 1);
            Point::from_direction(dir.chars().next().unwrap())
                * i64::from_str_radix(size, 16).unwrap()
        })
        .collect();
    get_capacity(dig_plan)
}
