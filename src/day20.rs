use dyn_clone::DynClone;
use num::integer::lcm;
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::ops::Not;

#[derive(Debug, Copy, Clone, PartialEq, Hash, Default, Eq)]
enum Pulse {
    #[default]
    Low,
    High,
}

impl Not for Pulse {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Pulse::Low => Pulse::High,
            Pulse::High => Pulse::Low,
        }
    }
}
trait Module: Debug + DynClone {
    fn recv(&mut self, signal: Pulse, sender: String);
    fn get_state(&self) -> Pulse;

    fn initial(&mut self, _: String) {}
    fn will_send(&self, _: Pulse) -> bool {
        true
    }
}

dyn_clone::clone_trait_object!(Module);
#[derive(Debug, Copy, Clone, Default)]
struct FlipFlop {
    state: Pulse,
}

impl Module for FlipFlop {
    fn recv(&mut self, signal: Pulse, _: String) {
        if signal == Pulse::Low {
            self.state = !self.state;
        }
    }

    fn get_state(&self) -> Pulse {
        self.state
    }

    fn will_send(&self, signal: Pulse) -> bool {
        signal == Pulse::Low
    }
}
#[derive(Debug, Copy, Clone, Default)]
struct Broadcaster {
    state: Pulse,
}

impl Module for Broadcaster {
    fn recv(&mut self, signal: Pulse, _: String) {
        self.state = signal;
    }

    fn get_state(&self) -> Pulse {
        self.state
    }
}

#[derive(Default, Clone, Debug)]
struct Conjunction {
    inputs: HashMap<String, Pulse>,
}

impl Module for Conjunction {
    fn recv(&mut self, signal: Pulse, sender: String) {
        self.inputs.insert(sender, signal);
    }

    fn get_state(&self) -> Pulse {
        if self.inputs.iter().all(|(_, &signal)| signal == Pulse::High) {
            Pulse::Low
        } else {
            Pulse::High
        }
    }

    fn initial(&mut self, sender: String) {
        self.inputs.insert(sender, Pulse::Low);
    }
}
#[derive(Clone)]
struct Input {
    modules: HashMap<String, Box<dyn Module>>,
    neighbors: HashMap<String, Vec<String>>,
}

#[aoc_generator(day20)]
fn parse_input(input: &str) -> Input {
    let mut modules = HashMap::new();
    let mut neighbors = HashMap::new();
    for line in input.lines() {
        let (module, dests) = line.split_once(" -> ").unwrap();
        let (module, name): (Box<dyn Module>, String) = match module.split_at(1) {
            ("%", name) => (Box::<FlipFlop>::default(), name.trim().to_string()),
            ("&", name) => (Box::<Conjunction>::default(), name.trim().to_string()),
            ("b", _) => (Box::<Broadcaster>::default(), "broadcaster".to_string()),
            _ => unreachable!("Malformed input"),
        };
        modules.insert(name.clone(), module);
        let neigh = dests.split(", ").map(ToString::to_string).collect();
        neighbors.insert(name, neigh);
    }
    for (name, dests) in &neighbors {
        for dest in dests {
            if let Some(module) = modules.get_mut(dest) {
                module.initial(name.to_string());
            }
        }
    }
    Input { modules, neighbors }
}

#[aoc(day20, part1)]
fn part1(input: &Input) -> u64 {
    let mut modules = input.modules.clone();
    let mut high_count = 0;
    let mut low_count = 0;

    for _ in 0..1000 {
        low_count += 1;
        let mut frontier = VecDeque::new();
        frontier.push_back(("broadcaster".to_string(), Pulse::Low));

        while let Some((name, signal)) = frontier.pop_front() {
            let neighbors = input.neighbors.get(&name).unwrap();
            match signal {
                Pulse::Low => low_count += neighbors.len() as u64,
                Pulse::High => high_count += neighbors.len() as u64,
            };

            for n in neighbors {
                if let Some(module) = modules.get_mut(n) {
                    if !module.will_send(signal) {
                        continue;
                    }
                    module.recv(signal, name.clone());
                    frontier.push_back((n.to_string(), module.get_state()));
                }
            }
        }
    }

    high_count * low_count
}

#[aoc(day20, part2)]
fn part2(input: &Input) -> u64 {
    let mut modules = input.modules.clone();
    let mut cycle = Vec::new();
    for i in 0..4096 {
        let mut frontier = VecDeque::new();
        frontier.push_back(("broadcaster".to_string(), Pulse::Low));
        while let Some((name, signal)) = frontier.pop_front() {
            for n in input.neighbors.get(&name).unwrap() {
                if n == "rx" || n == "th" {
                    if signal == Pulse::High {
                        cycle.push(i + 1u64);
                    }
                    continue;
                }
                if let Some(module) = modules.get_mut(n) {
                    if !module.will_send(signal) {
                        continue;
                    }
                    module.recv(signal, name.clone());
                    frontier.push_back((n.to_string(), module.get_state()));
                }
            }
        }
    }
    cycle.into_iter().reduce(lcm).unwrap()
}
