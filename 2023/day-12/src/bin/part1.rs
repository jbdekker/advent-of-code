use std::collections::HashSet;


fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn number_of_options(record: &str, n: usize) -> usize {
    let delta: usize = record.len() - n;
    let n_options: usize = (0..delta+1).into_iter().map(|i| {
        let mut masked_record = record.to_string();
        masked_record.replace_range(i..i+n, "");

        match masked_record.contains('#') {
            true => 0,
            false => 1,
        }
    }
    ).sum();

    n_options
}

fn process(input: &str) -> usize {

    let result = input.lines().into_iter().map(|line| {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let records: Vec<_> = parts[0].split('.').filter(|p| !p.is_empty()).collect();
        let groups: Vec<_> = parts[1].split(',').map(|x| x.parse::<usize>().unwrap()).collect();

        dbg!(&records);
        dbg!(&groups);

        if records.len() == groups.len() {
            let n_options = records.iter().zip(groups.clone()).map(|(a, b)| number_of_options(a, b)).product::<usize>();
            dbg!(n_options)
        } else {
        
            let records = records.into_iter().map(|g| {
                let n = g.chars().collect::<HashSet<_>>().len();
                let x = match n {
                    1 => g.len(),
                    _ => 1,
                };
            }).collect::<Vec<_>>();
            0
        }
        }).sum();

    dbg!(&input);
    dbg!(&result);
    result
    
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
