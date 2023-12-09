use gcd::binary_u64;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
#[derive(Debug, Copy, Clone)]
enum Instruction {
    Left,
    Right,
}

impl Instruction {
    fn from_char(c: char) -> Self {
        match c {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => unreachable!(),
        }
    }
}

type NodeMap = HashMap<String, (String, String)>;
#[derive(Clone, Debug)]
struct Input {
    instructions: Vec<Instruction>,
    nodes: NodeMap,
}

#[aoc_generator(day8)]
fn parse_input(input: &str) -> Input {
    let re = Regex::new(r"\w{3}").unwrap();

    let (instructions, rest) = input.split_once("\n\n").unwrap();

    let instructions = instructions.chars().map(Instruction::from_char).collect();

    let nodes = rest
        .lines()
        .map(|line| {
            let (source, left, right) = re
                .find_iter(line)
                .map(|m| m.as_str().to_owned())
                .collect_tuple()
                .unwrap();
            (source, (left, right))
        })
        .collect::<NodeMap>();

    Input {
        instructions,
        nodes,
    }
}

fn follow(start: &str, nodes: &NodeMap, instructions: &[Instruction]) -> u64 {
    let mut curr = start;
    let mut steps = 0;
    for instr in instructions.iter().cycle() {
        steps += 1;
        curr = match instr {
            Instruction::Left => nodes[curr].0.as_str(),
            Instruction::Right => nodes[curr].1.as_str(),
        };
        if curr.ends_with('Z') {
            return steps;
        }
    }
    0
}

#[aoc(day8, part1)]
fn part1(input: &Input) -> u64 {
    follow("AAA", &input.nodes, &input.instructions)
}

#[aoc(day8, part2)]
fn part2(input: &Input) -> u64 {
    input
        .nodes
        .keys()
        .filter(|s| s.ends_with('A'))
        .map(|s| s.as_str())
        .map(|s| follow(s, &input.nodes, &input.instructions))
        .reduce(|acc, x| acc * x / binary_u64(acc, x))
        .unwrap()
}
