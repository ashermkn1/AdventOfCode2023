use itertools::Itertools;
use num::Rational64;
use std::collections::HashSet;

const START: (usize, usize) = (65, 65);
const STEPS: u32 = 64;
#[aoc_generator(day21)]
fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .replace('S', ".")
        .lines()
        .map(|s| s.chars().collect())
        .collect()
}

#[aoc(day21, part1)]
fn part1(input: &[Vec<char>]) -> usize {
    let rows = input.len();
    let cols = input[0].len();
    let mut visited = HashSet::new();
    visited.insert(START);
    for _ in 0..STEPS {
        visited = visited
            .into_iter()
            .flat_map(|(row, col)| {
                [
                    (row.saturating_sub(1), col),
                    (row, col + 1),
                    (row + 1, col),
                    (row, col.saturating_sub(1)),
                ]
                .into_iter()
                .filter(|&(row, col)| row < rows && col < cols && input[row][col] == '.')
            })
            .collect::<HashSet<_>>();
    }
    visited.len()
}

fn lagrange_poly(inputs: &[usize], outputs: &[usize], point: usize) -> usize {
    fn make_basis(xs: &[usize], x: usize) -> Vec<Rational64> {
        xs.iter()
            .map(|&xi| {
                Rational64::new(
                    xs.iter()
                        .filter_map(|&xj| {
                            if xj == xi {
                                None
                            } else {
                                Some(x as i64 - xj as i64)
                            }
                        })
                        .product(),
                    xs.iter()
                        .filter_map(|&xj| {
                            if xj == xi {
                                None
                            } else {
                                Some(xi as i64 - xj as i64)
                            }
                        })
                        .product(),
                )
            })
            .collect()
    }
    let basis = make_basis(&inputs, point);

    basis
        .iter()
        .zip(outputs)
        .map(|(&ai, yi)| ai * *yi as i64)
        .sum::<Rational64>()
        .to_integer() as usize
}

fn walk(grid: &[Vec<char>], steps: usize) -> usize {
    let rows = grid.len() as i64;
    let cols = grid[0].len() as i64;
    let start = (START.0 as i64, START.1 as i64);
    let mut frontier = HashSet::from([start]);
    let mut visited = HashSet::new();
    let mut reachable = HashSet::new();
    for i in 0..=steps {
        let mut new_frontier = HashSet::new();
        for (row, col) in frontier {
            visited.insert((row, col));
            if i & 1 == steps & 1 {
                reachable.insert((row, col));
            }
            for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let new_row = row + dr;
                let new_col = col + dc;

                if grid[new_row.rem_euclid(rows) as usize][new_col.rem_euclid(cols) as usize] == '#'
                {
                    continue;
                }
                if visited.contains(&(new_row, new_col)) {
                    continue;
                }
                new_frontier.insert((new_row, new_col));
            }
        }
        frontier = new_frontier;
    }
    reachable.len()
}

#[aoc(day21, part2)]
fn part2(input: &[Vec<char>]) -> usize {
    let rows = input.len();
    let inputs = (1..=5).step_by(2).map(|i| i * rows / 2).collect::<Vec<_>>();
    let outputs = inputs.iter().map(|&x| walk(input, x)).collect::<Vec<_>>();

    lagrange_poly(&inputs, &outputs, 26_501_365)
}
