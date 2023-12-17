use std::collections::{BinaryHeap, HashMap};

fn dijkstras(grid: &[Vec<u32>], minstep: isize, maxstep: isize) -> i64 {
    let goal = (grid.len() - 1, grid[0].len() - 1);
    let mut dists = HashMap::new();
    let mut pq = BinaryHeap::new();
    pq.push((0, (0, 0, (0, 0))));
    while let Some((cost, (row, col, (dr, dc)))) = pq.pop() {
        if (row, col) == goal {
            return -cost;
        }
        if dists
            .get(&(row, col, (dr, dc)))
            .is_some_and(|&old_cost| -cost > old_cost)
        {
            continue;
        }
        for next_step in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
            if next_step == (dr, dc) || next_step == (-dr, -dc) {
                continue;
            }
            let mut new_cost = -cost;
            for jump in 1..=maxstep {
                let newr = (row as isize + next_step.0 * jump) as usize;
                let newc = (col as isize + next_step.1 * jump) as usize;
                if newr >= grid.len() || newc >= grid[0].len() {
                    continue;
                }
                new_cost += i64::from(grid[newr][newc]);
                if jump < minstep {
                    continue;
                }
                if new_cost < *dists.get(&(newr, newc, next_step)).unwrap_or(&i64::MAX) {
                    dists.insert((newr, newc, next_step), new_cost);
                    pq.push((-new_cost, (newr, newc, next_step)));
                }
            }
        }
    }
    unreachable!("No path found")
}

#[aoc_generator(day17)]
fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect()
}

#[aoc(day17, part1)]
fn part1(input: &[Vec<u32>]) -> i64 {
    let input = input.to_vec();
    dijkstras(&input, 1, 3)
}

#[aoc(day17, part2)]
fn part2(input: &[Vec<u32>]) -> i64 {
    let input = input.to_vec();
    dijkstras(&input, 4, 10)
}
