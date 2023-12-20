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

#[derive(Debug, Clone, Copy, PartialEq)]
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
    name: String,
    mtype: ModuleType,
    destinations: Vec<String>,
    state: State,
    history: BTreeMap<String, Pulse>,
}

impl Module {
    fn init_history(&mut self, inputs: Vec<String>) -> () {
        for input in inputs.into_iter() {
            self.history.insert(input, Pulse::Low);
        }
    }
    fn process(&mut self, pulse: Pulse, origin: String) -> Option<(Vec<String>, Pulse)> {
        match self.mtype {
            ModuleType::Broadcaster => Some((self.destinations.clone(), pulse)),
            ModuleType::Button => Some((self.destinations.clone(), Pulse::Low)),
            ModuleType::Conjunction => {
                self.history.insert(origin, pulse);
                // println!(
                //     "\t> Conjunction `{}` states: #{}",
                //     self.name,
                //     self.history.len()
                // );
                // for (k, v) in self.history.iter() {
                //     println!("\t\t{} -> {:?}", k, v);
                // }
                match self.history.values().all(|v| *v == Pulse::High) {
                    true => Some((self.destinations.clone(), Pulse::Low)),
                    false => Some((self.destinations.clone(), Pulse::High)),
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

    let mut modules: Vec<(String, Module)> = Vec::new();
    modules.push((
        "button".to_string(),
        Module {
            name: "button".to_string(),
            mtype: ModuleType::Button,
            destinations: vec!["broadcaster".to_string()],
            history: BTreeMap::new(),
            state: State::None,
        },
    ));

    for ((name, mtype), destination) in module_name.iter().zip(&module_type).zip(&destinations) {
        modules.push((
            name.clone(),
            Module {
                name: name.clone(),
                mtype: mtype.clone(),
                destinations: destination.clone(),
                state: match mtype {
                    ModuleType::FlipFlop => State::Off,
                    _ => State::None,
                },
                history: BTreeMap::new(),
            },
        ));
    }

    let modules_copy = modules.clone();

    // init the conjunctions
    for (name, module) in modules.iter_mut() {
        if module.mtype == ModuleType::Conjunction {
            let inputs: Vec<String> = modules_copy
                .iter()
                .filter_map(|(n, m)| {
                    if m.destinations.contains(name) {
                        Some(n.to_string())
                    } else {
                        None
                    }
                })
                .collect();

            // dbg!(&inputs);
            module.init_history(inputs);
        }
    }

    // panic!();
    // dbg!(&modules);

    let mut n_high_pulses = 0;
    let mut n_low_pulses = 0;

    'outer: for pushes in 1..100000 {
        let mut queue: VecDeque<(String, Pulse, String)> = VecDeque::new();

        queue.push_back(("broadcaster".to_string(), Pulse::Low, "button".to_string()));

        while !queue.is_empty() {
            let (target, pulse, origin) = queue.pop_front().unwrap();

            match pulse {
                Pulse::High => n_high_pulses += 1,
                Pulse::Low => n_low_pulses += 1,
            }
            for (name, module) in modules.iter_mut() {
                if name.to_string() == target {
                    match module.process(pulse, origin.to_string()) {
                        Some((new_targets, p)) => {
                            for dest in new_targets {
                                queue.push_back((dest.clone(), p, target.clone()));
                                if dest == "vr" && pulse == Pulse::Low {
                                    println!("Origin: {}, pushes: {}", target, pushes);
                                }
                            }
                        }
                        None => (),
                    }
                }
            }
        }
    }

    dbg!(n_low_pulses);
    dbg!(n_high_pulses);

    n_low_pulses * n_high_pulses
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

    #[test]
    fn it_works_2() {
        let result = process(
            "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output",
        );
        assert_eq!(result, 11687500)
    }
}
