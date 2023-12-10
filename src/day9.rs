#[aoc_generator(day9)]
fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect()
        })
        .collect()
}

fn extrapolate(nums: &[i64]) -> i64 {
    if nums.iter().copied().all(|x| x == 0) {
        0
    } else {
        let diffs = nums.windows(2).map(|x| x[1] - x[0]).collect::<Vec<_>>();
        *nums.last().unwrap() + extrapolate(&diffs)
    }
}

#[aoc(day9, part1)]
fn part1(input: &[Vec<i64>]) -> i64 {
    input.iter().map(|nums| extrapolate(nums)).sum()
}

#[aoc(day9, part2)]
fn part2(input: &[Vec<i64>]) -> i64 {
    input
        .iter()
        .map(|nums| extrapolate(nums.iter().copied().rev().collect::<Vec<_>>().as_slice()))
        .sum()
}
