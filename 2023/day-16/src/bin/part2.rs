use memoize::memoize;
use std::collections::VecDeque;

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Kind {
    Empty,
    LRMirror,
    RLMirror,
    HSplitter,
    VSplitter,
}

impl Kind {
    fn from_char(c: char) -> Kind {
        match c {
            '.' => Kind::Empty,
            '|' => Kind::VSplitter,
            '-' => Kind::HSplitter,
            '/' => Kind::LRMirror,
            '\\' => Kind::RLMirror,
            _ => panic!("Should be unreachable, got char {c}"),
        }
    }
}

#[derive(Eq, PartialEq, Hash)]
struct Grid {
    squares: Vec<Vec<Square>>,
}

#[derive(Eq, PartialEq, Hash)]
struct Square {
    kind: Kind,
    energized: bool,
}

#[derive(PartialEq, Eq, Copy, Clone, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Grid {
    fn num_energized(&self) -> usize {
        self.squares
            .iter()
            .map(|r| r.iter().filter(|x| x.energized).count())
            .sum()
    }

    fn reset(&mut self) -> () {
        for row in self.squares.iter_mut() {
            for s in row.iter_mut() {
                s.energized = false;
            }
        }
    }

    fn get(&mut self, point: &Point) -> &mut Square {
        &mut self.squares[point.y][point.x]
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    x: usize,
    y: usize,
}

fn maybe_step(span: (usize, usize), direction: &Direction, point: &Point) -> Option<Point> {
    match direction {
        Direction::Left => {
            if point.x == 0 {
                None
            } else {
                Some(Point {
                    x: point.x - 1,
                    y: point.y,
                })
            }
        }
        Direction::Right => {
            if point.x == span.0 {
                None
            } else {
                Some(Point {
                    x: point.x + 1,
                    y: point.y,
                })
            }
        }
        Direction::Up => {
            if point.y == 0 {
                None
            } else {
                Some(Point {
                    x: point.x,
                    y: point.y - 1,
                })
            }
        }
        Direction::Down => {
            if point.y == span.1 {
                None
            } else {
                Some(Point {
                    x: point.x,
                    y: point.y + 1,
                })
            }
        }
    }
}

#[memoize]
fn do_step(
    kind: Kind,
    span: (usize, usize),
    point: Point,
    direction: Direction,
) -> Vec<(Option<Point>, Direction)> {
    let mut res = Vec::new();

    match kind {
        Kind::Empty => res.push((maybe_step(span, &direction, &point), direction)),
        Kind::HSplitter => match direction {
            Direction::Left | Direction::Right => {
                res.push((maybe_step(span, &direction, &point), direction))
            }
            Direction::Up | Direction::Down => {
                res.push((maybe_step(span, &Direction::Left, &point), Direction::Left));
                res.push((
                    maybe_step(span, &Direction::Right, &point),
                    Direction::Right,
                ));
            }
        },
        Kind::VSplitter => match direction {
            Direction::Up | Direction::Down => {
                res.push((maybe_step(span, &direction, &point), direction))
            }
            Direction::Left | Direction::Right => {
                res.push((maybe_step(span, &Direction::Up, &point), Direction::Up));
                res.push((maybe_step(span, &Direction::Down, &point), Direction::Down));
            }
        },
        Kind::LRMirror => match direction {
            Direction::Right => res.push((maybe_step(span, &Direction::Up, &point), Direction::Up)),
            Direction::Left => {
                res.push((maybe_step(span, &Direction::Down, &point), Direction::Down))
            }
            Direction::Down => {
                res.push((maybe_step(span, &Direction::Left, &point), Direction::Left))
            }
            Direction::Up => res.push((
                maybe_step(span, &Direction::Right, &point),
                Direction::Right,
            )),
        },
        Kind::RLMirror => match direction {
            Direction::Left => res.push((maybe_step(span, &Direction::Up, &point), Direction::Up)),
            Direction::Right => {
                res.push((maybe_step(span, &Direction::Down, &point), Direction::Down))
            }
            Direction::Up => {
                res.push((maybe_step(span, &Direction::Left, &point), Direction::Left))
            }
            Direction::Down => res.push((
                maybe_step(span, &Direction::Right, &point),
                Direction::Right,
            )),
        },
    }
    res
}

fn run(grid: &mut Grid, starting_point: Point, starting_direction: Direction) -> usize {
    grid.reset();

    let span = (grid.squares[0].len() - 1, grid.squares.len() - 1);

    let mut queue: VecDeque<(Option<Point>, Direction)> = VecDeque::new();
    let mut seen: Vec<(Point, Direction)> = Vec::new();

    queue.push_back((Some(starting_point), starting_direction));
    while !queue.is_empty() {
        let (point, direction) = queue.pop_front().unwrap();

        match point {
            Some(p) => {
                if !seen.contains(&(p, direction)) {
                    grid.get(&p).energized = true;
                    for opt in do_step(grid.get(&p).kind, span, p, direction).iter() {
                        queue.push_back(opt.clone());
                    }
                    seen.push((p, direction))
                }
            }
            None => (),
        }
    }

    grid.num_energized()
}

fn process(input: &str) -> usize {
    let squares: Vec<Vec<Square>> = input
        .lines()
        .map(|row| {
            row.chars()
                .map(|c| Square {
                    kind: Kind::from_char(c),
                    energized: false,
                })
                .collect()
        })
        .collect();

    let mut grid = Grid { squares };
    let (max_x, max_y) = (grid.squares[0].len() - 1, grid.squares.len() - 1);
    let mut res = Vec::new();
    for x in 0..grid.squares[0].len() {
        res.push(run(&mut grid, Point { x, y: 0 }, Direction::Down));
        res.push(run(&mut grid, Point { x, y: max_y }, Direction::Up));
    }
    for y in 0..grid.squares.len() {
        res.push(run(&mut grid, Point { x: 0, y }, Direction::Right));
        res.push(run(&mut grid, Point { x: max_x, y }, Direction::Left));
    }

    *res.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....",
        );
        assert_eq!(result, 51)
    }
}

// Memoize: 7.599 sec
// No-memoize: 7.348 sec
