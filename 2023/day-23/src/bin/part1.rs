use std::collections::{HashMap, VecDeque};

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> usize {
    let maze: Vec<Vec<char>> = input.lines().map(|j| j.trim().chars().collect()).collect();

    let seen: HashMap<(isize, isize), bool> = HashMap::new();
    let mut queue: VecDeque<(HashMap<(isize, isize), bool>, (isize, isize), usize)> =
        VecDeque::new();
    queue.push_back((seen, (1, 0), 0));

    let mut res: Vec<(HashMap<(isize, isize), bool>, usize)> = Vec::new();

    // let end = (maze[0].len() as isize - 2, maze.len() as isize - 1);

    while !queue.is_empty() {
        let (mut seen, point, n_steps) = queue.pop_front().unwrap();
        // dbg!(point);

        if point.1 == maze.len() as isize - 1 {
            println!("> Reached the end of the maze in {} steps!", n_steps);
            // dbg!(&seen);
            res.push((seen, n_steps));
            continue;
        }

        if point.0 < 0 || point.1 < 0 || seen.get(&point).is_some() {
            // dbg!(point);
            continue;
        }

        seen.insert(point, true);

        match maze[point.1 as usize][point.0 as usize] {
            '>' => queue.push_front((seen, (point.0 + 1, point.1), n_steps + 1)),
            '<' => queue.push_front((seen, (point.0 - 1, point.1), n_steps + 1)),
            'v' => queue.push_front((seen, (point.0, point.1 + 1), n_steps + 1)),
            '.' => {
                for p in vec![(1, 0), (-1, 0), (0, 1), (0, -1)].iter() {
                    let new_point = (point.0 + p.0, point.1 + p.1);
                    if seen.get(&new_point).is_none() {
                        queue.push_front((seen.clone(), new_point, n_steps + 1));
                    }
                }
            }
            '#' => {
                // println!("nothing to do, go char '#' continue;");
                continue;
            }
            _ => panic!("Should be unreachable!"),
        }
        // println!("next loop!")
    }

    // dbg!(&res);

    // 0
    let (_, n_steps) = res.iter().max_by_key(|(_, n)| n).unwrap();

    *n_steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "#.#####################
            #.......#########...###
            #######.#########.#.###
            ###.....#.>.>.###.#.###
            ###v#####.#v#.###.#.###
            ###.>...#.#.#.....#...#
            ###v###.#.#.#########.#
            ###...#.#.#.......#...#
            #####.#.#.#######.#.###
            #.....#.#.#.......#...#
            #.#####.#.#.#########v#
            #.#...#...#...###...>.#
            #.#.#v#######v###.###v#
            #...#.>.#...>.>.#.###.#
            #####v#.#.###v#.#.###.#
            #.....#...#...#.#.#...#
            #.#########.###.#.#.###
            #...###...#...#...#.###
            ###.###.#.###v#####v###
            #...#...#.#.>.>.#.>.###
            #.###.###.#.###.#.#v###
            #.....###...###...#...#
            #####################.#",
        );
        assert_eq!(result, 94)
    }
}
