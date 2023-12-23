use std::collections::HashMap;

use tailcall::tailcall;
// use std::cmp::max_by

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn walk(
    maze: &Vec<Vec<char>>,
    seen: HashMap<(isize, isize), bool>,
    point: (isize, isize),
    n_steps: usize,
) -> Option<(HashMap<(isize, isize), bool>, usize)> {
    if point == (maze[0].len() as isize - 1, maze.len() as isize - 2) {
        return Some((seen, n_steps));
    }

    if point.0 < 0 || point.1 < 0 || seen.get(&point).is_some() {
        return None;
    }

    match maze[point.1 as usize][point.0 as usize] {
        '#' => return None,
        '>' => return walk(maze, seen, (point.0 + 1, point.1), n_steps + 1),
        '<' => return walk(maze, seen, (point.0 - 1, point.1), n_steps + 1),
        'v' => return walk(maze, seen, (point.0, point.1 + 1), n_steps + 1),
        _ => {
            return {
                vec![
                    walk(maze, seen.clone(), (point.0 + 1, point.1), n_steps + 1),
                    walk(maze, seen.clone(), (point.0 - 1, point.1), n_steps + 1),
                    walk(maze, seen.clone(), (point.0, point.1 + 1), n_steps + 1),
                    walk(maze, seen.clone(), (point.0, point.1 - 1), n_steps + 1),
                ]
                .into_iter()
                .filter_map(|v| match v {
                    Some(x) => Some(x),
                    _ => None,
                })
                .max_by_key(|x| x.1)
            }
        }
    }
}

fn process(input: &str) -> usize {
    let maze: Vec<Vec<char>> = input.lines().map(|j| j.trim().chars().collect()).collect();

    let mut seen: HashMap<(isize, isize), bool> = HashMap::new();
    let mut n_steps = 1;
    (seen, n_steps) = walk(&maze, seen, (1, 0), n_steps).unwrap();

    dbg!(&seen);

    n_steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "#S#####################
        #OOOOOOO#########...###
        #######O#########.#.###
        ###OOOOO#OOO>.###.#.###
        ###O#####O#O#.###.#.###
        ###OOOOO#O#O#.....#...#
        ###v###O#O#O#########.#
        ###...#O#O#OOOOOOO#...#
        #####.#O#O#######O#.###
        #.....#O#O#OOOOOOO#...#
        #.#####O#O#O#########v#
        #.#...#OOO#OOO###OOOOO#
        #.#.#v#######O###O###O#
        #...#.>.#...>OOO#O###O#
        #####v#.#.###v#O#O###O#
        #.....#...#...#O#O#OOO#
        #.#########.###O#O#O###
        #...###...#...#OOO#O###
        ###.###.#.###v#####O###
        #...#...#.#.>.>.#.>O###
        #.###.###.#.###.#.#O###
        #.....###...###...#OOO#
        #####################O#",
        );
        assert_eq!(result, 94)
    }
}
