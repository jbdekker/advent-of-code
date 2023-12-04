use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let split: Vec<&str> = line.split(':').collect();
            let _game: &str = split[0].trim();

            let winning_numbers: HashSet<i32> = HashSet::from_iter(
                split[1].split('|').collect::<Vec<&str>>()[1]
                    .trim()
                    .split_whitespace()
                    .map(|x| x.trim().parse::<i32>().unwrap()),
            );

            let my_numbers: HashSet<i32> = HashSet::from_iter(
                split[1].split('|').collect::<Vec<&str>>()[0]
                    .trim()
                    .split_whitespace()
                    .map(|x| x.trim().parse::<i32>().unwrap()),
            );

            let intersect: usize = winning_numbers
                .intersection(&my_numbers)
                .collect::<Vec<&i32>>()
                .len();

            let two: i32 = 2;

            match intersect {
                0 => 0,
                _ => two.pow(intersect as u32 - 1),
            }
        })
        .sum::<i32>()
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
