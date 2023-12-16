use crate::day15::Operation::{Add, Remove};
use itertools::Itertools;

#[aoc_generator(day15, part1)]
fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .split(',')
        .map(str::to_owned)
        .map(String::into_bytes)
        .collect()
}

fn hash(s: &[u8]) -> usize {
    s.iter().fold(0, |acc, c| {
        let acc = acc + (*c as usize);
        (acc * 17) % 256
    })
}

#[aoc(day15, part1)]
fn part1(input: &[Vec<u8>]) -> usize {
    input.iter().map(|s| hash(s)).sum()
}
#[derive(Debug, Copy, Clone)]
enum Operation {
    Remove,
    Add(u32),
}
#[derive(Debug, Clone)]
struct Instruction {
    label: String,
    hash: usize,
    op: Operation,
}

#[aoc_generator(day15, part2)]
fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .split(',')
        .map(|s| {
            if let Some((label, length)) = s.split_once('=') {
                Instruction {
                    label: label.to_string(),
                    hash: hash(label.as_bytes()),
                    op: Add(length.parse().unwrap()),
                }
            } else {
                let label = s[0..s.len() - 1].to_string();
                let hash = hash(label.as_bytes());
                Instruction {
                    label,
                    hash,
                    op: Remove,
                }
            }
        })
        .collect()
}

#[aoc(day15, part2)]
fn part2(input: &[Instruction]) -> usize {
    let mut boxes: Vec<Vec<(&str, u32)>> = vec![vec![]; 256];
    'outer: for instr in input {
        match instr.op {
            Remove => {
                if let Some((i, _)) = boxes[instr.hash]
                    .iter()
                    .find_position(|(label, _)| *label == instr.label)
                {
                    boxes[instr.hash].remove(i);
                }
            }
            Add(length) => {
                for (i, &l) in boxes[instr.hash].iter().enumerate() {
                    if l.0 == instr.label {
                        boxes[instr.hash][i] = (instr.label.as_str(), length);
                        continue 'outer;
                    }
                }
                boxes[instr.hash].push((instr.label.as_str(), length));
            }
        }
    }

    let mut res = 0;
    for (i, list) in boxes.iter().enumerate() {
        for (j, &(_, length)) in list.iter().enumerate() {
            let mut power = 1 + i;
            power *= j + 1;
            power *= length as usize;
            res += power;
        }
    }
    res
}
