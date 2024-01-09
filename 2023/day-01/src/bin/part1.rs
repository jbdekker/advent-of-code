use regex::Regex;


fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn parse(word: &str) -> i32 {
    word.parse::<i32>().unwrap()
}

fn first_match(re: Regex, line: &str) -> String {
    re.captures(line).unwrap()[0].to_string()
}

fn extract_number(line: &str) -> i32 {
    let re: Regex = Regex::new(r"\d").unwrap();
    let line_reversed: String = line.chars().rev().collect();

    let first: String = first_match(re.clone(), &line);
    let last: String = first_match(re.clone(), &line_reversed).chars().rev().collect();

    10 * parse(&first) + parse(&last)
}


fn process(input: &str) -> i32 {
    input.lines().map(|line| extract_number(line)).sum()
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