use std::collections::HashMap;
type Grid = Vec<Vec<char>>;
#[aoc_generator(day14)]
fn parse_input(input: &str) -> Grid {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn load(grid: &Grid) -> usize {
    (0..grid.len())
        .map(|r| {
            (0..grid[0].len())
                .filter(|&c| grid[r][c] == 'O')
                .map(|_| grid.len() - r)
                .sum::<usize>()
        })
        .sum()
}
#[aoc(day14, part1)]
fn part1(grid: &[Vec<char>]) -> usize {
    let mut grid = grid.to_vec();
    roll_north(&mut grid);
    load(&grid)
}

fn roll_north(grid: &mut Grid) {
    let mut done = false;
    while !done {
        done = true;
        for r in 0..grid.len() - 1 {
            for c in 0..grid[0].len() {
                if grid[r + 1][c] == 'O' && grid[r][c] == '.' {
                    grid[r][c] = 'O';
                    grid[r + 1][c] = '.';
                    done = false;
                }
            }
        }
    }
}

fn rotate_grid(grid: &Grid) -> Grid {
    let mut new = vec![vec!['.'; grid.len()]; grid[0].len()];
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            new[c][grid.len() - r - 1] = grid[r][c];
        }
    }
    new
}
#[aoc(day14, part2)]
fn part2(input: &[Vec<char>]) -> usize {
    let mut grid = input.to_vec();
    let mut seen = HashMap::new();
    for i in 1..1_000_000_000 {
        for _ in 0..4 {
            roll_north(&mut grid);
            grid = rotate_grid(&grid);
        }
        if let Some(prev_index) = seen.insert(grid.clone(), i) {
            if (1_000_000_000 - i) % (i - prev_index) == 0 {
                return load(&grid);
            }
        }
    }
    0
}
