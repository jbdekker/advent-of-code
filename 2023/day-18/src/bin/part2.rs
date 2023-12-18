use itertools::izip;
use std::collections::HashSet;
use std::ops::Range;

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn dig(
    hplan: &mut HashSet<(Range<isize>, isize)>,
    vplan: &mut HashSet<(isize, Range<isize>)>,
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

    if step.0 != 0 {
        hplan.insert((
            (last_dug_pos.0 + 1)..(last_dug_pos.0 + 5 + 1),
            last_dug_pos.1,
        ));
        return (last_dug_pos.0 + 5, last_dug_pos.1);
    } else {
        vplan.insert((
            last_dug_pos.0,
            (last_dug_pos.1 + 1)..(last_dug_pos.1 + 5 + 1),
        ));
        return (last_dug_pos.0, last_dug_pos.1 + 5);
    }
}

fn is_hole(
    hplan: &HashSet<(Range<isize>, isize)>,
    vplan: &HashSet<(isize, Range<isize>)>,
    point: (isize, isize),
) -> bool {
    vplan
        .iter()
        .any(|(x, r)| point.0 == *x && r.contains(&point.1))
        || hplan
            .iter()
            .any(|(r, y)| point.1 == *y && r.contains(&point.0))
}

fn holes_on_row(
    hplan: &HashSet<(Range<isize>, isize)>,
    vplan: &HashSet<(isize, Range<isize>)>,
    y: isize,
) -> Vec<(isize, isize)> {
    let mut res: Vec<(isize, isize)> = Vec::new();

    for (r, y_hat) in hplan.iter() {
        if y_hat == &y {
            res.push((r.start, *y_hat));
            res.push((r.end - 1, *y_hat)); // range is exclusive
        }
    }
    for (x_hat, r) in vplan.iter() {
        if r.contains(&y) {
            res.push((*x_hat, y))
        }
    }

    res
}

fn dig_interior(
    hplan: &HashSet<(Range<isize>, isize)>,
    vplan: &HashSet<(isize, Range<isize>)>,
) -> HashSet<(Range<isize>, isize)> {
    // calc the plan size
    let x0 = hplan.iter().map(|k| k.0.start).min().unwrap();
    let x1 = hplan.iter().map(|k| k.0.end).max().unwrap();
    let y0 = vplan.iter().map(|k| k.1.start).min().unwrap();
    let y1 = vplan.iter().map(|k| k.1.end).max().unwrap();

    println!("({x0}, {x1}), ({y0}, {y1})");

    let mut inner_points: HashSet<(Range<isize>, isize)> = HashSet::new();
    let (mut x, mut y) = (x0, y0);
    loop {
        if y > y1 {
            break;
        }
        loop {
            if x > x1 {
                break;
            }
            if !is_hole(hplan, vplan, (x, y)) {
                let holes = holes_on_row(hplan, vplan, y);
                let (ups, downs) = holes.iter().fold((0, 0), |acc, (x_hat, y_hat)| {
                    let mut res = acc;
                    if is_hole(hplan, vplan, (*x_hat, y_hat + 1)) {
                        res = (res.0, res.1 + 1);
                    }
                    if is_hole(hplan, vplan, (*x_hat, y_hat - 1)) {
                        res = (res.0 + 1, res.1);
                    }
                    res
                });

                if ups % 2 == 1 && downs % 2 == 1 {
                    let inner_range = flood_range(&holes, (x, y));
                    inner_points.insert((inner_range.clone(), y));
                    x = inner_range.end;
                }
            }
        }
        y += 1;
        x = 0;
    }

    inner_points
}

fn flood_range(plan: &Vec<(isize, isize)>, point: (isize, isize)) -> Range<isize> {
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
    let mut hplan: HashSet<(Range<isize>, isize)> = HashSet::new();
    let mut vplan: HashSet<(isize, Range<isize>)> = HashSet::new();
    for (d, n) in izip!(&dir, &num) {
        pos = dig(&mut hplan, &mut vplan, pos, *d, *n);
    }

    let inner_holes = dig_interior(&hplan, &vplan);

    let h_holes = hplan.iter().fold(0, |acc, (r, _)| acc + r.len());
    let v_holes = vplan.iter().fold(0, |acc, (_, r)| acc + r.len());
    let inner_holes = inner_holes.iter().fold(0, |acc, (r, _)| acc + r.len());
    // dbg!(&plan);

    h_holes + v_holes + inner_holes
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
