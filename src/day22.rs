use itertools::Itertools;
use std::collections::VecDeque;

struct Input {
    above: Vec<Vec<usize>>,
    below: Vec<Vec<usize>>,
}

fn parse_bricks(input: &str) -> Vec<[usize; 6]> {
    input
        .lines()
        .map(|line| {
            let line = line.replace('~', ",");
            line.split(',')
                .filter_map(|s| s.parse().ok())
                .collect_vec()
                .try_into()
                .unwrap()
        })
        .sorted_unstable_by_key(|brick: &[usize; 6]| brick[2])
        .collect()
}
#[aoc_generator(day22)]
fn parse_input(input: &str) -> Input {
    let bricks = parse_bricks(input);
    let mut heights = [[0; 10]; 10];
    let mut indices = [[usize::MAX; 10]; 10];
    let mut above = vec![vec![]; bricks.len()];
    let mut below = vec![vec![]; bricks.len()];

    for (i, [x1, y1, z1, x2, y2, z2]) in bricks.into_iter().enumerate() {
        let height = z2 - z1 + 1;
        let top = (x1..=x2)
            .cartesian_product(y1..=y2)
            .map(|(x, y)| heights[x][y])
            .max()
            .unwrap_or(0);
        let mut prev = usize::MAX;

        for x in x1..=x2 {
            for y in y1..=y2 {
                if heights[x][y] == top {
                    let index = indices[x][y];
                    if index != prev {
                        above[index].push(i);
                        below[i].push(index);
                        prev = index;
                    }
                }

                heights[x][y] = top + height;
                indices[x][y] = i;
            }
        }
    }
    Input { above, below }
}

fn safe_to_remove(supports: &[Vec<usize>]) -> Vec<bool> {
    let mut removable = vec![true; supports.len()];

    for underneath in supports {
        if underneath.len() == 1 {
            removable[underneath[0]] = false;
        }
    }
    removable
}

#[aoc(day22, part1)]
fn part1(input: &Input) -> usize {
    let Input { below, .. } = input;

    safe_to_remove(below).into_iter().filter(|&b| b).count()
}

#[aoc(day22, part2)]
fn part2(input: &Input) -> usize {
    let safe = safe_to_remove(&input.below);
    let mut res = 0;
    // which bricks we have to still deal with
    let mut falling = VecDeque::new();
    // which brick is responsible for making each one fall
    let mut removed = vec![usize::MAX; input.below.len()];
    // only makes sense to remove blocks that would make others fall
    for start in (0..safe.len()).filter(|&i| !safe[i]) {
        falling.push_back(start);
        removed[start] = start;

        while let Some(brick) = falling.pop_front() {
            for &upper in &input.above[brick] {
                // make sure that this block hasn't already been removed in this reaction
                // and that it would actually fall
                if removed[upper] != start
                    && input.below[upper].iter().all(|&i| removed[i] == start)
                {
                    res += 1;
                    removed[upper] = start;
                    falling.push_back(upper);
                }
            }
        }
    }
    res
}
