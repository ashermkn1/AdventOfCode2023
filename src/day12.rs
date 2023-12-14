use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
struct Record {
    spring: String,
    counts: Vec<usize>,
}

#[aoc_generator(day12)]
fn parse_input(input: &str) -> Vec<Record> {
    input
        .lines()
        .map(|line| {
            let (spring, counts) = line.split_once(' ').unwrap();
            let counts = counts
                .split(',')
                .filter_map(|s| s.parse::<usize>().ok())
                .collect();
            Record {
                spring: spring.to_string(),
                counts,
            }
        })
        .collect()
}
type Key = (usize, usize, usize);
type Cache = HashMap<Key, u64>;
fn solve(cache: &mut Cache, springs: &Record, (springi, blocki, curr_len): Key) -> u64 {
    let blocks = &springs.counts;
    let spring = springs.spring.as_str();
    if let Some(&res) = cache.get(&(springi, blocki, curr_len)) {
        return res;
    }
    // end case
    if springi == spring.len() {
        if blocki == blocks.len() && curr_len == 0
            || blocki == blocks.len() - 1 && blocks[blocki] == curr_len
        {
            return 1;
        }
        return 0;
    }

    let chars = match spring.chars().nth(springi).unwrap() {
        '?' => vec!['#', '.'],
        c => vec![c],
    };
    let mut ans = 0;
    for c in chars {
        ans += if c == '.' {
            if curr_len == 0 {
                solve(cache, springs, (springi + 1, blocki, curr_len))
            } else if blocki < blocks.len() && blocks[blocki] == curr_len {
                solve(cache, springs, (springi + 1, blocki + 1, 0))
            } else {
                0
            }
        } else {
            solve(cache, springs, (springi + 1, blocki, curr_len + 1))
        }
    }
    cache.insert((springi, blocki, curr_len), ans);
    ans
}
#[aoc(day12, part1)]
fn part1(input: &[Record]) -> u64 {
    input.iter().fold(0, |acc, record| {
        acc + solve(&mut HashMap::new(), record, (0, 0, 0))
    })
}

#[aoc(day12, part2)]
fn part2(input: &[Record]) -> u64 {
    input
        .iter()
        .map(|record| {
            let unfolded = vec![record.spring.clone(); 5].join("?");
            Record {
                spring: unfolded,
                counts: record.counts.repeat(5),
            }
        })
        .fold(0, |acc, record| {
            acc + solve(&mut HashMap::new(), &record, (0, 0, 0))
        })
}
