use std::collections::{BTreeMap, VecDeque};

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn step(
    grid: &Vec<Vec<usize>>,
    seen: &mut BTreeMap<((usize, usize), (isize, isize), usize), usize>,
    position: (usize, usize),
    last_step: (isize, isize),
    n_last_direction: usize,
    heatloss: usize,
) -> Option<Vec<((usize, usize), (isize, isize), usize, usize)>> {
    let mut heatloss = heatloss;
    if position != (0, 0) {
        heatloss += grid[position.1][position.0];
    }

    let key = (position, last_step, n_last_direction);
    if !seen.contains_key(&key) {
        seen.insert(key, heatloss);
    } else {
        if seen.get(&key).unwrap() <= &heatloss {
            return None;
        } else {
            seen.insert(key, heatloss);
        }
    }

    if position == (grid[0].len() - 1, grid.len() - 1) {
        return None;
    }

    let mut options = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    // crusible cant reverse
    options.retain(|v| v != &(-last_step.0, -last_step.1));

    // continue for >4 steps in the same direction
    if n_last_direction < 4 {
        options = vec![last_step];
    }
    if n_last_direction == 10 {
        // at most 3 steps in the same direction
        options.retain(|v| v != &last_step);
    }

    // dbg!(&options);
    let mut res: Vec<_> = Vec::new();
    for option in options.into_iter() {
        let n_last_direction = match option == last_step {
            true => n_last_direction + 1,
            false => 1,
        };

        if (option.0 == -1 && position.0 == 0)
            || (option.1 == -1 && position.1 == 0)
            || (option.1 == 1 && position.1 == grid.len() - 1)
            || (option.0 == 1 && position.0 == grid[0].len() - 1)
        {
            continue;
        }

        res.push((
            (
                (position.0 as isize + option.0) as usize,
                (position.1 as isize + option.1) as usize,
            ),
            option,
            n_last_direction,
            heatloss,
        ));
    }

    Some(res)
}

fn process(input: &str) -> usize {
    let grid: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    let mut seen: BTreeMap<((usize, usize), (isize, isize), usize), usize> = BTreeMap::new();
    let mut queue: VecDeque<((usize, usize), (isize, isize), usize, usize)> = VecDeque::new();

    queue.push_back(((0, 0), (0, 0), 0, 0));

    while !queue.is_empty() {
        let (position, last_step, n, h) = queue.pop_front().unwrap();
        let res = step(&grid, &mut seen, position, last_step, n, h);

        match res {
            Some(x) => {
                for r in x.into_iter() {
                    queue.push_back(r);
                }
            }
            None => (),
        }
    }
    // dbg!(&seen);

    // for y in 0..grid.len() {
    //     for x in 0..grid[0].len() {
    //         print!("{:4} ", seen.get(&(x, y)).unwrap());
    //     }
    //     println!();
    // }

    let mut res = Vec::new();
    for ((pos, _, _), v) in seen.into_iter() {
        if pos == (grid[0].len() - 1, grid.len() - 1) {
            res.push(v);
        }
    }

    *res.iter().min().unwrap()
    // *seen.get(&(grid[0].len() - 1, grid.len() - 1)).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533",
        );
        assert_eq!(result, 94)
    }
}

// 925 => to high
// 922
// 902
