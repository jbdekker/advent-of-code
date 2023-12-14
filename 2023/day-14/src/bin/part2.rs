use memoize::memoize;
use std::collections::BTreeMap;

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

#[derive (Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[memoize]
fn spin(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    (0..grid[0].len())
                .map(|x| (0..grid.len()).rev().map(|y| grid[y][x]).collect())
                .collect::<Vec<Vec<char>>>()
}

fn roll_up(grid: &mut Vec<Vec<char>>, point: Point) -> usize {
    match point.y {
        0 => grid.len(),
        _ => match grid[point.y - 1][point.x] {
                '#' | 'O' => grid.len() - point.y,
                '.' => {
                    grid[point.y][point.x] = '.';
                    grid[point.y-1][point.x] = 'O';
                    roll_up(grid, Point { x: point.x, y: point.y - 1})},
                _ => panic!("should be unreachable!"),
            }
    }
}

fn calculate_load(grid: &Vec<Vec<char>>) -> usize {
    (0..grid.len()).map(|y| (0..grid[0].len()).map(|x| {
        match grid[y][x] {
            'O' => grid.len() - y,
            _ => 0,
        }
    }).sum::<usize>()).sum()
}

fn find_cycle(list: &Vec<usize>, max_length: usize) -> Option<usize> {
    if list.len() < max_length * 2 {
        return None;
    }

    let list = &list[list.len()-max_length*2..];

    for i in 1..max_length {
        if list.iter().rev().zip(list.iter().rev().skip(i)).all(|(a, b)| a == b) == true {
            return Some(i);
        }
    }

    return None;
}

fn process(input: &str) -> usize {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.trim().chars().collect::<Vec<_>>()).collect();
    let mut results: BTreeMap<usize, usize> = BTreeMap::new();
    let mut _res = 0;
    let spin_cycles = 1e9 as usize;
    for c in 0..spin_cycles {
        for _ in 0..4 {
            _res = (0..grid.len()).map(|y| (0..grid[0].len()).map(|x| {
            match grid[y][x] {
                'O' => roll_up(&mut grid, Point {x, y}),
                _ => 0,
            }}).sum::<usize>()).sum();
            grid = spin(grid);
        }

        results.insert(c, calculate_load(&grid));

        if c % 1000 == 0 {
            let detected_cycle = find_cycle(&results.values().cloned().collect(), 100);

            match detected_cycle {
                Some(n) => {
                    results.retain(|&k,_| k > c - n);
                    let results: BTreeMap<usize, usize> = results.iter().map(|(&k, &v)| (k % n, v)).collect();
                    let ans = results.get(&((spin_cycles - 1) % n)).unwrap();
                    return *ans;
                }
                _ => continue,
            }
        }
    }
    panic!("should be unreachable!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process("O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....");
        assert_eq!(result, 64)
    }
}