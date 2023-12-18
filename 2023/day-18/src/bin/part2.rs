use itertools::izip;
use std::collections::HashSet;
use std::ops::Range;

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn dig(
    plan: &mut HashSet<(isize, isize)>,
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
        plan.insert(*position);
    }

    return *positions_to_dig.iter().last().unwrap();
}

fn dig_interior(plan: &mut HashSet<(isize, isize)>) -> HashSet<(Range<isize>, isize)> {
    // calc the plan size
    let x0 = plan.iter().map(|k| k.0).min().unwrap();
    let x1 = plan.iter().map(|k| k.0).max().unwrap();
    let y0 = plan.iter().map(|k| k.1).min().unwrap();
    let y1 = plan.iter().map(|k| k.1).max().unwrap();

    println!("({x0}, {x1}), ({y0}, {y1})");

    let mut inner_points: HashSet<(Range<isize>, isize)> = HashSet::new();
    let mut x = x0;
    let mut y = y0;
    loop {
        if y > y1 {
            break;
        }
        loop {
            if x > x1 {
                break;
            }
            if !plan.contains(&(x, y)) {
                let holes = plan
                    .iter()
                    .filter(|k| k.1 == y && k.0 > x)
                    .collect::<HashSet<_>>();

                let (ups, downs) = holes.iter().fold((0, 0), |acc, k| {
                    let mut res = acc;
                    if plan.contains(&(k.0, k.1 + 1)) {
                        res = (res.0, res.1 + 1);
                    }
                    if plan.contains(&(k.0, k.1 - 1)) {
                        res = (res.0 + 1, res.1);
                    }
                    res
                });

                if ups % 2 == 1 && downs % 2 == 1 {
                    let inner_range = flood_range(&holes, (x, y));
                    inner_points.insert((inner_range.clone(), y));
                    x = inner_range.end;

                    print!("I");
                } else {
                    print!(".");
                }
            } else {
                print!("#");
            }
        }
        y += 1;
        x = 0;
        print!("\n");
    }

    inner_points
}

fn flood_range(plan: &HashSet<&(isize, isize)>, point: (isize, isize)) -> Range<isize> {
    let mut i = 1;
    loop {
        if plan.contains(&(point.0 + i, point.1)) {
            return (point.0)..(point.0 + i);
        }
        i += 1;
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
    let mut plan: HashSet<(isize, isize)> = HashSet::new();
    for (d, n) in izip!(&dir, &num) {
        pos = dig(&mut plan, pos, *d, *n);
    }

    // dig_interior(&mut plan);

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
