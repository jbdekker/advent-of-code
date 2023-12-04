use itertools::Itertools;
use std::collections::BTreeSet;

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            let (mine, winning) = line.split(':').collect::<Vec<&str>>()[1]
                .split('|')
                .map(|x| {
                    BTreeSet::from_iter(x.split_whitespace().map(|y| y.parse::<i32>().unwrap()))
                })
                .collect_tuple()
                .unwrap();

            let n = mine.intersection(&winning).collect::<Vec<&i32>>().len();

            match n {
                0 => None,
                _ => Some(1 << n - 1),
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(result, 13)
    }
}
