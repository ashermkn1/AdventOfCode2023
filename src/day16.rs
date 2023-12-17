use rayon::prelude::*;
#[derive(Clone, Debug)]
struct Traversal {
    layout: Vec<u8>,
    rows: usize,
    cols: usize,
}

const UP: u8 = 1;
const RIGHT: u8 = 2;
const DOWN: u8 = 4;
const LEFT: u8 = 8;

const SPLITTER_H: u8 = 0;
const SPLITTER_V: u8 = 1;
const MIRROR_F: u8 = 2;
const MIRROR_B: u8 = 3;
const OPEN: u8 = 4;

impl Traversal {
    fn idx(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    fn step(&self, row: usize, col: usize, dir: u8) -> Option<(usize, usize)> {
        match dir {
            UP if row == 0 => None,
            UP => Some((row - 1, col)),
            RIGHT if col == self.cols - 1 => None,
            RIGHT => Some((row, col + 1)),
            DOWN if row == self.rows - 1 => None,
            DOWN => Some((row + 1, col)),
            LEFT if col == 0 => None,
            LEFT => Some((row, col - 1)),
            _ => unreachable!("Invalid direction"),
        }
    }

    fn traverse(&mut self, mut row: usize, mut col: usize, mut dir: u8) -> usize {
        let mut res = 0;
        loop {
            let idx = self.idx(row, col);
            // LSB stores tile information
            let tile = self.layout[idx] & 0xF;
            // MSB stores visited information for memoization
            let visited = self.layout[idx] >> 4;
            // We have been here before from this direction
            if dir & visited != 0 {
                break;
            }
            // We haven't touched this before
            if visited == 0 {
                res += 1;
            }
            // Add direction to memoization
            self.layout[idx] |= dir << 4;
            match tile {
                SPLITTER_H => {
                    if dir == UP || dir == DOWN {
                        if let Some((r, c)) = self.step(row, col, LEFT) {
                            res += self.traverse(r, c, LEFT);
                        }
                        dir = RIGHT;
                    }
                }
                SPLITTER_V => {
                    if dir == LEFT || dir == RIGHT {
                        if let Some((r, c)) = self.step(row, col, UP) {
                            res += self.traverse(r, c, UP);
                        }
                        dir = DOWN;
                    }
                }
                MIRROR_F => {
                    dir = match dir {
                        UP => RIGHT,
                        RIGHT => UP,
                        DOWN => LEFT,
                        LEFT => DOWN,
                        _ => unreachable!("Invalid direction"),
                    }
                }
                MIRROR_B => {
                    dir = match dir {
                        UP => LEFT,
                        LEFT => UP,
                        RIGHT => DOWN,
                        DOWN => RIGHT,
                        _ => unreachable!("Invalid direction"),
                    }
                }
                _ => {}
            }
            if let Some((r, c)) = self.step(row, col, dir) {
                row = r;
                col = c;
            } else {
                break;
            }
        }
        res
    }
}
#[aoc_generator(day16)]
fn parse_input(input: &str) -> Traversal {
    let cols = input.find('\n').unwrap();
    let rows = input.len() / cols;
    let layout = input
        .chars()
        .filter(|&c| c != '\n')
        .map(|c| match c {
            '.' => OPEN,
            '\\' => MIRROR_B,
            '/' => MIRROR_F,
            '-' => SPLITTER_H,
            '|' => SPLITTER_V,
            _ => unreachable!("Invalid character"),
        })
        .collect();

    Traversal { layout, rows, cols }
}

#[aoc(day16, part1)]
fn part1(input: &Traversal) -> usize {
    input.clone().traverse(0, 0, RIGHT)
}

#[aoc(day16, part2)]
fn part2(input: &Traversal) -> usize {
    let mut options = Vec::with_capacity(500);
    for r in 0..input.rows {
        options.push((r, 0, RIGHT));
        options.push((r, input.cols - 1, LEFT));
    }
    for c in 0..input.cols {
        options.push((0, c, DOWN));
        options.push((input.rows - 1, c, UP));
    }
    options
        .par_iter()
        .map(|&(r, c, d)| input.clone().traverse(r, c, d))
        .max()
        .unwrap()
}
