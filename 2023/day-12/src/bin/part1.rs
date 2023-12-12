fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn number_of_options(record: &str, nums: &Vec<usize>) -> usize {
    dbg!(&record);
    dbg!(&nums);

    // if dbg!(record.len() + 1) < dbg!(nums.iter().sum::<usize>() + nums.len()) {
    //     println!("record is too short");
    //     return 0;
    // }

    if record.len() == 0 {
        match nums.is_empty() {
            true => return dbg!(1),
            false => return dbg!(0),
        };
    }

    if nums.is_empty() {
        match record.contains('#') {
            true => return dbg!(0),
            false => return dbg!(1),
        };
    }

    let mut result = 0;

    let next_char = record.chars().next().unwrap();
    if ['.', '?'].into_iter().any(|s| s == next_char) {
        println!("Next char: {}", next_char);
        result += number_of_options(&record[1..], &nums)
    }

    if ['#', '?'].into_iter().any(|s| s == next_char) {
        // must be enough springs left
        if record.len() >= nums[0] && !record[..nums[0]].contains('.') {
            if record.len() == nums[0] {
                println!("record: {}, nums: {:?}", record, nums);
                result += number_of_options(&record[nums[0]..], &nums[1..].to_vec());
            } else {
                result += number_of_options(&record[(nums[0] + 1)..], &nums[1..].to_vec());
            }
        }
    }

    return result;
}

fn process(input: &str) -> usize {
    let result: usize = input
        .lines()
        .into_iter()
        .map(|line| {
            let parts = line.split_whitespace().collect::<Vec<_>>();

            let record = parts[0];
            let numbers: Vec<_> = parts[1]
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect();

            let options = number_of_options(&record, &numbers);

            // dbg!(&record);
            // dbg!(&numbers);

            dbg!(options)
        })
        .sum();

    return result;
    // todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process("?###???????? 3,2,1");
        assert_eq!(result, 10)
    }
}
