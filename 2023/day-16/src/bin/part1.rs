use std::collections::VecDeque;

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

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

struct Grid {
    squares: Vec<Vec<Square>>,
}

struct Square {
    kind: Kind,
    energized: bool,
}

#[derive(PartialEq, Copy, Clone)]
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

    fn get(&mut self, point: &Point) -> &mut Square {
        &mut self.squares[point.y][point.x]
    }

    fn step(&self, direction: &Direction, point: &Point) -> Option<Point> {
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
                if point.x == self.squares[0].len() - 1 {
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
                if point.y == self.squares.len() - 1 {
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
}

#[derive(PartialEq, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

fn run(
    queue: &mut VecDeque<(Option<Point>, Direction)>,
    grid: &mut Grid,
    point: &Point,
    direction: Direction,
) -> () {
    grid.get(&point).energized = true;

    match grid.get(&point).kind {
        Kind::Empty => queue.push_back((grid.step(&direction, &point), direction)),
        Kind::HSplitter => match direction {
            Direction::Left | Direction::Right => {
                queue.push_back((grid.step(&direction, &point), direction))
            }
            Direction::Up | Direction::Down => {
                queue.push_back((grid.step(&Direction::Left, &point), Direction::Left));
                queue.push_back((grid.step(&Direction::Right, &point), Direction::Right));
            }
        },
        Kind::VSplitter => match direction {
            Direction::Up | Direction::Down => {
                queue.push_back((grid.step(&direction, &point), direction))
            }
            Direction::Left | Direction::Right => {
                queue.push_back((grid.step(&Direction::Up, &point), Direction::Up));
                queue.push_back((grid.step(&Direction::Down, &point), Direction::Down));
            }
        },
        Kind::LRMirror => match direction {
            Direction::Right => queue.push_back((grid.step(&Direction::Up, &point), Direction::Up)),
            Direction::Left => {
                queue.push_back((grid.step(&Direction::Down, &point), Direction::Down))
            }
            Direction::Down => {
                queue.push_back((grid.step(&Direction::Left, &point), Direction::Left))
            }
            Direction::Up => {
                queue.push_back((grid.step(&Direction::Right, &point), Direction::Right))
            }
        },
        Kind::RLMirror => match direction {
            Direction::Left => queue.push_back((grid.step(&Direction::Up, &point), Direction::Up)),
            Direction::Right => {
                queue.push_back((grid.step(&Direction::Down, &point), Direction::Down))
            }
            Direction::Up => {
                queue.push_back((grid.step(&Direction::Left, &point), Direction::Left))
            }
            Direction::Down => {
                queue.push_back((grid.step(&Direction::Right, &point), Direction::Right))
            }
        },
    }
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
    let mut queue: VecDeque<(Option<Point>, Direction)> = VecDeque::new();
    let mut seen: Vec<(Point, Direction)> = Vec::new();

    queue.push_back((Some(Point { x: 0, y: 0 }), Direction::Right));
    while !queue.is_empty() {
        let (point, direction) = queue.pop_front().unwrap();

        match point {
            Some(p) => {
                if !seen.contains(&(p, direction)) {
                    run(&mut queue, &mut grid, &p, direction);
                    seen.push((p, direction))
                }
            }
            None => (),
        }
    }

    grid.num_energized()
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
        assert_eq!(result, 46)
    }
}
