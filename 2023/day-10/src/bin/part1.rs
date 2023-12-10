use std::thread;

fn main() {
    let num: u64 = 100_000;
    let input = include_str!("input.txt");

    let output = thread::Builder::new()
        .stack_size(num as usize * 0xFF)
        .spawn(move || process(input))
        .unwrap()
        .join()
        .unwrap();
    // let output = process(input);
    dbg!(output);
}

#[derive(Debug, Clone)]
enum PipeType {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    StartingPosition,
    Ground,
}

#[derive(Debug, Clone)]
struct Pipe {
    pipetype: PipeType,
    visited: bool,
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

fn valid_steps(pipe: &PipeType) -> Vec<(i32, i32)> {
    match pipe {
        PipeType::Vertical => vec![(0, 1), (0, -1)],
        PipeType::Horizontal => vec![(-1, 0), (1, 0)],

        PipeType::NorthEast => vec![(0, -1), (1, 0)],
        PipeType::NorthWest => vec![(0, -1), (-1, 0)],

        PipeType::SouthEast => vec![(0, 1), (1, 0)],
        PipeType::SouthWest => vec![(0, 1), (-1, 0)],

        PipeType::StartingPosition => vec![(1, 0), (0, 1), (-1, 0), (0, -1)],

        PipeType::Ground => panic!("should never happen, standing on ground!"),
    }
}

fn valid_starting_steps(pipe_grid: &Vec<Vec<Pipe>>, position: &Point) -> Vec<(i32, i32)> {
    let mut steps: Vec<(i32, i32)> = Vec::new();

    if position.x < pipe_grid[0].len() {
        match pipe_grid[position.y][position.x + 1].pipetype {
            PipeType::Horizontal | PipeType::NorthWest | PipeType::SouthWest => steps.push((1, 0)),
            _ => (),
        }
    }

    if position.x > 0 {
        match pipe_grid[position.y][position.x - 1].pipetype {
            PipeType::Horizontal | PipeType::NorthEast | PipeType::SouthEast => steps.push((-1, 0)),
            _ => (),
        }
    }

    if position.y < pipe_grid.len() {
        match pipe_grid[position.y + 1][position.x].pipetype {
            PipeType::Vertical | PipeType::NorthWest | PipeType::NorthEast => steps.push((0, 1)),
            _ => (),
        }
    }

    if position.y > 0 {
        match pipe_grid[position.y - 1][position.x].pipetype {
            PipeType::Horizontal | PipeType::SouthEast | PipeType::SouthWest => steps.push((0, -1)),
            _ => (),
        }
    }

    steps
}

fn step(pipe_grid: &mut Vec<Vec<Pipe>>, position: Point, step_nr: usize) -> usize {
    let max_x = pipe_grid[0].len();
    let max_y = pipe_grid.len();

    dbg!(&position);

    if position.x >= max_x || position.y >= max_y {
        panic!("out of bounds!");
    }

    let this_pipe: &mut Pipe = &mut pipe_grid[position.y][position.x];

    match this_pipe.visited {
        true => {
            println!("Reached visited point {this_pipe:?}, Done in {step_nr} steps!");
            return step_nr; // done!
        }
        false => this_pipe.visited = true,
    }

    dbg!(&this_pipe.pipetype);

    let valid_steps = match this_pipe.pipetype {
        PipeType::Ground => panic!("should never happen, standing on ground!"),
        PipeType::StartingPosition => valid_starting_steps(&pipe_grid, &position),
        _ => valid_steps(&this_pipe.pipetype),
    };
    dbg!(&valid_steps);

    valid_steps
        .into_iter()
        .filter_map(|p| {
            dbg!(&p);
            dbg!(&step_nr);

            if p.0 == 1 && position.x == max_x - 2 {
                return None;
            }
            if p.0 == -1 && position.x == 0 {
                return None;
            }
            if p.1 == 1 && position.y == max_y - 2 {
                return None;
            }
            if p.1 == -1 && position.y == 0 {
                return None;
            }
            Some(step(
                pipe_grid,
                Point {
                    x: (position.x as i32 + p.0) as usize,
                    y: (position.y as i32 + p.1) as usize,
                },
                step_nr + 1,
            ))
        })
        .max()
        .unwrap()
}

fn process(input: &str) -> usize {
    let mut start: Point = Point { x: 0, y: 0 };
    let mut grid: Vec<Vec<Pipe>> = input
        .lines()
        .enumerate()
        .map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(|(x, c)| {
                    let pipe = Pipe {
                        pipetype: match c {
                            '|' => PipeType::Vertical,
                            '-' => PipeType::Horizontal,
                            'L' => PipeType::NorthEast,
                            'J' => PipeType::NorthWest,
                            '7' => PipeType::SouthWest,
                            'F' => PipeType::SouthEast,
                            '.' => PipeType::Ground,
                            'S' => {
                                start = Point { x, y };
                                PipeType::StartingPosition
                            }
                            _ => panic!("should never happen, got char {c}!"),
                        },
                        visited: false,
                    };
                    pipe
                })
                .collect()
        })
        .collect();

    dbg!(&grid);

    step(&mut grid, start, 0) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
        );
        assert_eq!(result, 8)
    }
}
