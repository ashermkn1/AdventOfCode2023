use std::str::FromStr;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl CubeSet {
    fn power(&self) -> u32 {
        self.red * self.blue * self.green
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    cubes: Vec<CubeSet>,
}

impl Game {
    fn valid_set(&self, set: CubeSet) -> bool {
        self.cubes.iter().max_by_key(|c| c.red).unwrap().red <= set.red
            && self.cubes.iter().max_by_key(|c| c.blue).unwrap().blue <= set.blue
            && self.cubes.iter().max_by_key(|c| c.green).unwrap().green <= set.green
    }
    fn minimum_set(&self) -> CubeSet {
        CubeSet {
            red: self.cubes.iter().max_by_key(|c| c.red).unwrap().red,
            green: self.cubes.iter().max_by_key(|c| c.green).unwrap().green,
            blue: self.cubes.iter().max_by_key(|c| c.blue).unwrap().blue,
        }
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, rest) = s.strip_prefix("Game ").unwrap().split_once(": ").unwrap();
        let id = id.parse::<u32>().unwrap();

        let cubes = rest
            .split("; ")
            .map(|set| {
                let mut green = 0;
                let mut red = 0;
                let mut blue = 0;
                for p in set.split(", ") {
                    let (num, color) = p.split_once(' ').unwrap();
                    let num = num.parse::<u32>().unwrap();
                    match color {
                        "red" => red = num,
                        "green" => green = num,
                        "blue" => blue = num,
                        _ => unreachable!(),
                    }
                }
                CubeSet { red, green, blue }
            })
            .collect();
        Ok(Game { id, cubes })
    }
}

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Vec<Game> {
    input
        .lines()
        .filter_map(|s| Game::from_str(s).ok())
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[Game]) -> u32 {
    let set = CubeSet {
        red: 12,
        green: 13,
        blue: 14,
    };
    input
        .iter()
        .filter(|game| game.valid_set(set))
        .map(|game| game.id)
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &[Game]) -> u32 {
    input.iter().map(|game| game.minimum_set().power()).sum()
}
