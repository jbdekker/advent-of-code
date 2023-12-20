use std::collections::BTreeMap;

use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Clone)]
enum ModuleType {
    Button,
    Broadcaster,
    FlipFlop,
    Conjunction,
}

#[derive(Clone)]
struct Module {
    mtype: ModuleType,
    name: String,
    destinations: Vec<String>,
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
                x if x.contains("%") => (x.replace("&", ""), ModuleType::FlipFlop),  // flip-flop
                x if x.contains("&") => (x.replace("&", ""), ModuleType::Conjunction), // conjunction
                _ => panic!("Should be unreachable, got {}", m.trim()),
            }
        })
        .unzip();

    let destinations: Vec<Vec<String>> = destinations
        .iter()
        .map(|d| d.split(",").map(|x| x.trim().to_string()).collect())
        .collect();

    let configurations: BTreeMap<String, Module> = module_name
        .iter()
        .zip(&module_type)
        .zip(&destinations)
        .map(|((name, mtype), destination)| {
            (
                name.clone(),
                Module {
                    mtype: mtype.clone(),
                    name: name.clone(),
                    destinations: destination.clone(),
                },
            )
        })
        .collect();

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
