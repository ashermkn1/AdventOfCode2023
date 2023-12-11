use crate::day10::Direction::{E, N, S, W};
use itertools::Itertools;

#[derive(Debug, Copy, Clone, PartialEq, Default)]
struct Cell {
    north: bool,
    east: bool,
    south: bool,
    west: bool,
}
static START_POS: (usize, usize) = (128, 36);

impl Cell {
    fn from_char(c: char) -> Self {
        match c {
            '|' => Self {
                north: true,
                south: true,
                ..Default::default()
            },
            '-' => Self {
                east: true,
                west: true,
                ..Default::default()
            },
            'L' => Self {
                north: true,
                east: true,
                ..Default::default()
            },
            'J' => Self {
                north: true,
                west: true,
                ..Default::default()
            },
            '7' => Self {
                south: true,
                west: true,
                ..Default::default()
            },
            'F' | 'S' => Self {
                south: true,
                east: true,
                ..Default::default()
            },
            '.' => Cell::default(),
            _ => unreachable!(),
        }
    }
}

fn get_path(grid: &[Vec<Cell>]) -> Vec<(usize, usize)> {
    let mut path = vec![START_POS];
    let mut came_from = W;
    let mut row = START_POS.0;
    let mut col = START_POS.1 + 1;
    while (row, col) != START_POS {
        path.push((row, col));
        if came_from != E && grid[row][col].east {
            col += 1;
            came_from = W;
        } else if came_from != S && grid[row][col].south {
            row += 1;
            came_from = N;
        } else if came_from != W && grid[row][col].west {
            col -= 1;
            came_from = E;
        } else if came_from != N && grid[row][col].north {
            row -= 1;
            came_from = S;
        }
    }
    path
}

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Vec<Vec<Cell>> {
    input
        .lines()
        .map(|line| line.chars().map(Cell::from_char).collect())
        .collect()
}

#[aoc(day10, part1)]
fn part1(input: &[Vec<Cell>]) -> usize {
    get_path(input).len() / 2
}
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    N,
    S,
    E,
    W,
}

#[aoc(day10, part2)]
fn part2(input: &[Vec<Cell>]) -> i64 {
    let mut path = get_path(input);
    let path_len = path.len() as i64;
    // add start again for shoelace formula
    path.push(START_POS);
    let area = path
        .into_iter()
        .map(|(x, y)| (x as i64, y as i64))
        .tuple_windows()
        .fold(0, |acc, ((x0, y0), (x1, y1))| acc + ((y0 + y1) * (x0 - x1)))
        .abs();

    area / 2 - (path_len / 2 - 1)
}
