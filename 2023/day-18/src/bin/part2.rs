use itertools::izip;
use std::collections::HashSet;
use std::i64;
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
    n: isize,
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
        if step.0 == 1 {
            hplan.insert((
                (last_dug_pos.0 + step.0)..(last_dug_pos.0 + (n + 1) * step.0),
                last_dug_pos.1,
            ));
        } else {
            hplan.insert(((last_dug_pos.0 - n)..last_dug_pos.0, last_dug_pos.1));
        }
        return (last_dug_pos.0 + n * step.0, last_dug_pos.1);
    } else {
        if step.1 == 1 {
            vplan.insert((
                last_dug_pos.0,
                (last_dug_pos.1 + step.1)..(last_dug_pos.1 + (n + 1) * step.1),
            ));
        } else {
            vplan.insert((last_dug_pos.0, (last_dug_pos.1 - n)..last_dug_pos.1));
        }
        return (last_dug_pos.0, last_dug_pos.1 + n * step.1);
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
            // println!("hplan range: y: {}, range: {:?}", y_hat, r);
            res.push((r.start, y));
            res.push((r.end - 1, y)); // range is exclusive
        }
    }
    for (x_hat, r) in vplan.iter() {
        if r.contains(&y) {
            res.push((*x_hat, y));
            // println!("vplan range: x: {}, range: {:?}", x_hat, r);
        }
    }

    res
}

fn dig_interior(
    hplan: &HashSet<(Range<isize>, isize)>,
    vplan: &HashSet<(isize, Range<isize>)>,
) -> usize {
    // calc the plan size
    let x0 = hplan.iter().map(|k| k.0.start).min().unwrap();
    let x1 = hplan.iter().map(|k| k.0.end).max().unwrap();
    let y0 = vplan.iter().map(|k| k.1.start).min().unwrap();
    let y1 = vplan.iter().map(|k| k.1.end).max().unwrap();

    println!("ranges: ({x0}, {x1}), ({y0}, {y1})");
    // panic!();
    let mut inner_points: usize = 0;
    let (mut x, mut y) = (x0, y0);
    loop {
        if y > y1 {
            break;
        }
        // dbg!(&y);
        let mut holes = holes_on_row(hplan, vplan, y);

        // if y == 5 {
        //     dbg!(hplan);
        //     dbg!(vplan);
        //     dbg!(holes);
        //     panic!();
        // }
        holes.sort_by(|a, b| a.0.cmp(&b.0));
        // dbg!(&holes);

        if holes.len() > 0 {
            'xloop: loop {
                if x >= x1 {
                    break 'xloop;
                }
                // dbg!(&x);
                if !is_hole(hplan, vplan, (x, y)) {
                    let (ups, downs) = holes.iter().filter(|(x_hat, _)| x_hat > &x).fold(
                        (0, 0),
                        |acc, (x_hat, y_hat)| {
                            let mut res = acc;
                            if is_hole(hplan, vplan, (*x_hat, y_hat + 1)) {
                                res = (res.0, res.1 + 1);
                            }
                            if is_hole(hplan, vplan, (*x_hat, y_hat - 1)) {
                                res = (res.0 + 1, res.1);
                            }
                            res
                        },
                    );

                    // if y == 5 {
                    //     dbg!(x);
                    //     dbg!(&holes);
                    // }

                    match holes.iter().filter(|(x_hat, _)| x_hat > &x).nth(0) {
                        Some(next_hole) => {
                            if ups % 2 == 1 && downs % 2 == 1 {
                                // dbg!(&y);
                                let inner_range = (x..(next_hole.0));

                                inner_points += (inner_range.len());
                            }
                            x = next_hole.0;
                        }
                        None => break 'xloop,
                    }
                }
                x += 1;
            }
        }
        y += 1;
        x = 0;
        if y % 10000 == 0 {
            // println!("y: {y:20}    {inner_points}");
        }
    }

    inner_points
}

