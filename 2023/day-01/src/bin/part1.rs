fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            ((line.chars().find(|x| x.is_ascii_digit()).unwrap() as u8 - b'0') * 10
                + (line.chars().rev().find(|c| c.is_ascii_digit()).unwrap() as u8 - b'0'))
                as usize
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet",
        );
        assert_eq!(result, 142)
    }
}
