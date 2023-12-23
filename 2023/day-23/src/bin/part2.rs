use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> usize {
    let maze: Vec<Vec<char>> = input.lines().map(|j| j.trim().chars().collect()).collect();

    // let mut row_counts: Vec<usize> = maze
    //     .iter()
    //     // .enumerate()
    //     .map(|r| {
    //         r.iter()
    //             .filter(|c| vec!['.', '>', '<', 'v'].contains(c))
    //             .count()
    //     })
    //     .collect();

    // row_counts.sort();

    // println!("{:?}", &row_counts);

    let seen: HashSet<(isize, isize)> = HashSet::new();
    let mut queue: VecDeque<(HashSet<(isize, isize)>, (isize, isize), usize)> = VecDeque::new();
    queue.push_back((seen, (1, 0), 0));

    let mut res: Vec<(HashSet<(isize, isize)>, usize)> = Vec::new();
    let dirs = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    let mut jumps: HashMap<(isize, isize), ((isize, isize), usize)> = HashMap::new();

    while !queue.is_empty() {
        // let ql = queue.len();
        // if ql % 5 == 0 {
        //     println!("Queue length: {}", ql);
        // }

        let (mut seen, point, n_steps) = queue.pop_front().unwrap();

        if point.1 == maze.len() as isize - 1 {
            res.push((seen, n_steps));
            continue;
        }

        // if point.0 < 0 || point.1 < 0 || seen.contains(&point) {
        //     // dbg!(point);
        //     continue;
        // }

        seen.insert(point);

        match maze[point.1 as usize][point.0 as usize] {
            '.' | '>' | '<' | 'v' => {
                for p in dirs.iter() {
                    let new_point = (point.0 + p.0, point.1 + p.1);
                    if new_point.1 >= 0
                        && new_point.0 >= 0
                        && maze[new_point.1 as usize][new_point.0 as usize] != '#'
                        && !seen.contains(&new_point)
                    {
                        queue.push_front((seen.clone(), new_point, n_steps + 1));
                    }
                }
            }
            '#' => {
                continue;
            }
            _ => panic!("Should be unreachable!"),
        }
    }

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
