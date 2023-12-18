use itertools::izip;
use std::collections::{HashMap, VecDeque};

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn dig(
    plan: &mut HashMap<(isize, isize), (isize, isize)>,
    last_dug_pos: (isize, isize),
    dir: char,
    n: usize,
) -> (isize, isize) {
    let step: (isize, isize) = {
        match dir {
            'U' => (0, -1),
            'L' => (-1, 0),
            'D' => (0, 1),
            'R' => (1, 0),
            _ => panic!("Should be unreachable, got char {dir}"),
        }
    };

    let positions_to_dig: Vec<(isize, isize)> = (0..n)
        .map(|i| {
            (
                last_dug_pos.0 + step.0 * (1 + i) as isize,
                last_dug_pos.1 + step.1 * (1 + i) as isize,
            )
        })
        .collect();

    for position in positions_to_dig.iter() {
        plan.insert(*position, step);
    }

    return *positions_to_dig.iter().last().unwrap();
}

fn dig_interior(plan: &mut HashMap<(isize, isize), (isize, isize)>) -> () {
    // calc the plan size
    let x0 = plan.iter().map(|(k, _)| k.0).min().unwrap();
    let x1 = plan.iter().map(|(k, _)| k.0).max().unwrap();
    let y0 = plan.iter().map(|(k, _)| k.1).min().unwrap();
    let y1 = plan.iter().map(|(k, _)| k.1).max().unwrap();

    println!("({x0}, {x1}), ({y0}, {y1})");

    let mut inner_points: HashMap<(isize, isize), (isize, isize)> = HashMap::new();
    for y in y0..=y1 {
        for x in x0..=x1 {
            if !inner_points.contains_key(&(x, y)) && !plan.contains_key(&(x, y)) {
                let (ups, downs) = plan.iter().filter(|(k, _)| k.1 == y && k.0 > x).fold(
                    (0, 0),
                    |acc, (k, _v)| {
                        let mut res = acc;
                        if plan.contains_key(&(k.0, k.1 + 1)) {
                            res = (res.0, res.1 + 1);
                        }
                        if plan.contains_key(&(k.0, k.1 - 1)) {
                            res = (res.0 + 1, res.1);
                        }
                        res
                    },
                );
                if ups % 2 == 1 && downs % 2 == 1 {
                    inner_points.insert((x, y), (0, 0));
                    flood(&plan, &mut inner_points, (x, y));
                    print!("I");
                } else {
                    print!(".");
                }
            } else {
                print!("#");
            }
        }
        print!("\n");
    }

    plan.extend(inner_points);
}

fn flood(
    plan: &HashMap<(isize, isize), (isize, isize)>,
    inner_points: &mut HashMap<(isize, isize), (isize, isize)>,
    point: (isize, isize),
) -> () {
    let mut queue: VecDeque<(isize, isize)> = VecDeque::new();
    queue.push_back(point);

    let options = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    while !queue.is_empty() {
        inner_points.insert(point, (0, 0));

        for option in options.iter() {
            let new_point = (point.0 + option.0, point.1 + option.1);
            match plan.contains_key(&new_point) {
                false => queue.push_back(new_point),
                true => (),
            }
        }
    }
}

fn process(input: &str) -> usize {
    let (dir, num): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| {
            let c = line.split_once('#').unwrap().1.replace(')', "");

            let num: usize = c.as_str()[..5]
                .chars()
                .map(|c| c.to_digit(16).unwrap().to_string())
                .collect::<String>()
                .parse::<usize>()
                .unwrap();
            let dir: char = match c.chars().nth(5).unwrap() {
                '0' => 'R',
                '1' => 'D',
                '2' => 'L',
                '3' => 'U',
                _ => panic!("should be unreachable, got {}", c.chars().nth(5).unwrap()),
            };

            (dir, num)
        })
        .unzip();

    dbg!(&dir);
    dbg!(&num);

    let mut pos = (-1, 0);
    let mut plan: HashMap<(isize, isize), (isize, isize)> = HashMap::new();
    for (d, n) in izip!(&dir, &num) {
        pos = dig(&mut plan, pos, *d, *n);
    }

    dig_interior(&mut plan);

    // dbg!(&plan);

    dbg!(&num.iter().sum::<usize>());
    plan.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)",
        );
        assert_eq!(result, 952408144115)
    }
}

//41368 => to low
