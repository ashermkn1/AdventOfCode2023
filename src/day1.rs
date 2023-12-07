#[aoc(day1, part1)]
fn part1(input: &str) -> u32 {
    let input: Vec<&str> = input.lines().collect();
    let mut sum = 0;
    for line in input {
        let first_char = line.chars().find(|&c| c.is_ascii_digit()).unwrap();
        let last_char = line.chars().rev().find(|&c| c.is_ascii_digit()).unwrap();
        let mut s = String::from(first_char);
        s.push(last_char);
        sum += s.parse::<u32>().unwrap();
    }
    sum
}

#[aoc(day1, part2)]
fn part2(input: &str) -> u32 {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.to_string()
                .replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine")
        })
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<_>>()
        })
        .map(|v| 10 * v.first().unwrap() + v.last().unwrap())
        .sum()
}
