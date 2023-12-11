use core::panic;

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
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

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Still,
}

impl Direction {
    fn to_step(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Still => (0, 0),
        }
    }
}

#[derive(Debug, Clone)]
enum Side {
    Right,
    Left,
    Unknown,
}

#[derive(Debug, Clone)]
struct Pipe {
    pipetype: PipeType,
    visited: bool,
    side: Side,
}

#[derive(Debug, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn do_move(&self, step: Direction) -> Point {
        let step = step.to_step();
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

    fn mark(&mut self, reference: &Point, directions: &Vec<Direction>, side: Side) -> () {
        let _ = directions
            .into_iter()
            .map(|direction| {
                if !self.validate_step(reference, *direction) {
                    0
                } else {
                    self.get_pipe(&reference.do_move(*direction)).side = side.clone();
                    1
                }
            })
            .collect::<Vec<i32>>();
    }

    fn limits(&self) -> (usize, usize) {
        (self.grid[0].len(), self.grid.len())
    }

    fn validate_step(&self, point: &Point, step: Direction) -> bool {
        let limits = self.limits();

        match step {
            Direction::Up if point.y == 0 => false,
            Direction::Down if point.y == limits.1 - 1 => false,
            Direction::Left if point.x == 0 => false,
            Direction::Right if point.x == limits.0 - 1 => false,
            _ => true,
        }
    }
}

fn step_options(pipe: &PipeType) -> Vec<Direction> {
    match pipe {
        PipeType::Vertical => vec![Direction::Down, Direction::Up],
        PipeType::Horizontal => vec![Direction::Left, Direction::Right],

        PipeType::NorthEast => vec![Direction::Up, Direction::Right],
        PipeType::NorthWest => vec![Direction::Up, Direction::Left],

        PipeType::SouthEast => vec![Direction::Down, Direction::Right],
        PipeType::SouthWest => vec![Direction::Down, Direction::Left],

        PipeType::StartingPosition => vec![
            Direction::Right,
            Direction::Down,
            Direction::Left,
            Direction::Up,
        ],

        PipeType::Ground => panic!("should never happen, standing on ground!"),
    }
}

fn valid_starting_step(pipe_map: &mut PipeMap, position: &Point) -> Direction {
    if pipe_map.validate_step(position, Direction::Right) {
        match pipe_map
            .get_pipe(&position.do_move(Direction::Right))
            .pipetype
        {
            PipeType::Horizontal | PipeType::NorthWest | PipeType::SouthWest => {
                return Direction::Right
            }
            _ => (),
        }
    }

    if pipe_map.validate_step(position, Direction::Left) {
        match pipe_map
            .get_pipe(&position.do_move(Direction::Left))
            .pipetype
        {
            PipeType::Horizontal | PipeType::NorthEast | PipeType::SouthEast => {
                return Direction::Left
            }
            _ => (),
        }
    }

    if pipe_map.validate_step(position, Direction::Down) {
        match pipe_map
            .get_pipe(&position.do_move(Direction::Down))
            .pipetype
        {
            PipeType::Vertical | PipeType::NorthWest | PipeType::NorthEast => {
                return Direction::Down
            }
            _ => (),
        }
    }

    if pipe_map.validate_step(position, Direction::Up) {
        match pipe_map.get_pipe(&position.do_move(Direction::Up)).pipetype {
            PipeType::Horizontal | PipeType::SouthEast | PipeType::SouthWest => {
                return Direction::Up
            }
            _ => (),
        }
    }

    panic!("should never be reached!")
}

