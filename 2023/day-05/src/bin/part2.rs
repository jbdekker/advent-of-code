use regex::Regex;
use std::cmp::{max, min};
use std::collections::BTreeMap;
use std::ops::Range;

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug)]
struct MappingRule {
    destination_start: i64,
    source_start: i64,
    length: i64,
}

impl MappingRule {
    fn map(&self, i: i64) -> i64 {
        i - self.source_start + self.destination_start
    }

    fn map_range(&self, range: Range<i64>) -> (Vec<Range<i64>>, Vec<Range<i64>>) {
        let source_end: i64 = self.source_start + self.length;

        let mut matched: Vec<Range<i64>> = Vec::new();
        let mut unmatched: Vec<Range<i64>> = Vec::new();

        if range.start < self.source_start {
            unmatched.push(range.start..min(range.end, self.source_start))
        }

        if range.end > source_end {
            unmatched.push(max(range.start, source_end)..range.end)
        }

        if range.start < source_end && range.end > self.source_start {
            matched.push(
                self.map(max(range.start, self.source_start))..self.map(min(range.end, source_end)),
            )
        }

        (matched, unmatched)
    }
}

#[derive(Debug)]
struct Map {
    mapping_rules: Vec<MappingRule>,
}

impl Map {
    fn map_range(&self, ranges: Vec<Range<i64>>) -> Vec<Range<i64>> {
        let mut destination_ranges: Vec<Range<i64>> = Vec::new();

        let mut todo = ranges;
        for rule in self.mapping_rules.iter() {
            let mut unmatched_ranges: Vec<Range<i64>> = Vec::new();
            for range in todo.into_iter() {
                let (mut matched, mut unmatched) = rule.map_range(range);

                destination_ranges.append(&mut matched);
                unmatched_ranges.append(&mut unmatched);
            }
            todo = unmatched_ranges;
        }
        destination_ranges.append(&mut todo.clone());

        destination_ranges
    }
}

fn process(input: &str) -> i64 {
    let mut maps: BTreeMap<&str, Map> = BTreeMap::new();
    let mut last_name: &str = "";

    let mut seed_ranges: Vec<Range<i64>> = Vec::new();

    let _ = input
        .lines()
        .filter_map(|line| {
            let re = Regex::new(r"(seeds:)\s*(((\d+)\s*)+)").unwrap();
            if re.is_match(line) {
                let caps = re.captures(line).unwrap();

                let items = caps
                    .get(2)
                    .unwrap()
                    .as_str()
                    .split_whitespace()
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>();

                for p in items.chunks(2) {
                    seed_ranges.push(p[0]..(p[0] + p[1]));
                }
            }

            let re: Regex = Regex::new(r"^(\S+) map:$").unwrap();
            if re.is_match(line) {
                let caps = re.captures(line).unwrap();
                last_name = caps.get(1).unwrap().as_str();
                maps.insert(
                    last_name,
                    Map {
                        mapping_rules: Vec::new(),
                    },
                );
            }

            let re: Regex = Regex::new(r"^(\d+) (\d+) (\d+)$").unwrap();
            if re.is_match(line) {
                let caps = re.captures(line).unwrap();
                let range = MappingRule {
                    destination_start: caps.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                    source_start: caps.get(2).unwrap().as_str().parse::<i64>().unwrap(),
                    length: caps.get(3).unwrap().as_str().parse::<i64>().unwrap(),
                };
                maps.entry(last_name)
                    .and_modify(|x| x.mapping_rules.push(range));
            }
            None
        })
        .collect::<Vec<&str>>();

    let names = Vec::from([
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location",
    ]);

    names
        .into_iter()
        .fold(seed_ranges, |acc, name| {
            maps.get(name).unwrap().map_range(acc)
        })
        .into_iter()
        .map(|range| range.start)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
        );
        assert_eq!(result, 46)
    }
}
