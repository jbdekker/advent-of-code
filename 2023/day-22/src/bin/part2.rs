use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::RangeInclusive,
};

use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Brick {
    name: char,
    x: RangeInclusive<usize>,
    y: RangeInclusive<usize>,
    z: RangeInclusive<usize>,
}

impl Brick {
    fn shadows(&self, other: &Brick) -> bool {
        self.overlaps_x(other) && self.overlaps_y(other) && self.above(other)
    }

    fn overlaps_y(&self, other: &Brick) -> bool {
        self.y.end() >= other.y.start() && self.y.start() <= other.y.end()
    }

    fn above(&self, other: &Brick) -> bool {
        self.z.start() > other.z.end()
    }

    fn overlaps_x(&self, other: &Brick) -> bool {
        self.x.end() >= other.x.start() && self.x.start() <= other.x.end()
    }

    fn is_supported_by(&self, other: &Brick) -> bool {
        self.shadows(other) && self.empty_blocks_between(other) == 0
    }

    fn empty_blocks_between(&self, other: &Brick) -> usize {
        match self.z.end() < other.z.start() {
            true => other.z.start().clone() - self.z.end().clone() - 1,
            false => self.z.start().clone() - other.z.end().clone() - 1,
        }
    }

    fn drop(&mut self, distance: usize) -> () {
        self.z = (self.z.start() - distance)..=(self.z.end() - distance);
    }
}

fn process(input: &str) -> usize {
    let mut bricks = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let parsed = line
                .trim()
                .split("~")
                .map(|x| {
                    x.split(",")
                        .map(|y| y.parse::<usize>().unwrap())
                        .collect_tuple::<(usize, usize, usize)>()
                        .unwrap()
                })
                .collect_tuple::<((usize, usize, usize), (usize, usize, usize))>()
                .unwrap();
            Brick {
                name: (i + 65) as u8 as char,
                x: parsed.0 .0..=parsed.1 .0,
                y: parsed.0 .1..=parsed.1 .1,
                z: parsed.0 .2..=parsed.1 .2,
            }
        })
        .collect::<Vec<_>>();

    let mut axis_limits = (0, 0, 0);
    for brick in bricks.iter() {
        if brick.x.clone().max().unwrap() > axis_limits.0 {
            axis_limits = (brick.x.clone().max().unwrap(), axis_limits.1, axis_limits.2);
        }

        if brick.y.clone().max().unwrap() > axis_limits.1 {
            axis_limits = (axis_limits.0, brick.y.clone().max().unwrap(), axis_limits.2);
        }

        if brick.z.clone().max().unwrap() > axis_limits.2 {
            axis_limits = (axis_limits.0, axis_limits.1, brick.z.clone().max().unwrap());
        }
    }

    loop {
        let mut something_moved = false;
        let bricks_copy = bricks.clone();
        for brick in bricks.iter_mut() {
            // get all bricks underneath the current one
            let drop: Option<usize> = bricks_copy
                .iter()
                .filter_map(|b| match brick.shadows(b) {
                    true => Some(b.empty_blocks_between(brick)),
                    false => None,
                })
                .min();

            match drop {
                Some(blocks_between) => {
                    if blocks_between > 0 {
                        brick.drop(blocks_between);
                        something_moved = true;
                    }
                }
                None => (),
            }
        }
        if !something_moved {
            break;
        }
    }

    let bricks_that_support: HashMap<&Brick, Vec<&Brick>> = bricks
        .iter()
        .map(|this| {
            (
                this,
                bricks
                    .iter()
                    .filter(|other| this.is_supported_by(other))
                    .collect(),
            )
        })
        .collect();

    let supports: HashMap<&Brick, Vec<&Brick>> = bricks
        .iter()
        .map(|this| {
            (
                this,
                bricks
                    .iter()
                    .filter(|other| other.is_supported_by(this))
                    .collect(),
            )
        })
        .collect();

    let sum_of_chains: usize = bricks
        .iter()
        .map(|brick| {
            let mut fallen_bricks: HashSet<&Brick> = HashSet::from([brick]);

            let mut queue: VecDeque<&Brick> = VecDeque::new();

            queue.push_back(brick);
            while !queue.is_empty() {
                let this_brick: &Brick = queue.pop_front().unwrap();

                let might_fall: &Vec<&Brick> = supports.get(this_brick).unwrap();

                let will_fall: Vec<&Brick> = might_fall
                    .into_iter()
                    .filter_map(|other_brick| {
                        let supports = bricks_that_support
                            .get(*other_brick)
                            .unwrap()
                            .into_iter()
                            .filter(|support| !fallen_bricks.contains(*support));

                        match supports.count() {
                            0 => Some(*other_brick),
                            _ => None,
                        }
                    })
                    .collect();

                for falling_brick in will_fall.iter() {
                    queue.push_back(*falling_brick);
                }

                fallen_bricks.extend(will_fall.into_iter());
            }

            if fallen_bricks.len() > 1 {
                // dbg!(&brick);
                // dbg!(&chain);
            }

            dbg!(&brick);
            dbg!(&fallen_bricks);
            fallen_bricks.len() - 1
        })
        .sum();

    sum_of_chains
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "1,0,1~1,2,1
        0,0,2~2,0,2
        0,2,3~2,2,3
        0,0,4~0,2,4
        2,0,5~2,2,5
        0,1,6~2,1,6
        1,1,8~1,1,9",
        );
        assert_eq!(result, 7)
    }
}

//  68326 => to low
