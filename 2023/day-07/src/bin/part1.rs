use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug)]
struct Card {
    card: char,
}

impl Card {
    fn value(&self, map: &BTreeMap<char, i32>) -> i32 {
        *map.get(&self.card).expect("no es bueno!")
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: i32,
    value: i32,
}

impl Hand {
    fn value(&self) {}
}

fn process(input: &str) -> i32 {
    let card_values = BTreeMap::from([
        ('A', 11),
        ('K', 10),
        ('Q', 9),
        ('J', 8),
        ('T', 7),
        ('8', 6),
        ('7', 5),
        ('6', 4),
        ('5', 3),
        ('4', 2),
        ('3', 1),
        ('2', 0),
    ]);

    let hands = input
        .lines()
        .into_iter()
        .map(|line| {
            let x: Vec<_> = line.split_whitespace().collect();

            let card_counts: BTreeMap<char, i32> = x[0]
                .chars()
                .sorted()
                .group_by(|&k| k)
                .into_iter()
                .map(|(k, v)| (k, v.count() as i32))
                .collect();

            let max_card_count = card_counts.values().max().unwrap();
            let hand_value = {
                if *max_card_count == 5 {
                    6
                } else if *max_card_count == 4 {
                    5
                } else if dbg!(
                    card_counts
                        .values()
                        .rev()
                        .take(2)
                        .collect::<BTreeSet<&i32>>()
                        .intersection(&BTreeSet::from([&3, &2]))
                        .collect()
                        == []
                ) {
                    4
                } else if *max_card_count == 3 {
                    3
                } else if dbg!(
                    card_counts
                        .values()
                        .rev()
                        .take(2)
                        .collect::<BTreeSet<&i32>>()
                        .intersection(&BTreeSet::from([&2]))
                        .collect()
                        == []
                ) {
                    2
                } else if *max_card_count == 2 {
                    1
                } else {
                    0
                }
            };

            Hand {
                cards: x[0].chars().map(|c| Card { card: c }).collect(),
                bid: x[1].parse::<i32>().unwrap(),
                value: hand_value,
            }
        })
        .collect::<Vec<Hand>>();

    // order the hands by value

    dbg!(hands);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        );
        assert_eq!(result, 6440)
    }
}
