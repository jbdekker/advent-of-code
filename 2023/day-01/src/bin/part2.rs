const NUMBERS: [&'static str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let chars: Vec<char> = line.chars().collect();
            (0..line.len()).find_map(|i| val(&chars, i)).unwrap() * 10
                + (0..line.len()).rev().find_map(|i| val(&chars, i)).unwrap() as usize
        })
        .sum()
}

fn val(chars: &Vec<char>, i: usize) -> Option<usize> {
    chars[i]
        .is_ascii_digit()
        .then_some((chars[i] as i8 - b'0' as i8) as usize)
        .or(NUMBERS
            .iter()
            .enumerate()
            .find(|(_, needle)| chars[i..].starts_with(&needle.chars().collect::<Vec<char>>()))
            .map(|(num, _)| num + 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen",
        );
        assert_eq!(result, 281)
    }
}