fn get_next_step(pipe_map: &mut PipeMap, position: &Point, step_nr: usize) -> (Direction, usize) {
    pipe_map.get_pipe(&position).visited = true;
    let this_pipe: &mut Pipe = &mut pipe_map.get_pipe(&position);

    let valid_steps = match this_pipe.pipetype {
        PipeType::Ground => panic!("should never happen, standing on ground!"),
        PipeType::StartingPosition => vec![valid_starting_step(pipe_map, &position)],
        _ => step_options(&this_pipe.pipetype),
    };

    let mut final_step = valid_steps[0];
    for step in valid_steps.into_iter() {
        let proposed_step = position.do_move(step);
        // are we still on the grid?
        if !pipe_map.validate_step(&position, step) {
            continue;
        }

        // have we been there before?
        if pipe_map.get_pipe(&proposed_step).visited {
            if step_nr > 1
                && matches!(
                    pipe_map.get_pipe(&proposed_step).pipetype,
                    PipeType::StartingPosition
                )
            {
                return (step, step_nr + 1);
            } else {
                continue;
            }
        }

        final_step = step;
    }

    return (final_step, step_nr + 1);
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
                        side: Side::Unknown,
                    };
                    pipe
                })
                .collect()
        })
        .collect();

    let mut map = PipeMap { grid: grid.clone() };

    let mut position = start.clone();
    let mut step_nr: usize = 0;
    let mut new_move: Direction;
    let _ = loop {
        (new_move, step_nr) = get_next_step(&mut map, &position, step_nr);

        position = position.do_move(new_move);

        match map.get_pipe(&position).pipetype {
            PipeType::StartingPosition => break step_nr,
            PipeType::Horizontal => match new_move {
                Direction::Left => {
                    map.mark(&position, &vec![Direction::Up], Side::Right);
                    map.mark(&position, &vec![Direction::Down], Side::Left);
                }
                Direction::Right => {
                    map.mark(&position, &vec![Direction::Down], Side::Right);
                    map.mark(&position, &vec![Direction::Up], Side::Left);
                }
                _ => panic!("can't happen!"),
            },
            PipeType::Vertical => match new_move {
                Direction::Up => {
                    map.mark(&position, &vec![Direction::Right], Side::Right);
                    map.mark(&position, &vec![Direction::Left], Side::Left);
                }
                Direction::Down => {
                    map.mark(&position, &vec![Direction::Left], Side::Right);
                    map.mark(&position, &vec![Direction::Right], Side::Left);
                }
                _ => panic!("can't happen!"),
            },
            PipeType::NorthEast => match new_move {
                Direction::Left => {
                    map.mark(
                        &position,
                        &vec![Direction::Down, Direction::Left],
                        Side::Left,
                    );
                }
                Direction::Down => {
                    map.mark(
                        &position,
                        &vec![Direction::Down, Direction::Left],
                        Side::Right,
                    );
                }
                _ => panic!("can't happen!"),
            },
            PipeType::NorthWest => match new_move {
                Direction::Right => {
                    map.mark(
                        &position,
                        &vec![Direction::Right, Direction::Down],
                        Side::Right,
                    );
                }
                Direction::Down => {
                    map.mark(
                        &position,
                        &vec![Direction::Right, Direction::Down],
                        Side::Left,
                    );
                }
                _ => panic!("can't happen!"),
            },
            PipeType::SouthEast => match new_move {
                Direction::Up => {
                    map.mark(&position, &vec![Direction::Left, Direction::Up], Side::Left);
                }
                Direction::Left => {
                    map.mark(
                        &position,
                        &vec![Direction::Left, Direction::Up],
                        Side::Right,
                    );
                }
                _ => panic!("can't happen!"),
            },
            PipeType::SouthWest => match new_move {
                Direction::Right => {
                    map.mark(
                        &position,
                        &vec![Direction::Right, Direction::Up],
                        Side::Left,
                    );
                }
                Direction::Up => {
                    map.mark(
                        &position,
                        &vec![Direction::Right, Direction::Up],
                        Side::Right,
                    );
                }
                _ => panic!("can't happen!"),
            },
            _ => panic!("should not be possible"),
        }
    };

    cleanup(&mut map);
    expand(&mut map);
    let (left, right) = sum_sides(&mut map);
    let first_border_point = map.grid[0].iter().filter(|x| !x.visited).next().unwrap();

    match first_border_point.side {
        Side::Left => right,
        Side::Right => left,
        Side::Unknown => panic!("Should not happen!"),
    }
}

fn sum_sides(map: &mut PipeMap) -> (usize, usize) {
    let (mut left, mut right) = (0, 0);

    for row in map.grid.iter() {
        for pipe in row.iter() {
            match pipe.side {
                Side::Left => left += 1,
                Side::Right => right += 1,
                _ => (),
            }
        }
    }

    (left, right)
}

fn cleanup(map: &mut PipeMap) -> () {
    for row in map.grid.iter_mut() {
        for pipe in row.iter_mut() {
            if pipe.visited && !matches!(pipe.side, Side::Unknown) {
                pipe.side = Side::Unknown
            }
        }
    }
}

fn expand(map: &mut PipeMap) -> () {
    let mut unknown_sides = get_unknown_side(&map);

    let directions = vec![
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];

    while unknown_sides.len() > 0 {
        for point in unknown_sides.iter() {
            for d in directions.iter() {
                if map.validate_step(point, *d) {
                    match map.get_pipe(&point.do_move(d.clone())).side {
                        Side::Unknown => (),
                        Side::Left => map.get_pipe(point).side = Side::Left,
                        Side::Right => map.get_pipe(point).side = Side::Right,
                    }
                }
            }
        }
        unknown_sides = get_unknown_side(&map)
    }
}

fn get_unknown_side(map: &PipeMap) -> Vec<Point> {
    map.grid
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(x, pipe)| match pipe.side {
                    Side::Unknown => match pipe.visited {
                        false => Some(Point { x, y }),
                        true => None,
                    },
                    _ => None,
                })
                .collect::<Vec<Point>>()
        })
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L",
        );
        assert_eq!(result, 4)
    }
}
