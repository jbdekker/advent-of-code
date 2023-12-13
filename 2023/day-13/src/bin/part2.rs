fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn find_folds(block: &Vec<Vec<char>>) -> Option<usize> {
    (0..block[0].len()).find_map(|i| try_fold(block, 0, i + 1, 0))
}

fn try_fold(
    block: &Vec<Vec<char>>,
    line_idx: usize,
    col_idx: usize,
    missing: usize,
) -> Option<usize> {
    let missing = missing
        + block[line_idx][..col_idx]
            .iter()
            .rev()
            .zip(block[line_idx][col_idx..].iter())
            .filter_map(|(a, b)| match a != b {
                true => Some(true),
                false => None,
            })
            .collect::<Vec<_>>()
            .len();

    if missing > 1 {
        return None;
    }

    match line_idx + 1 == block.len() {
        true => match missing == 1 {
            true => Some(col_idx),
            false => None,
        },
        false => try_fold(block, line_idx + 1, col_idx, missing),
    }
}

fn find(block: &Vec<Vec<char>>) -> usize {
    match find_folds(block) {
        Some(v) => v,
        None => {
            let block_transposed = (0..block[0].len())
                .map(|col| (0..block.len()).map(|row| block[row][col]).collect())
                .collect();

            match find_folds(&block_transposed) {
                Some(v) => 100 * v,
                None => panic!("No fold found!"),
            }
        }
    }
}

fn process(input: &str) -> usize {
    let blocks: Vec<_> = input.split("\n\n").collect();

    let result: usize = blocks
        .iter()
        .map(|block| {
            let block: Vec<Vec<char>> = block
                .lines()
                .filter_map(|line| {
                    let line = line.trim();

                    match line {
                        "" => None,
                        _ => Some(line.chars().collect()),
                    }
                })
                .collect();

            find(&block)
        })
        .sum();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "#.##..##.
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
#....#..#",
        );
        assert_eq!(result, 400)
    }
}
