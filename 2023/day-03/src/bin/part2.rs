use regex::Regex;
use std::cmp;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn is_gear(x: char) -> bool {
    match x {
        '*' => true,
        _ => false,
    }
}

fn row_range(i: usize, limit: usize) -> (usize, usize) {
    match i {
        0 => (0, cmp::min(i + 1, limit)),
        _ => (i - 1, cmp::min(i + 1, limit)),
    }
}

fn column_range(j: usize, length: usize, limit: usize) -> (usize, usize) {
    match j {
        0 => (0, cmp::min(j + length, limit)),
        _ => (j - 1, cmp::min(j + length, limit)),
    }
}

fn process(input: &str) -> usize {
    let data: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let re = Regex::new(r"(\d+)").unwrap();
    let mut gearsets: HashMap<(usize, usize), Vec<usize>> = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        let finds = re.find_iter(line);

        for fnd in finds {
            let value: usize = fnd.as_str().parse::<usize>().unwrap();

            let row_limits = row_range(i, data.len() - 1);
            let col_limits = column_range(fnd.start(), fnd.len(), data[0].len() - 1);

            for row in row_limits.0..=row_limits.1 {
                for col in col_limits.0..=col_limits.1 {
                    if is_gear(data[row][col]) {
                        gearsets
                            .entry((row, col))
                            .and_modify(|v| v.push(value))
                            .or_insert(Vec::from([value]));
                    }
                }
            }
        }
    }
    let sum_of_products: usize = gearsets
        .iter()
        .filter(|&(_k, v)| v.len() == 2)
        .map(|x| x.1[0] * x.1[1])
        .sum::<usize>();

    sum_of_products
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(result, 4361)
    }
}
