use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug, Clone, Copy)]
struct Node {
    left: i32,
    right: i32,
}

impl Node {
    fn diff(&self) -> i32 {
        self.right - self.left
    }
}

fn node_diffs(nodes: &Vec<Node>) -> Vec<i32> {
    nodes.iter().map(|n| n.diff()).collect()
}

fn process(input: &str) -> i32 {
    let data: Vec<Vec<_>> = input
        .lines()
        .into_iter()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .tuple_windows()
                .map(|(a, b)| Node { left: a, right: b })
                .collect()
        })
        .collect::<Vec<Vec<_>>>();

    let mut extrapolated_values: Vec<i32> = Vec::new();
    for list in data.iter() {
        let mut stack: Vec<Vec<Node>> = Vec::from([list.clone()]);
        let mut diffs = node_diffs(&list);

        while !diffs.iter().all(|i| i == &0) {
            let new_nodes: Vec<Node> = diffs
                .iter()
                .tuple_windows()
                .map(|(a, b)| Node {
                    left: *a,
                    right: *b,
                })
                .collect();

            diffs = node_diffs(&new_nodes);
            stack.push(new_nodes.clone());
        }

        let extrapolated_value: i32 = stack
            .iter()
            .map(|list| list.into_iter().next().copied().unwrap())
            .rev()
            .fold(0 as i32, |acc, n| n.left - acc);

        extrapolated_values.push(extrapolated_value);
    }

    extrapolated_values.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
        );
        assert_eq!(result, 2)
    }
}
