use regex::Regex;
use std::collections::BTreeMap;

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug)]
enum Direction {
    LEFT = 1,
    RIGHT = 2,
}

#[derive(Debug)]
struct Node {
    // name: String,
    left: String,
    right: String,
}

fn process(input: &str) -> usize {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let instructions: Vec<Direction> = parts[0]
        .trim()
        .replace("\n", "")
        .chars()
        .map(|c| match c {
            'L' => Direction::LEFT,
            'R' => Direction::RIGHT,
            _ => panic!("Should never happen, got char {c:?}"),
        })
        .collect();

    let re = Regex::new(r"([A-Z0-9]+) = \(([A-Z0-9]+), ([A-Z0-9]+)\)").unwrap();

    let nodes: BTreeMap<String, Node> = parts[1]
        .trim()
        .lines()
        .map(|line| {
            // dbg!(&line);
            let capt = re.captures(line).unwrap();

            (
                capt[1].to_string(),
                Node {
                    left: capt[2].to_string(),
                    right: capt[3].to_string(),
                },
            )
        })
        .collect();

    // dbg!(&instructions);
    // dbg!(&nodes);

    let mut cur_node_names: Vec<&String> = nodes
        .keys()
        .filter(|k| k.chars().last().unwrap() == 'A')
        .collect();

    // dbg!(&cur_node_names);

    let mut checked_nodes: BTreeMap<&String, usize> = BTreeMap::new();
    let mut i = 0;
    while !cur_node_names
        .iter()
        .all(|x| x.chars().last().unwrap() == 'Z')
    {
        // println!("{cur_node_names:?}");
        let cur_nodes: Vec<&Node> = cur_node_names
            .into_iter()
            .map(|name| nodes.get(name).unwrap())
            .collect();

        cur_node_names = cur_nodes
            .into_iter()
            .map(|node| match instructions[i % instructions.len()] {
                Direction::LEFT => &node.left,
                Direction::RIGHT => &node.right,
            })
            .collect();

        checked_nodes.extend(
            cur_node_names
                .iter()
                .filter(|name| name.chars().last().unwrap() == 'Z')
                .map(|name| (*name, i)),
        );

        i += 1;
    }

    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
        );
        assert_eq!(result, 6)
    }
}
