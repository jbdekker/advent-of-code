use std::{collections::{HashMap, VecDeque}, cmp::Ordering};

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> usize {
    let mut start: (isize, isize) = (0, 0);
    let grid: Vec<Vec<char>> = input.lines().enumerate().map(|(y, line)| line.trim().chars().enumerate().map(|(x, c)| {if c == 'S' { start = (x as isize, y as isize)}
c}).collect()).collect();

    let mut seen: HashMap<(isize, isize), usize> = HashMap::new();
    let mut queue: VecDeque<((isize, isize), usize)> = VecDeque::new();

    let grid_size = (grid[0].len(), grid[0].len());
    dbg!(&grid_size);
    // let max_steps = 26501365;
    let max_steps = 65+131*8;
    queue.push_back((start, 0));
    dbg!(&start);
    while !queue.is_empty() {
        let (mut node, n_steps) = queue.pop_front().unwrap();

        if n_steps > max_steps {
            continue
        }

        if node.0 < 0 {
            node = (node.0 + grid_size.1 as isize, node.1);
        }
        if node.1 < 0 {
            node = (node.0, node.1 + grid_size.0 as isize);
        }

        // if node.0 + 1 > grid[0].len() as isize || node.1 + 1 > grid.len() as isize {
        //     continue
        // }

        // dbg!(&node);
        match grid[(node.1 % grid_size.1 as isize) as usize][(node.0 % grid_size.0 as isize) as usize] {
            '#' => (),
            '.' | 'S' => {
                if seen.contains_key(&node) {
                    match seen.get(&node).or(Some(&usize::MAX)).unwrap().cmp(&n_steps) {
                        Ordering::Greater => {seen.insert(node, n_steps);},
                        Ordering::Less | Ordering::Equal => continue,
                    }
                } else {
                    seen.insert(node, n_steps);
                }
                for n in vec![(1, 0), (-1, 0), (0, 1), (0,-1)].into_iter() {
                    queue.push_back(((node.0 + n.0, node.1 + n.1), n_steps + 1))
                }
            },
            _ => panic!("Should be unreachable!"),
        }

    }

    seen.iter().filter(|(_, v)| *v % 2 == 0).collect::<Vec<_>>().len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process("...........
        .....###.#.
        .###.##..#.
        ..#.#...#..
        ....#.#....
        .##..S####.
        .##..#...#.
        .......##..
        .##.#.####.
        .##..##.##.
        ...........");
        assert_eq!(result, 16)
    }
}

