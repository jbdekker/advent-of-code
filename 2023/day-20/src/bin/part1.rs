use std::collections::{BTreeMap, VecDeque};

use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug, Clone, PartialEq, Copy)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Clone, Copy)]
enum State {
    On,
    Off,
    None,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum ModuleType {
    Button,
    Broadcaster,
    FlipFlop,
    Conjunction,
}

#[derive(Debug, Clone)]
struct Module {
    mtype: ModuleType,
    name: String,
    destinations: Vec<String>,
    state: State,
    history: BTreeMap<String, Pulse>,
}

impl Module {
    fn process(&mut self, pulse: Pulse, origin: String) -> Option<(Vec<String>, Pulse)> {
        match self.mtype {
            ModuleType::Broadcaster => Some((self.destinations.clone(), pulse)),
            ModuleType::Button => Some((self.destinations.clone(), Pulse::Low)),
            ModuleType::Conjunction => {
                self.history.insert(origin, pulse);
                match self.history.values().all(|v| *v == Pulse::High) {
                    true => Some((self.destinations.clone(), Pulse::Low)),
                    false => Some((self.destinations.clone(), Pulse::Low)),
                }
            }
            ModuleType::FlipFlop => match pulse {
                Pulse::High => None,
                Pulse::Low => match self.state {
                    State::Off => {
                        self.state = State::On;
                        Some((self.destinations.clone(), Pulse::High))
                    }
                    State::On => {
                        self.state = State::Off;
                        Some((self.destinations.clone(), Pulse::Low))
                    }
                    State::None => None,
                },
            },
            _ => panic!(),
        }
    }
}

fn process(input: &str) -> usize {
    let (modules, destinations): (Vec<&str>, Vec<&str>) = input
        .lines()
        .map(|line| line.split("->").collect_tuple::<(&str, &str)>().unwrap())
        .unzip();

    let (module_name, module_type): (Vec<String>, Vec<ModuleType>) = modules
        .iter()
        .map(|m| {
            let x = m.trim();
            match x.trim() {
                x if x == "broadcaster" => (x.to_string(), ModuleType::Broadcaster), // broadcaster
                x if x.contains("%") => (x.replace("%", ""), ModuleType::FlipFlop),  // flip-flop
                x if x.contains("&") => (x.replace("&", ""), ModuleType::Conjunction), // conjunction
                _ => panic!("Should be unreachable, got {}", m.trim()),
            }
        })
        .unzip();

    let destinations: Vec<Vec<String>> = destinations
        .iter()
        .map(|d| d.split(",").map(|x| x.trim().to_string()).collect())
        .collect();

    let mut modules: Vec<Module> = Vec::new();
    modules.push(Module {
        name: "button".to_string(),
        mtype: ModuleType::Button,
        destinations: vec!["broadcaster".to_string()],
        history: BTreeMap::new(),
        state: State::None,
    });

    let mut configurations: BTreeMap<String, &mut Module> = BTreeMap::new();

    for ((name, mtype), destination) in module_name.iter().zip(&module_type).zip(&destinations) {
        modules.push(Module {
            mtype: mtype.clone(),
            name: name.clone(),
            destinations: destination.clone(),
            state: match mtype {
                ModuleType::FlipFlop => State::Off,
                _ => State::None,
            },
            history: BTreeMap::new(),
        });

        configurations.insert(name.clone(), &modules.last().unwrap());
    }

    // init the conjunctions
    for (name, module) in configurations.iter_mut() {
        for destination in module.destinations.iter() {
            if configurations.get(destination).unwrap().mtype == ModuleType::Conjunction {
                configurations
                    .entry(destination.to_string())
                    .and_modify(|f| {
                        f.history.insert(name.clone(), Pulse::Low);
                    });
            }
        }
    }

    dbg!(&configurations);

    let mut queue: VecDeque<&Module> = VecDeque::new();
    // loop {}

    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a",
        );
        assert_eq!(result, 32000000)
    }
}
