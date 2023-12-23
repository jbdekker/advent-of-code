use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> usize {
    let maze: Vec<Vec<char>> = input.lines().map(|j| j.trim().chars().collect()).collect();

    let seen: HashSet<(isize, isize)> = HashSet::new();
    let mut queue: VecDeque<(HashSet<(isize, isize)>, (isize, isize), usize)> = VecDeque::new();
    queue.push_back((seen, (1, 0), 0));

    let dirs = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut cache: HashMap<((isize, isize), (isize, isize)), Option<((isize, isize), usize)>> =
        HashMap::new();

    fn jump(
        point: (isize, isize),
        prev_point: (isize, isize),
        n_steps: usize,
        maze: &Vec<Vec<char>>,
    ) -> Option<((isize, isize), usize)> {
        if point.1 + 1 == maze.len() as isize {
            return Some((point, n_steps));
        }

        let next_steps: Vec<_> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)]
            .iter()
            .filter_map(|p| {
                let new_point = (point.0 + p.0, point.1 + p.1);
                match new_point.1 >= 0
                    && new_point.0 >= 0
                    && maze[new_point.1 as usize][new_point.0 as usize] != '#'
                    && new_point != prev_point
                {
                    true => Some(new_point),
                    false => None,
                }
            })
            .collect();

        match next_steps.len() {
            0 => None,
            1 => return jump(next_steps[0], point, n_steps + 1, maze),
            _ => Some((point, n_steps)),
        }
    }

    let mut ans = 0;
    while !queue.is_empty() {
        let (mut seen, point, n_steps) = queue.pop_front().unwrap();

        if point.1 == maze.len() as isize - 1 {
            if n_steps > ans {
                ans = n_steps;
                println!("Found best solution until now: {} steps!", n_steps);
            }
            continue;
        }

        seen.insert(point);

        match maze[point.1 as usize][point.0 as usize] {
            '.' | '>' | '<' | 'v' => {
                let next_steps: Vec<_> = dirs
                    .iter()
                    .filter_map(|p| {
                        let new_point = (point.0 + p.0, point.1 + p.1);
                        match new_point.1 >= 0
                            && new_point.0 >= 0
                            && maze[new_point.1 as usize][new_point.0 as usize] != '#'
                            && !seen.contains(&new_point)
                        {
                            true => Some(new_point),
                            false => None,
                        }
                    })
                    .collect();

                if next_steps.len() == 1 {
                    let jmp_res = match cache.contains_key(&(next_steps[0], point)) {
                        true => cache.get(&(next_steps[0], point)).unwrap(),
                        false => {
                            let jmp_res = jump(next_steps[0], point, 1, &maze);
                            cache.insert((next_steps[0], point), jmp_res);
                            cache.get(&(next_steps[0], point)).unwrap()
                        }
                    };

                    match jmp_res {
                        None => continue, // dead-end
                        Some((next_point, n_jumped)) => {
                            if seen.contains(&next_point) {
                                continue;
                            }
                            queue.push_front((seen.clone(), *next_point, n_steps + n_jumped));
                        }
                    }
                } else {
                    for next_point in next_steps.into_iter() {
                        queue.push_front((seen.clone(), next_point, n_steps + 1));
                    }
                }
            }
            '#' => {
                continue;
            }
            _ => panic!("Should be unreachable!"),
        }
    }

    ans
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
        assert_eq!(result, 154)
    }
}
