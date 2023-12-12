fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn number_of_options(record: &str, nums: &Vec<usize>) -> usize {
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

    let next_char = record.chars().nth(0).unwrap();
    if ['.', '?'].into_iter().any(|s| s == next_char) {
        result += number_of_options(&record[1..], &nums)
    }

    if ['#', '?'].into_iter().any(|s| s == next_char) {
            // must be enough springs left
            if record.len() >= nums[0] && !record[..nums[0]].contains('.') && (record.len() == nums[0] || record.chars().nth(nums[0]).unwrap() != '#') {
                result += number_of_options(&record[nums[0]+1..], *nums.clone()[1..]);    
            }
                result += 0;
        }

    return result;
}

fn process(input: &str) -> usize {
    let result: usize = input.lines().into_iter().map(|line| {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        
        let record = parts[0];
        let numbers: Vec<_> = parts[1].split(',').map(|x| x.parse::<usize>().unwrap()).collect();

        let options = number_of_options(&record, &numbers);

        dbg!(&record);
        dbg!(&numbers);

        options
    }).sum();

    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process("???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1");
        assert_eq!(result, 21)
    }
}
