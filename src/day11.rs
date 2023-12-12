use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::BTreeSet;

#[derive(Debug, Clone)]
struct Cosmos {
    stars: Vec<(usize, usize)>,
    empty_rows: BTreeSet<usize>,
    empty_cols: BTreeSet<usize>,
}

#[aoc_generator(day11)]
fn parse_input(input: &str) -> Cosmos {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let empty_rows = (0..grid.len())
        .filter(|&i| !grid[i].contains(&'#'))
        .collect();
    let empty_cols = (0..grid[0].len())
        .filter(|&j| !(0..grid.len()).map(|i| grid[i][j]).contains(&'#'))
        .collect();
    let stars = grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, cell)| *cell == '#')
                .map(move |(j, _)| (i, j))
        })
        .collect();

    Cosmos {
        stars,
        empty_rows,
        empty_cols,
    }
}

fn minmax(a: usize, b: usize) -> (usize, usize) {
    (min(a, b), max(a, b))
}

fn distances(input: &Cosmos, big: bool) -> usize {
    let mult = if big { 999_999 } else { 1 };
    input
        .stars
        .iter()
        .tuple_combinations()
        .map(|(&(i1, j1), &(i2, j2))| {
            let (start_row, end_row) = minmax(i1, i2);
            let (start_col, end_col) = minmax(j1, j2);
            let vert_dist = (end_row - start_row)
                + (start_row..end_row)
                    .collect::<BTreeSet<_>>()
                    .intersection(&input.empty_rows)
                    .count()
                    * mult;
            let horiz_dist = (end_col - start_col)
                + (start_col..end_col)
                    .collect::<BTreeSet<_>>()
                    .intersection(&input.empty_cols)
                    .count()
                    * mult;
            vert_dist + horiz_dist
        })
        .sum()
}

#[aoc(day11, part1)]
fn part1(input: &Cosmos) -> usize {
    distances(input, false)
}

#[aoc(day11, part2)]
fn part2(input: &Cosmos) -> usize {
    distances(input, true)
}
