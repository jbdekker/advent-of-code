use std::fs::read_to_string;
use regex::Regex;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn string_to_number_repr(word: &str) -> String {
    match word {
        "one" => String::from("1"),
        "two" => String::from("2"),
        "three" => String::from("3"),
        "four" => String::from("4"),
        "five" => String::from("5"),
        "six" => String::from("6"),
        "seven" => String::from("7"),
        "eight" => String::from("8"),
        "nine" => String::from("9"),
        _ => word.to_string(),
    }
}

fn extract_numbers(line: &str) -> i32 {
    let re: Regex = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").unwrap();

    let mut first: String = String::new();
    let mut last: String = String::new();
    for (i, caps) in re.captures_iter(line).enumerate() {
        if i == 0 {
            first = caps[0].to_string();
        }
        last = caps[0].to_string();

    }


    let mut result: String = string_to_number_repr(&first);
    result.push_str(&string_to_number_repr(&last));

    println!("Line: {} -> {} {} -> {}", line, first, last, result);

    return result.parse::<i32>().unwrap();
}

fn main() {
    let fp: &str = "./src/input";

    let lines: Vec<String> = read_lines(fp);

    let mut numbers: Vec<i32> = Vec::new();
    for line in &lines {
        numbers.push(extract_numbers(&line));
    }
    
    // for number in &numbers {
    //     println!("{}", number);
    // }
    let result: i32 = numbers.iter().sum();
    println!("Answer: {}", result);

}
