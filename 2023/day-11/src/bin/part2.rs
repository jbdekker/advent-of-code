use itertools::Itertools;
use std::collections::BTreeSet;

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn transpose_space(space: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = space[0].len();
    let cols = space.len();

    (0..rows)
        .map(|x| (0..cols).map(|y| space[y][x]).collect())
        .collect()
}

fn num_empty(space: &Vec<Vec<char>>, a: usize, b: usize, transpose: bool) -> usize {
    let space = match transpose {
        true => transpose_space(space.clone()),
        false => space.clone(),
    };

    let (a, b) = match a > b {
        true => (b, a),
        false => (a, b),
    };

    // expand space-rows
    space[a..b]
        .iter()
        .filter_map(|x| match x.iter().collect::<BTreeSet<&char>>().len() {
            1 => Some(1000000 - 1),
            _ => None,
        })
        .sum()
}

fn process(input: &str) -> i64 {
    // generate space
    let space = input
        .lines()
        .map(|a| a.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    for line in space.iter() {
        println!("{}", line.into_iter().collect::<String>());
    }
    // dbg!(&space);

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

    let sum_of_distances = galaxies
        .iter()
        .combinations(2)
        .map(|v| {
            i64::abs(v[0].0 - v[1].0)
                + num_empty(&space, v[0].0 as usize, v[1].0 as usize, true) as i64
                + i64::abs(v[0].1 - v[1].1)
                + num_empty(&space, v[0].1 as usize, v[1].1 as usize, false) as i64
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
        assert_eq!(result, 374)
    }
}
