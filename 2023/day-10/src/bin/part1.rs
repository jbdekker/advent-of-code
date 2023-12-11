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

#[derive(Debug, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn set_step(&self, step: (i32, i32)) -> Point {
        Point {
            x: (self.x as i32 + step.0) as usize,
            y: (self.y as i32 + step.1) as usize,
        }
    }
}

struct PipeMap {
    grid: Vec<Vec<Pipe>>,
}

impl PipeMap {
    fn get_pipe(&mut self, point: &Point) -> &mut Pipe {
        &mut self.grid[point.y][point.x]
    }

    fn limits(&self) -> (usize, usize) {
        (self.grid[0].len(), self.grid.len())
    }

    fn validate_step(&self, point: &Point, step: (i32, i32)) -> bool {
        let limits = self.limits();

        if step.0 == 1 && point.x == limits.0 - 1 {
            return false;
        }
        if step.0 == -1 && point.x == 0 {
            return false;
        }
        if step.1 == 1 && point.y == limits.1 - 1 {
            return false;
        }
        if step.1 == -1 && point.y == 0 {
            return false;
        }
        return true;
    }
}

fn step_options(pipe: &PipeType) -> Vec<(i32, i32)> {
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

fn valid_starting_step(pipe_map: &mut PipeMap, position: &Point) -> (i32, i32) {
    let mut steps: Vec<(i32, i32)> = Vec::new();

    if pipe_map.validate_step(position, (1, 0)) {
        match pipe_map.get_pipe(&position.set_step((1, 0))).pipetype {
            PipeType::Horizontal | PipeType::NorthWest | PipeType::SouthWest => steps.push((1, 0)),
            _ => (),
        }
    }

    if pipe_map.validate_step(position, (-1, 0)) {
        match pipe_map.get_pipe(&position.set_step((-1, 0))).pipetype {
            PipeType::Horizontal | PipeType::NorthEast | PipeType::SouthEast => steps.push((-1, 0)),
            _ => (),
        }
    }

    if pipe_map.validate_step(position, (0, 1)) {
        match pipe_map.get_pipe(&position.set_step((0, 1))).pipetype {
            PipeType::Vertical | PipeType::NorthWest | PipeType::NorthEast => steps.push((0, 1)),
            _ => (),
        }
    }

    if pipe_map.validate_step(position, (0, -1)) {
        match pipe_map.get_pipe(&position.set_step((0, -1))).pipetype {
            PipeType::Horizontal | PipeType::SouthEast | PipeType::SouthWest => steps.push((0, -1)),
            _ => (),
        }
    }

    steps[0] // just take the first option...
}

fn set_step(pipe_map: &mut PipeMap, position: &Point, step_nr: usize) -> (Point, usize) {
    pipe_map.get_pipe(&position).visited = true;
    let this_pipe: &mut Pipe = &mut pipe_map.get_pipe(&position);

    dbg!(&position);
    dbg!(&this_pipe.pipetype);

    let valid_steps = match this_pipe.pipetype {
        PipeType::Ground => panic!("should never happen, standing on ground!"),
        PipeType::StartingPosition => vec![valid_starting_step(pipe_map, &position)],
        _ => step_options(&this_pipe.pipetype),
    };
    dbg!(&valid_steps);

    let mut final_step = valid_steps[0];
    for step in valid_steps.into_iter() {
        let proposed_step = position.set_step(step);
        dbg!(&proposed_step);
        // are we still on the grid?
        if !pipe_map.validate_step(&position, step) {
            dbg!("{} is off the grid!", &proposed_step);
            continue;
        }

        // have we been there before?
        dbg!(&pipe_map.get_pipe(&proposed_step).visited);
        if pipe_map.get_pipe(&proposed_step).visited {
            dbg!("{} already visited!", &proposed_step);

            if step_nr > 1
                && matches!(
                    pipe_map.get_pipe(&proposed_step).pipetype,
                    PipeType::StartingPosition
                )
            {
                return (position.set_step(step), step_nr + 1);
            } else {
                continue;
            }
        }

        // new step
        println!("Final step: {:?}", &step);
        final_step = step;
    }

    let new_position = position.set_step(final_step);

    return (new_position, step_nr + 1);
}

fn process(input: &str) -> usize {
    let mut start: Point = Point { x: 0, y: 0 };
    let grid: Vec<Vec<Pipe>> = input
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
    // panic!("");
    let mut map = PipeMap { grid: grid.clone() };

    let mut position = start.clone();
    let mut step_nr: usize = 0;
    let n_steps = loop {
        (position, step_nr) = set_step(&mut map, &position, step_nr);

        dbg!(&position);
        dbg!(&step_nr);
        dbg!(&map.get_pipe(&position).pipetype);

        match map.get_pipe(&position).pipetype {
            PipeType::StartingPosition => break step_nr,
            _ => (),
        }
    };

    n_steps / 2
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
