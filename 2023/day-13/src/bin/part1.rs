use itertools::Itertools;
use itertools::EitherOrBoth::Both;
use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn find_line_reflections(line: &str, idx: usize, seen: &mut Vec<usize>) -> () {
    if idx > line.len() - 1 {
        return; // done
    }

    let left = line[..idx].chars().rev();
    let right = &line[idx..];
    let matched: Vec<bool> = left.zip_longest(right.chars()).filter_map(|r| {
        match r {
            Both(l, r) => match l == r {true => Some(true), false => Some(false),},
            _ => None,
        }
    }).collect();

    if matched.iter().all(|x| *x) && matched.len() > 0 {
        seen.push(idx);
    }

    return find_line_reflections(line, idx+1, seen);
}

fn find_block_reflection(block: &Vec<String>, find_horizontal: bool) -> Option<usize> {

    let transposed_block = transpose(block);
    let block = match find_horizontal {true => &transposed_block, false => block};

    let mut line_seen: Vec<Vec<usize>> = Vec::new();
    let _reflections: Vec<_> = block.iter().map(|line| {
        let mut seen: Vec<usize> = Vec::new();
        find_line_reflections(line, 1, &mut seen);
        line_seen.push(seen);
    }).collect();

    let sets: Vec<HashSet<_>> = line_seen.iter()
        .map(|list| list.iter().cloned().collect())
        .collect();

    for &num in &sets[0] {
        if sets.iter().all(|s| s.contains(&num)) {
            match find_horizontal {
                true => return Some(100 * num),
                false => return Some(num),
            }
        }
    }

    return None

}

fn transpose(input: &Vec<String>) -> Vec<String> {
    (0..input[0].len()).map(|i| {
        input.iter()
            .map(|s| s.chars().nth(i).unwrap_or(' ')) // Replace `unwrap_or(' ')` with another character if needed
            .collect()
    }).collect()
}

fn process(input: &str) -> usize {
    let blocks: Vec<_> = input.split("\n\n").collect();

    let result: usize = blocks.iter().map(|block| {
        let lines: Vec<String> = block.lines().filter_map(|line| {
            let line = line.trim();
            
            match line {
                "" => None,
                _ => Some(line.to_string())
            }
        }).collect();

        let res = vec![true, false].into_iter().filter_map(|v| find_block_reflection(&lines,v)).collect::<Vec<_>>();

        res[0]
    }).sum();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process("#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#");
        assert_eq!(result, 405)
    }
}
