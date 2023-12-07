use itertools::Itertools;
use std::collections::BTreeSet;

#[derive(Debug, PartialEq)]
struct MapEntry {
    dest_start: u64,
    source_start: u64,
    len: u64,
}

impl MapEntry {
    fn convert(&self, value: u64) -> Option<u64> {
        if self.source_start <= value && value < self.source_start + self.len {
            Some(value - self.source_start + self.dest_start)
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq)]
struct Map(Vec<MapEntry>);

type Range = (u64, u64);
impl Map {
    fn convert(&self, value: u64) -> u64 {
        if let Some(dest) = self.0.iter().find_map(|entry| entry.convert(value)) {
            dest
        } else {
            value
        }
    }

    fn convert_range(&self, (range_start, range_len): Range) -> Vec<Range> {
        let range_end = range_start + range_len;
        let mut slices = self.0.iter().fold(BTreeSet::new(), |mut set, entry| {
            let source_start = entry.source_start;
            let len = entry.len;
            let source_end = source_start + len;

            if range_end < source_start || range_start >= source_end {
                set
            } else {
                if source_start > range_start {
                    set.insert(source_start);
                }

                if source_end < range_end {
                    set.insert(source_end);
                }
                set
            }
        });
        slices.insert(range_end);
        slices
            .iter()
            .fold(
                (Vec::new(), range_start),
                |(mut ranges, current): (Vec<Range>, u64), pos| {
                    ranges.push((self.convert(current), pos - current));
                    (ranges, *pos)
                },
            )
            .0
    }
}
#[derive(Debug, PartialEq)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

impl Almanac {
    fn seed_to_location(&self, seed: u64) -> u64 {
        self.maps.iter().fold(seed, |val, map| map.convert(val))
    }

    fn seed_ranges(&self) -> Vec<Range> {
        self.seeds.iter().copied().tuples::<(_, _)>().collect()
    }
}

fn parse_seeds(input: &str) -> Vec<u64> {
    input
        .strip_prefix("seeds: ")
        .unwrap()
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn parse_map(input: &str) -> Map {
    let entries = input
        .lines()
        .skip(1)
        .map(|line| {
            let (dest_start, source_start, len) = line
                .split_ascii_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect_tuple()
                .unwrap();

            MapEntry {
                dest_start,
                source_start,
                len,
            }
        })
        .collect();

    Map(entries)
}

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Almanac {
    let mut blocks = input.split("\n\n");
    let seeds = parse_seeds(blocks.next().unwrap());

    let maps = blocks.map(parse_map).collect();

    Almanac { seeds, maps }
}

#[aoc(day5, part1)]
fn part1(input: &Almanac) -> u64 {
    input
        .seeds
        .iter()
        .map(|&seed| input.seed_to_location(seed))
        .min()
        .unwrap()
}

#[aoc(day5, part2)]
fn part2(input: &Almanac) -> u64 {
    let mut ranges = input.seed_ranges();
    let mut next = Vec::new();

    for map in &input.maps {
        for range in ranges {
            next.extend(map.convert_range(range));
        }
        ranges = next;
        next = Vec::new();
    }
    ranges.iter().map(|range| range.0).min().unwrap()
}
