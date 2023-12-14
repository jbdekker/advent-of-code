use memoize::memoize;
use std::collections::BTreeMap;
use std::io::{stdout, Write};

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
    let grid = (0..grid[0].len())
                .map(|x| (0..grid.len()).rev().map(|y| grid[y][x]).collect())
                .collect::<Vec<Vec<char>>>();
    grid
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

fn load(grid: &Vec<Vec<char>>) -> usize {
    (0..grid.len()).map(|y| (0..grid[0].len()).map(|x| {
        match grid[y][x] {
            'O' => grid.len() - y,
            _ => 0,
        }
    }).sum::<usize>()).sum()
}

fn detect_cycle(list: &Vec<usize>, max_length: usize) -> Option<usize> {
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
    let mut stdout = stdout();
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.trim().chars().collect::<Vec<_>>()).collect();
    // let mut results: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut results: BTreeMap<usize, usize> = BTreeMap::new();
    let mut _res = 0;
    let mut load_north: usize = 0;
    // let spin_cycles = 1e9 as usize;
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

        load_north = load(&grid);
        results.insert(c, load_north);

        if c % 1000 == 0 {
            stdout.flush().unwrap();
            print!("\rSpin cycle #{}, res={}", c, load_north);

            let detected_cycle = detect_cycle(&results.values().cloned().collect(), 100);

            match detected_cycle {
                Some(n) => {
                    results.retain(|&k,_| k > c - n);
                    let results: BTreeMap<usize, usize> = results.iter().map(|(&k, &v)| (k % n, v)).collect();
                    dbg!(&results);
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