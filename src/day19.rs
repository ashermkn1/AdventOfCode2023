use lazy_static::lazy_static;
use regex::{Match, Regex};
use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

lazy_static! {
    static ref PART_REGEX: Regex = Regex::new(r"^\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}$").unwrap();
}
#[derive(Debug)]
struct ParseError;
#[derive(Debug, PartialEq)]
struct WorkflowStep {
    property: char,
    comp: Ordering,
    rhs: u32,
    dest: String,
}

impl FromStr for WorkflowStep {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (test, dest) = s.split_once(':').ok_or(ParseError)?;
        let (beginning, rhs) = test.split_at(2);
        let rhs = rhs.parse::<u32>().map_err(|_| ParseError)?;
        let (prop, comp) = {
            let mut c = beginning.chars();
            let prop = c.next().ok_or(ParseError)?;
            let comp = match c.next().ok_or(ParseError)? {
                '<' => Ordering::Less,
                '>' => Ordering::Greater,
                _ => return Err(ParseError),
            };
            (prop, comp)
        };
        Ok(Self {
            property: prop,
            comp,
            rhs,
            dest: dest.to_string(),
        })
    }
}

#[derive(Debug, PartialEq)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn get(&self, property: char) -> u32 {
        match property {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => unreachable!("Malformed property"),
        }
    }

    fn matches(&self, step: &WorkflowStep) -> bool {
        self.get(step.property).cmp(&step.rhs) == step.comp
    }

    fn total(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}

impl FromStr for Part {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = PART_REGEX.captures(s).ok_or(ParseError)?;
        let process_cap = |cap: Match| cap.as_str().parse::<u32>().map_err(|_| ParseError);

        Ok(Self {
            x: process_cap(captures.get(1).ok_or(ParseError)?)?,
            m: process_cap(captures.get(2).ok_or(ParseError)?)?,
            a: process_cap(captures.get(3).ok_or(ParseError)?)?,
            s: process_cap(captures.get(4).ok_or(ParseError)?)?,
        })
    }
}
#[derive(Debug, PartialEq)]
struct Workflow {
    name: String,
    steps: Vec<WorkflowStep>,
    default: String,
}

impl Workflow {
    fn process(&self, part: &Part) -> &str {
        if let Some(matching) = self.steps.iter().find(|&step| part.matches(step)) {
            &matching.dest
        } else {
            &self.default
        }
    }

    fn process_state(&self, state: PartState, queue: &mut VecDeque<(PartState, String)>) {
        let mut curr_state = Some(state);

        for step in &self.steps {
            if curr_state.is_none() {
                return;
            }

            let state = curr_state.unwrap();
            let (stay, go) = state.split(step);

            if let Some(go) = go {
                queue.push_back(go);
            }

            curr_state = stay;
        }

        if let Some(default) = curr_state {
            queue.push_back((default, self.default.to_string()));
        }
    }
}

impl FromStr for Workflow {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_suffix('}').ok_or(ParseError)?;
        let (name, steps) = s.split_once('{').ok_or(ParseError)?;
        let last_comma = steps
            .chars()
            .rev()
            .position(|c| c == ',')
            .ok_or(ParseError)?;
        let last_comma = steps.len() - last_comma - 1;
        let (steps, default) = steps.split_at(last_comma);
        let default = &default[1..];
        Ok(Self {
            name: name.to_string(),
            steps: steps.split(',').filter_map(|s| s.parse().ok()).collect(),
            default: default.to_string(),
        })
    }
}
#[derive(Debug)]
struct Input {
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>,
}

impl Input {
    fn process(&self, part: &Part) -> String {
        let mut current = "in";
        while let Some(next) = self.workflows.get(current) {
            current = next.process(part);
        }
        current.to_string()
    }
}

#[aoc_generator(day19)]
fn parse_input(input: &str) -> Input {
    let (workflows, parts) = input.split_once("\n\n").expect("Malformed input");
    let workflows = workflows
        .lines()
        .map(|line| {
            let workflow = line.parse::<Workflow>().unwrap();
            (workflow.name.clone(), workflow)
        })
        .collect::<HashMap<_, _>>();
    let parts = parts.lines().filter_map(|line| line.parse().ok()).collect();

    Input { workflows, parts }
}

#[aoc(day19, part1)]
fn part1(input: &Input) -> u32 {
    input
        .parts
        .iter()
        .filter_map(|part| {
            if input.process(part) == "A" {
                Some(part.total())
            } else {
                None
            }
        })
        .sum()
}
#[derive(Debug, Copy, Clone, PartialEq)]
struct Range(u32, u32);

impl Range {
    fn range(self) -> u32 {
        self.1 - self.0 + 1
    }
    // returns (stay, go)
    fn split(self, comp: Ordering, rhs: u32) -> (Option<Self>, Option<Self>) {
        match comp {
            Ordering::Less => {
                if rhs <= self.0 {
                    (Some(self), None)
                } else if rhs > self.1 {
                    (None, Some(self))
                } else {
                    (
                        Some(Self(rhs, self.1)),
                        Some(Self(self.0, rhs.saturating_sub(1))),
                    )
                }
            }
            Ordering::Equal => (None, None),
            Ordering::Greater => {
                if rhs >= self.1 {
                    (Some(self), None)
                } else if rhs < self.0 {
                    (None, Some(self))
                } else {
                    (Some(Self(self.0, rhs)), Some(Self(rhs + 1, self.1)))
                }
            }
        }
    }
}
#[derive(Debug, PartialEq)]
struct PartState {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

impl PartState {
    fn initial() -> Self {
        Self {
            x: Range(1, 4000),
            m: Range(1, 4000),
            a: Range(1, 4000),
            s: Range(1, 4000),
        }
    }

    fn get(&self, property: char) -> Range {
        match property {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => unreachable!("Malformed property"),
        }
    }

    fn replace_range(&self, property: char, new: Range) -> Self {
        match property {
            'x' => Self { x: new, ..*self },
            'm' => Self { m: new, ..*self },
            'a' => Self { a: new, ..*self },
            's' => Self { s: new, ..*self },
            _ => unreachable!("Malformed property"),
        }
    }
    fn split(&self, step: &WorkflowStep) -> (Option<PartState>, Option<(PartState, String)>) {
        let (stay, go) = self.get(step.property).split(step.comp, step.rhs);
        let stay = stay.map(|range| self.replace_range(step.property, range));

        let next_state = go.map(|range| {
            (
                self.replace_range(step.property, range),
                step.dest.to_string(),
            )
        });
        (stay, next_state)
    }

    fn distinct_combinations(&self) -> u64 {
        u64::from(self.x.range())
            * u64::from(self.m.range())
            * u64::from(self.a.range())
            * u64::from(self.s.range())
    }
}

#[aoc(day19, part2)]
fn part2(input: &Input) -> u64 {
    let mut res = 0;
    let mut q = VecDeque::new();
    q.push_back((PartState::initial(), "in".to_string()));
    while let Some((state, workflow)) = q.pop_front() {
        if workflow == "A" {
            res += state.distinct_combinations();
            continue;
        }

        if let Some(next) = input.workflows.get(&workflow) {
            next.process_state(state, &mut q);
        }
    }
    res
}
