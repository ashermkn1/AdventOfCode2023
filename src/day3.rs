use itertools::Itertools;
use regex::Regex;
use std::collections::HashSet;

type Position = (usize, usize);

type Input = (Vec<(String, Position)>, HashSet<Position>);

#[aoc_generator(day3, part1)]
fn parse_input1(input: &str) -> Input {
    let mut numbers = vec![];
    let num_re = Regex::new(r"\d+").unwrap();

    for (row, line) in input.lines().enumerate() {
        for num in num_re.find_iter(line) {
            numbers.push((num.as_str().to_owned(), (row, num.start())));
        }
    }

    let symbols = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.char_indices()
                .filter(|&(_, c)| !c.is_ascii_digit() && c != '.')
                .map(move |(col, _)| (row, col))
        })
        .collect::<HashSet<_, _>>();

    (numbers, symbols)
}

fn symbol_adjacent((num_str, (row, col)): (&str, Position), symbols: &HashSet<Position>) -> bool {
    let end_col = col + num_str.len() - 1;
    let mut neighbors = vec![
        (row.saturating_sub(1), col.saturating_sub(1)),
        (row, col.saturating_sub(1)),
        (row + 1, col.saturating_sub(1)),
        (row.saturating_sub(1), end_col + 1),
        (row, end_col + 1),
        (row + 1, end_col + 1),
    ];
    for c in col..=end_col {
        neighbors.push((row.saturating_sub(1), c));
        neighbors.push((row + 1, c));
    }
    neighbors.iter().any(|pos| symbols.contains(pos))
}

#[aoc(day3, part1)]
fn part1((nums, symbols): &Input) -> u32 {
    nums.iter()
        .filter(|&(num, (row, col))| symbol_adjacent((num, (*row, *col)), symbols))
        .map(|(n, _)| n.parse::<u32>().unwrap())
        .sum()
}

#[aoc_generator(day3, part2)]
fn parse_input2(input: &str) -> Input {
    let mut numbers = vec![];
    let num_re = Regex::new(r"\d+").unwrap();

    for (row, line) in input.lines().enumerate() {
        for num in num_re.find_iter(line) {
            numbers.push((num.as_str().to_owned(), (row, num.start())));
        }
    }

    let symbols = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.char_indices()
                .filter(|&(_, c)| c == '*')
                .map(move |(col, _)| (row, col))
        })
        .collect::<HashSet<_, _>>();

    (numbers, symbols)
}

#[aoc(day3, part2)]
fn part2((nums, gears): &Input) -> u32 {
    let mut ratios: Vec<u32> = vec![];
    for &(grow, gcol) in gears {
        let neighbors = [
            (grow.saturating_sub(1), gcol.saturating_sub(1)),
            (grow.saturating_sub(1), gcol),
            (grow.saturating_sub(1), gcol + 1),
            (grow, gcol.saturating_sub(1)),
            (grow, gcol + 1),
            (grow + 1, gcol.saturating_sub(1)),
            (grow + 1, gcol),
            (grow + 1, gcol + 1),
        ];

        let adjacent_nums = nums
            .iter()
            .filter(|(num_str, (nrow, ncol))| {
                let end_col = ncol + num_str.len() - 1;
                neighbors
                    .iter()
                    .any(|pos| pos.0 == *nrow && pos.1 >= *ncol && pos.1 <= end_col)
            })
            .map(|(num_str, _)| num_str.parse().unwrap())
            .collect_vec();

        if adjacent_nums.len() == 2 {
            ratios.push(adjacent_nums.iter().product());
        }
    }

    ratios.into_iter().sum()
}
