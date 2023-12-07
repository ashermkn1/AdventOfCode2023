#[derive(Copy, Clone)]
struct Hand {
    bid: u32,
    strength: u32,
}

impl Hand {
    fn new(line: &str) -> Self {
        let mut strength = 0;
        let mut counts = [0; 13];
        let (cards, bid) = line.split_once(' ').unwrap();
        for (i, card) in cards.char_indices() {
            let value = match card {
                'A' => 12,
                'K' => 11,
                'Q' => 10,
                'J' => 9,
                'T' => 8,
                c => c.to_digit(10).unwrap() - 2,
            };
            strength |= value << ((4 - i) * 4);
            counts[value as usize] += 1;
        }
        counts.sort_unstable();
        strength |= hand_type(counts[12], counts[11]) << 20;
        Hand {
            strength,
            bid: bid.parse().unwrap(),
        }
    }
    fn new_with_jokers(line: &str) -> Self {
        let mut strength = 0;
        let mut jokers = 0;
        let mut counts = [0; 13];
        let (cards, bid) = line.split_once(' ').unwrap();
        for (i, card) in cards.char_indices() {
            let value = match card {
                'A' => 12,
                'K' => 11,
                'Q' => 10,
                'J' => 0,
                'T' => 9,
                c => c.to_digit(10).unwrap() - 1,
            };
            if value == 0 {
                jokers += 1;
            } else {
                counts[value as usize] += 1;
            }

            strength |= value << ((4 - i) * 4);
        }
        counts.sort_unstable();
        strength |= hand_type(counts[12] + jokers, counts[11]) << 20;
        Hand {
            strength,
            bid: bid.parse().unwrap(),
        }
    }
}

fn hand_type(max_count: u32, second_max_count: u32) -> u32 {
    match max_count {
        // 5 of a kind
        5 => 6,
        // 4 of a kind
        4 => 5,
        // full house
        3 if second_max_count == 2 => 4,
        // three of a kind
        3 => 3,
        // two pair
        2 if second_max_count == 2 => 2,
        // one pair
        2 => 1,
        // high card
        _ => 0,
    }
}

#[aoc_generator(day7, part1)]
fn parse_input1(input: &str) -> Vec<Hand> {
    input.lines().map(Hand::new).collect()
}

#[aoc_generator(day7, part2)]
fn parse_input2(input: &str) -> Vec<Hand> {
    input.lines().map(Hand::new_with_jokers).collect()
}

#[aoc(day7, part1)]
fn part1(input: &[Hand]) -> u32 {
    let mut hands = input.to_vec();
    hands.sort_unstable_by_key(|hand| hand.strength);
    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + (i + 1) * hand.bid as usize)
        .try_into()
        .unwrap()
}

#[aoc(day7, part2)]
fn part2(input: &[Hand]) -> u32 {
    let mut hands = input.to_vec();
    hands.sort_unstable_by_key(|hand| hand.strength);
    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + (i + 1) * hand.bid as usize)
        .try_into()
        .unwrap()
}