fn process(input: &str) -> usize {
    let (dir, num): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| {
            let c = line.split_once('#').unwrap().1.replace(')', "");

            let num: usize = i64::from_str_radix(&c.as_str()[..5], 16).unwrap() as usize;
            // .chars()
            // .map(|c| c.to_digit(16).unwrap().to_string())
            // .collect::<String>()
            // .parse::<usize>()
            // .unwrap();
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

    // dbg!(&dir);
    // dbg!(&num);

    let mut pos = (-1, 0);
    let mut hplan: HashSet<(Range<isize>, isize)> = HashSet::new();
    let mut vplan: HashSet<(isize, Range<isize>)> = HashSet::new();

    // println!();
    for (d, n) in izip!(&dir, &num) {
        pos = dig(&mut hplan, &mut vplan, pos, *d, *n as isize);
        println!("{:20}  =>  {}  =>  {:10?}", n, d, pos);
    }
    // println!();

    // dbg!(&vplan);
    // dbg!(&hplan);

    let inner_holes = dig_interior(&hplan, &vplan);
    // let inner_holes = 0;
    let h_holes = hplan.iter().fold(0, |acc, (r, _)| acc + r.len());
    let v_holes = vplan.iter().fold(0, |acc, (_, r)| acc + r.len());
    // dbg!(&plan);

    dbg!(&h_holes + &v_holes);

    h_holes + v_holes + inner_holes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dig() {
        let mut hplan: HashSet<(Range<isize>, isize)> = HashSet::new();
        let mut vplan: HashSet<(isize, Range<isize>)> = HashSet::new();

        let result = dig(&mut hplan, &mut vplan, (-1, 0), 'R', 3);
        dbg!(&result);
        let result = dig(&mut hplan, &mut vplan, result, 'D', 5);
        dbg!(&result);
        let result = dig(&mut hplan, &mut vplan, result, 'L', 3);
        dbg!(&result);
        let result = dig(&mut hplan, &mut vplan, result, 'U', 5);

        assert_eq!(result, (-1, 0));

        dbg!(&hplan);
        dbg!(&vplan);

        assert_eq!(
            hplan.iter().fold(0, |acc, x| acc + x.0.len())
                + vplan.iter().fold(0, |acc, x| acc + x.1.len()),
            16
        );
    }

    #[test]
    fn test_is_hole() {
        let mut hplan: HashSet<(Range<isize>, isize)> = HashSet::new();
        let mut vplan: HashSet<(isize, Range<isize>)> = HashSet::new();

        hplan.insert((0..3, 0));
        vplan.insert((2, 1..3));

        let result = is_hole(&hplan, &vplan, (1, 0));
        assert_eq!(result, true);

        let result = is_hole(&hplan, &vplan, (2, 2));
        assert_eq!(result, true);
    }

    #[test]
    fn test_holes_on_row() {
        let mut hplan: HashSet<(Range<isize>, isize)> = HashSet::new();
        let mut vplan: HashSet<(isize, Range<isize>)> = HashSet::new();

        hplan.insert((1..3, 2));
        vplan.insert((0, 1..3));

        let result = holes_on_row(&hplan, &vplan, 0);
        assert_eq!(result.len(), 0);

        let mut result = holes_on_row(&hplan, &vplan, 2);
        assert_eq!(result.len(), 3);

        result.sort_by(|a, b| a.0.cmp(&b.0));
        assert_eq!(result, vec![(0, 2), (1, 2), (2, 2)]);
    }

    #[test]
    fn test_full() {
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

    #[test]
    fn it_works() {
        let result = process(
            "R 6 (#000040)
        D 5 (#000041)
        R 6 (#000040)
        U 6 (#000043)
        R 6 (#000040)
        D 2 (#000091)
        L 6 (#000012)
        U 6 (#000043)
        L 6 (#000032)
        D 6 (#000031)
        R 6 (#000020)
        D 6 (#000011)
        L 2 (#0000a2)
        U 6 (#000093)",
        );
        assert_eq!(result, 114)
    }
}

//41368 => to low
//82156731921602 => to low
