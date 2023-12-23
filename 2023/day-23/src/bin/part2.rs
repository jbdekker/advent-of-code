use std::collections::{HashSet, VecDeque};

use memoize::memoize;

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

    let holes_per_row: Vec<Vec<(isize, isize)>> = maze
        .iter()
        .enumerate()
        .map(|(y, r)| {
            r.iter()
                .enumerate()
                .filter_map(|(x, c)| match c {
                    '#' => None,
                    _ => Some((x as isize, y as isize)),
                })
                .collect()
        })
        .collect();

    let mut holes_per_column: Vec<Vec<(isize, isize)>> = Vec::new();
    for x in 0..maze[0].len() {
        holes_per_column.push(Vec::new());
        for y in 0..maze.len() {
            match maze[y][x] {
                '#' => (),
                _ => {
                    // if !holes_per_column.len() < x {
                    //     holes_per_column.push(Vec::new());
                    // }
                    holes_per_column[x].push((x as isize, y as isize));
                }
            }
        }
    }

    #[memoize]
    fn jump(
        point: (isize, isize),
        prev_point: (isize, isize),
        n_steps: usize,
        maze: Vec<Vec<char>>,
        visited: Vec<(isize, isize)>,
    ) -> Option<((isize, isize), usize, Vec<(isize, isize)>)> {
        let mut visited = visited;
        visited.push(point);

        if point.1 + 1 == maze.len() as isize {
            return Some((point, n_steps, visited));
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
            0 => {
                // println!("Jump {:?} -> {:?} ended on a dead-end", prev_point, point);
                None
            }
            1 => return jump(next_steps[0], point, n_steps + 1, maze, visited),
            _ => Some((point, n_steps, visited)),
        }
    }

    let mut ans = 0;
    while !queue.is_empty() {
        let (mut seen, point, n_steps) = queue.pop_front().unwrap();

        // println!("point: {:?}, n_steps: {}", point, n_steps);

        if point.1 == maze.len() as isize - 1 {
            if n_steps > ans {
                ans = n_steps;
                println!("Found best solution until now: {} steps!", n_steps);
            }
            if ans == 6158 {
                break;
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
                    // println!("Look for jump, starting at {:?}", point);
                    match jump(next_steps[0], point, 1, maze.clone(), Vec::new()) {
                        None => continue, // dead-end
                        Some((next_point, n_jumped, visited)) => {
                            if seen.contains(&next_point) {
                                continue;
                            }
                            seen.extend(visited.iter());
                            // println!("Jumped from {:?} to @ {:?}", point, next_point);
                            queue.push_front((seen.clone(), next_point, n_steps + n_jumped));
                        }
                    }
                } else {
                    // check to the right
                    let holes_on_the_right = &holes_per_column[point.0 as usize + 1];
                    if holes_on_the_right.iter().all(|h| seen.contains(&h)) {
                        println!("Continue because of holes-on-the-right check!");
                        continue;
                    }

                    // check below
                    let holes_below = &holes_per_row[point.1 as usize + 1];
                    if holes_below.iter().all(|h| seen.contains(&h)) {
                        println!("Continue because of holes-below check!");
                        continue;
                    }

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

// 6658 => too high (also someone else's answer?)
// 6258 => too low (also someone else's answer?)
// 6478 => too low (_not_ someone else's answer!)
// 6534 => (someone else's answer!)

// Found best solution until now: 5362 steps!
// Found best solution until now: 5438 steps!
// Found best solution until now: 5562 steps!
// Found best solution until now: 5566 steps!
// Found best solution until now: 5706 steps!
// Found best solution until now: 5722 steps!
// Found best solution until now: 5866 steps!
// Found best solution until now: 5918 steps!
// Found best solution until now: 6026 steps!
// Found best solution until now: 6050 steps!
// Found best solution until now: 6158 steps!
// Found best solution until now: 6258 steps!
// Found best solution until now: 6358 steps!
// Found best solution until now: 6402 steps!
// Found best solution until now: 6478 steps!
// Found best solution until now: 6534 steps!

// Benchmark, time to 6158
// no hole checks: 27.513s
// check right: 27.558s
// right & below check: 28.223s
