use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub struct Card {
    id: u32,
    matches: u32,
}

#[aoc_generator(day4)]
pub fn parse_input(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|line| {
            let line = line.strip_prefix("Card").unwrap().trim();
            let (id, nums) = line.split_once(':').unwrap();

            let (winning, rest) = nums.split_once('|').unwrap();

            let winning: HashSet<u32> = HashSet::from_iter(
                winning
                    .split_ascii_whitespace()
                    .map(|num| num.parse::<u32>().unwrap()),
            );
            let rest = HashSet::from_iter(
                rest.split_ascii_whitespace()
                    .map(|num| num.parse::<u32>().unwrap()),
            );

            let matches = winning.intersection(&rest);
            Card {
                id: id.parse().unwrap(),
                matches: matches.count() as u32,
            }
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn part1(input: &[Card]) -> u32 {
    input
        .iter()
        .map(|card| {
            if card.matches == 0 {
                0
            } else {
                2_u32.pow(card.matches - 1)
            }
        })
        .sum()
}

#[aoc(day4, part2)]
pub fn part2(input: &[Card]) -> u32 {
    let card_map: HashMap<u32, u32> =
        HashMap::from_iter(input.iter().map(|card| (card.id, card.matches)));
    let mut card_counts: HashMap<u32, u32> =
        HashMap::from_iter(input.iter().map(|card| (card.id, 1)));

    for (id, num) in card_map.iter().sorted_by_key(|(id, _)| *id) {
        let copies = card_counts[id];
        for next in (*id + 1)..=(*id + *num) {
            card_counts.entry(next).and_modify(|x| *x += copies);
        }
    }

    card_counts.values().sum()
}
