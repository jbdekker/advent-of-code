use itertools::Itertools;
use std::collections::BTreeSet;

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn transpose(space: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = space[0].len();
    let cols = space.len();

    (0..rows)
        .map(|x| (0..cols).map(|y| space[y][x]).collect())
        .collect()
}

fn empty_rows(space: &Vec<Vec<char>>) -> Vec<usize> {
    space
        .iter()
        .enumerate()
        .filter_map(|(i, x)| match x.iter().collect::<BTreeSet<&char>>().len() {
            1 => Some(i),
            _ => None,
        })
        .collect::<Vec<usize>>()
}

fn get_expansions(space: &Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
    (empty_rows(&space), empty_rows(&transpose(space)))
}

fn count_between(v: &Vec<usize>, a: usize, b: usize) -> usize {
    let range = match a > b {
        true => (b, a),
        false => (a, b),
    };

    let count = v.iter().filter(|x| *x > &range.0 && *x < &range.1).count();

    match count {
        0 => 0,
        _ => count * (1000_000 - 1),
    }
}

fn process(input: &str) -> i64 {
    let space = input
        .lines()
        .map(|a| a.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let galaxies = space
        .iter()
        .enumerate()
        .map(|(y, a)| {
            a.iter()
                .enumerate()
                .filter_map(|(x, b)| match b {
                    '.' => None,
                    _ => Some((x as i64, y as i64)),
                })
                .collect::<Vec<(i64, i64)>>()
        })
        .flatten()
        .collect::<Vec<(i64, i64)>>();

    let (rows, cols) = get_expansions(&space);

    let sum_of_distances = galaxies
        .iter()
        .combinations(2)
        .map(|v| {
            i64::abs(v[0].0 - v[1].0)
                + count_between(&cols, v[0].0 as usize, v[1].0 as usize) as i64
                + i64::abs(v[0].1 - v[1].1)
                + count_between(&rows, v[0].1 as usize, v[1].1 as usize) as i64
        })
        .sum::<i64>();

    sum_of_distances
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        );
        assert_eq!(result, 1030)
    }
}
