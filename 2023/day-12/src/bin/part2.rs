use memoize::memoize;

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

#[memoize]
fn number_of_options(record: String, nums: Vec<usize>) -> usize {
    let max = record
        .chars()
        .fold(0, |acc, c| if c == '#' || c == '?' { acc + 1 } else { acc });
    match &max < nums.iter().max().unwrap_or(&0) {
        true => return 0, //short circuit
        false => (),
    }

    if record.chars().filter(|c| *c == '#' || *c == '?').count() < nums.iter().sum::<usize>() {
        return 0; //short circuit
    }

    if record.len() == 0 {
        match nums.is_empty() {
            true => return 1,
            false => return 0,
        };
    }

    if nums.is_empty() {
        match record.contains('#') {
            true => return 0,
            false => return 1,
        };
    }

    let mut result = 0;

    let next_char = record.chars().next().unwrap();
    if ['.', '?'].into_iter().any(|s| s == next_char) {
        result += number_of_options(record[1..].to_string(), nums.clone())
    }

    if ['#', '?'].into_iter().any(|s| s == next_char) {
        if record.len() >= nums[0] && !record[..nums[0]].contains('.') {
            if record.len() == nums[0] {
                result += number_of_options(record[nums[0]..].to_string(), nums[1..].to_vec());
            } else if record.chars().nth(nums[0]).unwrap() != '#' {
                result +=
                    number_of_options(record[(nums[0] + 1)..].to_string(), nums[1..].to_vec());
            } else {
                result += 0;
            }
        }
    }

    return result;
}

fn process(input: &str) -> usize {
    let result = input
        .lines()
        .into_iter()
        .map(|line| {
            let parts = line.split_whitespace().collect::<Vec<_>>();

            let record = parts[0];
            let numbers: Vec<_> = parts[1]
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            let record = Vec::from([record; 5]).join("?");
            let numbers = vec![numbers; 5].into_iter().flatten().collect();

            number_of_options(record.to_string(), numbers)
        })
        .sum();

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1",
        );

        assert_eq!(result, 525152)
    }
}
