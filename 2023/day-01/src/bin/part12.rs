use std::fs::read_to_string;
use std::collections::HashMap;
use regex::Regex;


fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}


fn str_to_num(word: &str) -> i32 {
    let mapping = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    mapping.get(&word as &str).cloned().unwrap_or_else(|| word.parse::<i32>().unwrap())
}


fn first_match(re: Regex, line: &str) -> String {
    re.captures(line).unwrap()[0].to_string()
}


fn extract_number(line: &str) -> i32 {
    let re: Regex = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let re_inv: Regex = Regex::new(r"\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();
    let line_reversed: String = line.chars().rev().collect();

    let first: String = first_match(re.clone(), &line);
    let last: String = first_match(re_inv.clone(), &line_reversed).chars().rev().collect();

    10 * str_to_num(&first) + str_to_num(&last)
}


fn main() {
    let fp: &str = "./src/bin/input.txt";
    let lines: Vec<String> = read_lines(fp);

    let mut numbers: Vec<i32> = Vec::new();
    for line in &lines {
        numbers.push(extract_number(&line));
    }
    
    let result: i32 = numbers.iter().sum();
    println!("Answer: {}", result);

}
