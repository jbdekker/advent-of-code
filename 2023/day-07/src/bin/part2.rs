use itertools::Itertools;
use std::collections::BTreeMap;
use std::cmp::Ordering;

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    bid: i32,
    value: i32,
}

impl Hand {
    fn cmp(&self, other: &Hand, card_values: &BTreeMap<char, i32>) -> Ordering {
        if self.value > other.value {
            Ordering::Greater
        } else if self.value < other.value {
            Ordering::Less
        } else {
            for (a, b) in self.cards.iter().zip(other.cards.iter())
                {
                    println!("{a:?} {b:?}");
                    let value_a = *card_values.get(&a).unwrap();
                    let value_b = *card_values.get(&b).unwrap();
                    if value_a > value_b {
                        return Ordering::Greater
                    } else if value_a < value_b {
                        return Ordering::Less
                    }
                }
            println!(">>>> ORDERING::EQUAL <<<<");
            dbg!(&self);
            dbg!(other);
            Ordering::Equal
        }
    }
}

fn process(input: &str) -> i32 {
    let card_values = BTreeMap::from([
        ('A', 12),
        ('K', 11),
        ('Q', 10),
        ('J', -1),
        ('T', 8),
        ('9', 7),
        ('8', 6),
        ('7', 5),
        ('6', 4),
        ('5', 3),
        ('4', 2),
        ('3', 1),
        ('2', 0),
    ]);

    let mut hands = input
        .lines()
        .into_iter()
        .map(|line| {
            let x: Vec<_> = line.split_whitespace().collect();
            
            
            let hand_value = card_values.iter().map(|(c, _)| {
                let modified_cards = x[0].replace("J", &c.to_string());
                let mut card_counts: Vec<(char, i32)> = modified_cards
                    .chars()
                    .sorted()
                    .group_by(|&k| k)
                    .into_iter()
                    .map(|(k, v)| (k, v.count() as i32))
                    .collect();

                card_counts.sort_by(|&(_, a), &(_, b)| b.cmp(&a) );

                let max_card_count = card_counts[0].1;

                if max_card_count == 5 { // five of a kind
                    6
                } else if max_card_count == 4 { // four of a kind
                    5
                } else if (
                    card_counts[0..2].iter().map(|(_, b)| b).collect::<Vec<_>>()) == vec![&3, &2]
                {
                    4
                } else if max_card_count == 3 { // three of a kind
                    3
                } else if (
                    card_counts[0..2].iter().map(|(_, b)| b).collect::<Vec<_>>()) == vec![&2, &2]
                { // two pair
                    2
                } else if max_card_count == 2 {  // one pair
                    1
                } else { // high card
                    0
                }
            }).max().unwrap();

            Hand {
                cards: x[0].chars().collect(),
                bid: x[1].parse::<i32>().unwrap(),
                value: hand_value,
            }
        })
        .collect::<Vec<Hand>>();

    hands.sort_by(|a, b| a.cmp(b, &card_values));
    
    hands.iter().enumerate().map(|(i, hand)| {
        let rank = (i+1) as i32;
        dbg!(&hand);
        dbg!(&rank);
        println!();
        rank * hand.bid
    }).sum()
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
        assert_eq!(result, 5905)
    }
}
