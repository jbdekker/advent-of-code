use itertools::izip;
use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn dig(
    plan: &mut HashMap<(isize, isize), ((isize, isize), String)>,
    last_dug_pos: (isize, isize),
    dir: char,
    color: &String,
    n: usize,
) -> (isize, isize) {
    if n == 0 {
        return last_dug_pos;
    }

    let step: (isize, isize) = {
        match dir {
            'U' => (0, -1),
            'L' => (-1, 0),
            'D' => (0, 1),
            'R' => (1, 0),
            _ => panic!("Should be unreachable, got char {dir}"),
        }
    };

    let dig_at_pos = (last_dug_pos.0 + step.0, last_dug_pos.1 + step.1);

    // if plan.contains_key(&dig_at_pos) {
    //     panic!("already dug @ {:?}", dig_at_pos);
    // }

    plan.insert(dig_at_pos, (step, color.clone()));

    return dig(plan, dig_at_pos, dir, color, n - 1);
}

fn dig_interior(plan: &mut HashMap<(isize, isize), ((isize, isize), String)>) -> () {
    // calc the plan size
    let x0 = plan.iter().map(|(k, _)| k.0).min().unwrap();
    let x1 = plan.iter().map(|(k, _)| k.0).max().unwrap();
    let y0 = plan.iter().map(|(k, _)| k.1).min().unwrap();
    let y1 = plan.iter().map(|(k, _)| k.1).max().unwrap();

    println!("({x0}, {x1}), ({y0}, {y1})");

    let mut inner_points: HashMap<(isize, isize), ((isize, isize), String)> = HashMap::new();
    for y in y0..=y1 {
        for x in x0..=x1 {
            if !plan.contains_key(&(x, y)) {
                let (ups, downs) = plan.iter().fold((0, 0), |acc, (k, _v)| {
                    let mut res = acc;
                    if k.1 == y && k.0 > x {
                        if plan.contains_key(&(k.0, k.1 + 1)) {
                            res = (res.0, res.1 + 1);
                        }
                        if plan.contains_key(&(k.0, k.1 - 1)) {
                            res = (res.0 + 1, res.1);
                        }
                    }
                    res
                });
                if ups % 2 == 1 && downs % 2 == 1 {
                    inner_points.insert((x, y), ((0, 0), "".to_string()));
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

fn process(input: &str) -> usize {
    let (dir, num, color): (Vec<_>, Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| {
            let (a, b, c) = line
                .split_whitespace()
                .collect_tuple::<(&str, &str, &str)>()
                .unwrap();

            (
                a.trim().chars().next().unwrap(),
                b.trim().parse::<usize>().unwrap(),
                c.trim().replace('(', "").replace(')', ""),
            )
        })
        .multiunzip();

    // dbg!(&dir);
    // dbg!(&num);
    // dbg!(&color);

    let mut pos = (-1, 0);
    let mut plan: HashMap<(isize, isize), ((isize, isize), String)> = HashMap::new();
    for (d, c, n) in izip!(&dir, &color, &num) {
        pos = dig(&mut plan, pos, *d, &c, *n);
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
        assert_eq!(result, 62)
    }
}
