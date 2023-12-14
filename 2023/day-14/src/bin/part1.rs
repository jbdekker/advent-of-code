fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

struct Point {
    x: usize,
    y: usize,
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

fn process(input: &str) -> usize {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.trim().chars().collect::<Vec<_>>()).collect();

    let res = (0..grid.len()).map(|y| (0..grid[0].len()).map(|x| {
        match grid[y][x] {
            'O' => roll_up(&mut grid, Point {x, y}),
            _ => 0,
        }}).sum::<usize>()).sum();

    res
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
        assert_eq!(result, 136)
    }
}
