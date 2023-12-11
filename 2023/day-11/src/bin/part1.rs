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

    // expand space-rows
    space[a..b]
        .iter()
        .filter_map(|x| match x.iter().collect::<BTreeSet<&char>>().len() {
            1 => Some(1),
            _ => None,
        })
        .count()
}

fn process(input: &str) -> i32 {
    // generate space
    let space = input
        .lines()
        .map(|a| a.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    dbg!(&space);

    // expand space-rows
    let space = space
        .iter()
        .map(|x| match x.iter().collect::<BTreeSet<&char>>().len() {
            1 => vec![x.clone(), x.clone()],
            _ => vec![x.clone()],
        })
        .flatten()
        .collect::<Vec<Vec<char>>>();

    dbg!(&space);

    // expand space-columns
    let space = transpose_space(space);
    let space = space
        .iter()
        .map(|x| match x.iter().collect::<BTreeSet<&char>>().len() {
            1 => vec![x.clone(), x.clone()],
            _ => vec![x.clone()],
        })
        .flatten()
        .collect::<Vec<Vec<char>>>();
    let space = transpose_space(space);

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
                    _ => Some((x as i32, y as i32)),
                })
                .collect::<Vec<(i32, i32)>>()
        })
        .flatten()
        .collect::<Vec<(i32, i32)>>();

    dbg!(&galaxies);

    let sum_of_distances = galaxies
        .iter()
        .combinations(2)
        .map(|v| i32::abs(v[0].0 - v[1].0) + i32::abs(v[0].1 - v[1].1))
        .sum::<i32>();

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
