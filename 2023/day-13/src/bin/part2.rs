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

fn find_block_reflection(block: &Vec<String>, find_horizontal: bool) -> Option<Vec<usize>> {
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

    let mut nums = Vec::new();
    for &num in &sets[0] {
        if sets.iter().all(|s| s.contains(&num)) {
            match find_horizontal {
                true => nums.push(100 * num),
                false => nums.push(num),
            }
        }
    }


    match nums.len() {
        0 => return None,
        _ => return Some(nums),
    }

}

fn transpose(input: &Vec<String>) -> Vec<String> {
    (0..input[0].len()).map(|i| {
        input.iter()
            .map(|s| s.chars().nth(i).unwrap()) // Replace `unwrap_or(' ')` with another character if needed
            .collect()
    }).collect()
}

fn flip_char(block: &Vec<String>, row: usize, col: usize) -> Vec<String> {
    let mut block = block.clone();
    let c = block[row].chars().nth(col).unwrap();

    dbg!(&row);
    dbg!(&col);
    match c {
        '#' => block[row].replace_range(col..=col, "."),
        '.' => block[row].replace_range(col..=col, "#"),
        _ => panic!("Should be unreachable, got char {c}"),
    }

    block
}

fn process(input: &str) -> usize {
    let blocks: Vec<_> = input.split("\n\n").collect();

    let result: usize = blocks.iter().enumerate().map(|(idx, block)| {
        let lines: Vec<String> = block.lines().filter_map(|line| {
            let line = line.trim();
            
            match line {
                "" => None,
                _ => Some(line.to_string())
            }
        }).collect();

        // if idx > 10 {
        //     0
        // } else {

        let res = vec![true, false].into_iter().filter_map(|v| find_block_reflection(&lines,v)).flatten().collect::<Vec<_>>();
        if res.len() > 1 {
            panic!("{res:?}");
        }
        let mut ans = 0;
        'outer: for row in 0..lines.len() {
            for col in 0..lines[0].len() {
                let flipped_lines = flip_char(&lines, row, col);
                
                let mut options = vec![true, false].into_iter().filter_map(|v| find_block_reflection(&flipped_lines,v)).flatten().collect::<Vec<_>>();

                dbg!(&options);
                dbg!(&res);
                
                options.retain(|&x| x != res[0]);
                if options.len() > 0 {
                    ans = options[0];

                    if options.len() > 1 {
                        panic!("{options:?}");
                    }
                    dbg!(&flipped_lines);
                    dbg!(&options);

                    break 'outer;
                } 

            }
        }
        ans
        
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
        assert_eq!(result, 400)
    }
}


// to-low: 32551
// to-low: 34672