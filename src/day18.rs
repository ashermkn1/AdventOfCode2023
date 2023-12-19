use crate::day18::Direction::*;
use itertools::Itertools;

#[derive(Debug, Copy, Clone)]
enum Direction {
    Right(u64),
    Up(u64),
    Down(u64),
    Left(u64),
}

impl Direction {
    fn from_char(dir: char, size: u64) -> Self {
        (match dir {
            'R' => Right,
            'U' => Up,
            'D' => Down,
            'L' => Left,
            _ => unreachable!("malformed direction"),
        })(size)
    }
}
struct Trench {
    edges: Vec<((i64, i64), Direction, u32)>,
}

impl Trench {
    fn get_lava_capacity(&self) -> u64 {
        let mut vertices: Vec<_> = self.edges.iter().copied().map(|(pos, _, _)| pos).collect();
        vertices.push((0, 0));
        let area = vertices
            .into_iter()
            .tuple_windows()
            .fold(0, |acc, ((x0, y0), (x1, y1))| acc + (x0 * y1 - x1 * y0));
        let area = (area / 2).abs_diff(0);

        let boundary: u64 = self
            .edges
            .iter()
            .copied()
            .map(|(_, dir, _)| match dir {
                Right(x) | Down(x) | Up(x) | Left(x) => x,
            })
            .sum();
        area + (boundary / 2) + 1
    }
}
#[aoc_generator(day18, part1)]
fn parse_trench(input: &str) -> Trench {
    let (mut row, mut col) = (0i64, 0i64);
    let edges = input
        .lines()
        .map(|line| {
            let (dir, size, color) = line
                .split_ascii_whitespace()
                .collect_tuple()
                .expect("malformed input");
            let trimmed_color = color
                .strip_prefix("(#")
                .map(|s| s.strip_suffix(')').unwrap())
                .unwrap();
            let hex_color = u32::from_str_radix(trimmed_color, 16).unwrap();
            let dir = Direction::from_char(dir.chars().next().unwrap(), size.parse().unwrap());
            let res = ((row, col), dir, hex_color);
            match dir {
                Right(dx) => {
                    col += dx as i64;
                }
                Up(dy) => {
                    row = row.saturating_sub(dy as i64);
                }
                Down(dy) => {
                    row += dy as i64;
                }
                Left(dx) => {
                    col = col.saturating_sub(dx as i64);
                }
            };
            res
        })
        .collect();
    Trench { edges: dbg!(edges) }
}

#[aoc(day18, part1)]
fn part1(input: &Trench) -> u64 {
    input.get_lava_capacity()
}

#[aoc_generator(day18, part2)]
fn parse_color_trench(input: &str) -> Trench {
    let (mut row, mut col) = (0i64, 0i64);
    let edges = input
        .lines()
        .map(|line| {
            let (_, _, color) = line
                .split_ascii_whitespace()
                .collect_tuple()
                .expect("malformed input");
            let color = color
                .strip_prefix("(#")
                .map(|s| s.strip_suffix(')').unwrap())
                .unwrap();

            let (size, dir) = color.split_at(color.len() - 1);
            let size = u64::from_str_radix(size, 16).unwrap();
            let dir = match dir.chars().next().unwrap() {
                '0' => 'R',
                '1' => 'D',
                '2' => 'L',
                '3' => 'U',
                _ => unreachable!(),
            };
            let dir = Direction::from_char(dir, size);
            let res = ((row, col), dir, 0);
            match dir {
                Right(dx) => {
                    col += dx as i64;
                }
                Up(dy) => {
                    row = row.saturating_sub(dy as i64);
                }
                Down(dy) => {
                    row += dy as i64;
                }
                Left(dx) => {
                    col = col.saturating_sub(dx as i64);
                }
            };
            res
        })
        .collect();
    Trench { edges }
}

#[aoc(day18, part2)]
fn part2(input: &Trench) -> u64 {
    input.get_lava_capacity()
}
