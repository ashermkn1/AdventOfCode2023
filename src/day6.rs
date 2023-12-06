#[derive(Debug, PartialOrd, PartialEq)]
pub struct Race {
    time: u64,
    record: u64,
}

impl Race {
    pub fn num_wins(&self) -> u64 {
        (0..self.time)
            .map(|time| time * (self.time - time))
            .filter(|&dist| dist > self.record)
            .count() as u64
    }
}
static RACES: [Race; 4] = [
    Race {
        time: 48,
        record: 390,
    },
    Race {
        time: 98,
        record: 1103,
    },
    Race {
        time: 90,
        record: 1112,
    },
    Race {
        time: 83,
        record: 1360,
    },
];

#[aoc(day6, part1)]
pub fn part1(_: &str) -> u64 {
    RACES.iter().map(Race::num_wins).product()
}

#[aoc(day6, part2)]
pub fn part2(_: &str) -> u64 {
    Race {
        time: 48989083,
        record: 390110311121360,
    }
    .num_wins()
}
