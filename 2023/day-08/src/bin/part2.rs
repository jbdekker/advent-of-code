use regex::Regex;
use std::collections::HashMap;

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

#[derive(Debug, Hash, Eq)]
struct Node {
    name: String,
    left: String,
    right: String,
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.name == other.name
    }
}

fn gcd(a: usize, b: usize) -> usize {
    // greatest common denominator
    assert!(a > b, "a must be larger than b");

    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}

fn lcm(a: usize, b: usize) -> usize {
    // least common multiple
    (a * b) / gcd(a, b)
}

fn lcm_vec(x: Vec<usize>) -> usize {
    // least common multiple over vector
    x.into_iter().fold(1, |a, b| match a > b {
        true => lcm(a, b),
        false => lcm(b, a),
    })
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

    let node_map: HashMap<String, Node> = parts[1]
        .trim()
        .lines()
        .map(|line| {
            // dbg!(&line);
            let capt = re.captures(line).unwrap();
            (
                capt[1].to_string(),
                Node {
                    name: capt[1].to_string(),
                    left: capt[2].to_string(),
                    right: capt[3].to_string(),
                },
            )
        })
        .collect();

    let mut nodes_todo: Vec<&Node> = node_map
        .values()
        .filter(|k| k.name.chars().last().unwrap() == 'A')
        .collect();

    let mut nodes_done: HashMap<&Node, usize> = HashMap::new();

    let mut i = 0;
    while !nodes_todo.is_empty() {
        // Nodes ending on a 'Z' are done, store the number of steps it took
        nodes_done.extend(nodes_todo.clone().into_iter().filter_map(|node| {
            match node.name.chars().last().unwrap() == 'Z' {
                true => Some((node, i)),
                false => None,
            }
        }));

        // take a step for the nodes that don't end on a Z
        nodes_todo = nodes_todo
            .into_iter()
            .filter_map(|node| match node.name.chars().last().unwrap() {
                'Z' => None,
                _ => match instructions[i % instructions.len()] {
                    Direction::LEFT => Some(node_map.get(&node.left).unwrap()),
                    Direction::RIGHT => Some(node_map.get(&node.right).unwrap()),
                },
            })
            .collect();

        i += 1;
    }

    lcm_vec(nodes_done.into_values().collect())
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
