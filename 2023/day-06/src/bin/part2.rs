use itertools::Itertools;
use std::ops::RangeInclusive;
fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> i64 {
    let (time_limits, distance_records) = input.lines().into_iter().map(|line| {
        line.trim().split(":").skip(1).map(|num| {
            num.replace(" ", "").parse::<f64>().expect("no es bueno!")
        }
        ).collect::<Vec<f64>>()
    }).collect_tuple().unwrap();

    fn abc_limits(a: f64, b: f64, c: f64) -> RangeInclusive<i64> {
        let (lower, upper) = (
            ((-b + (b*b - 4.0*a*c).sqrt()) / (2.0 * a)) as i64 + 1,
            ((-b - (b*b - 4.0*a*c).sqrt()) / (2.0 * a)).ceil() as i64 - 1,
        );
        lower..=upper
    }

    let product_of_ways = (0..time_limits.len()).into_iter().map(|i: usize| {
        abc_limits(-1., time_limits[i], -distance_records[i]).try_len().expect("no es bueno!") as i64
    }).reduce(|a, b| a * b).unwrap();
    product_of_ways
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process("Time:      7  15   30
Distance:  9  40  200");
        assert_eq!(result, 71503)
    }
}