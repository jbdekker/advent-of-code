// https://blog.adamchalmers.com/nom-chars/

use std::collections::{BTreeSet, BTreeMap};
use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug)]
struct Range {
    destination_start: i64,
    source_start: i64,
    length: i64,
}

impl Range {
    fn contains(&self, i: i64) -> bool {
        (i >= self.source_start) && (i < self.source_start + self.length)
    }

    fn map(&self, i: i64) -> i64 {
        i - self.source_start + self.destination_start
    }
}

#[derive(Debug)]
struct Map {
    ranges: Vec<Range>,
}

impl Map {
    fn contains(&self, i: i64) -> bool {
        self.ranges.iter().any(|r| r.contains(i))}

    fn map(&self, i: i64) -> Vec<i64> {
        self.ranges.iter().filter_map(|x| {
            match x.contains(i) {
                true => Some(x.map(i)),
                false => None,
            }
        }
    ).collect()
}
}

fn process(input: &str) -> i64 {

    let mut seeds: BTreeSet<i64> = BTreeSet::new();
    let mut maps: BTreeMap<&str, Map> = BTreeMap::new();
    let mut last_name: &str = "";

    let _ = input.lines().filter_map(|line| {

        let re = Regex::new(r"(seeds:)\s*(((\d+)\s*)+)").unwrap();
        if re.is_match(line) {
            let caps = re.captures(line).unwrap();
            let values = {
                let items = caps.get(2).unwrap().as_str().split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
                
                let value_iter = items.chunks(2).map(|p| {
                    let end: i64 = p[0] + p[1];
                    (p[0]..end).map(i64::from).collect::<Vec<i64>>()      
                });

                let out = value_iter.collect::<Vec<Vec<i64>>>().into_iter().flatten();
                out.clone()
            };
            seeds = BTreeSet::from_iter(values);
        }

        let re: Regex = Regex::new(r"^(\S+) map:$").unwrap(); 
        if re.is_match(line) {
            let caps = re.captures(line).unwrap();
            last_name = caps.get(1).unwrap().as_str();
            maps.insert(last_name, Map {ranges: Vec::new()});
        }

        let re: Regex = Regex::new(r"^(\d+) (\d+) (\d+)$").unwrap();
        if re.is_match(line) {
            let caps = re.captures(line).unwrap();
            let range = Range {destination_start: caps.get(1).unwrap().as_str().parse::<i64>().unwrap(), source_start: caps.get(2).unwrap().as_str().parse::<i64>().unwrap(), length: caps.get(3).unwrap().as_str().parse::<i64>().unwrap()};
            maps.entry(last_name).and_modify(|x| x.ranges.push(range));
        }
        None
    }).collect::<Vec<&str>>();

    fn get_mapping(mapping: &BTreeMap<&str, Map>, key: &str, ids: Vec<i64>) -> Vec<i64> {
        let map: &Map = mapping.get(key).unwrap();

        let mut out: Vec<i64> = Vec::new();
        for i in ids.iter() {
            if map.contains(*i) {
                out.append(&mut map.map(*i));
            }
            else {
                out.push(*i);
            }
        }
        out
    }

    let soils: Vec<i64> = get_mapping(&maps, "seed-to-soil", seeds.into_iter().collect());
    // println!("soils: {soils:?}");
    let fertilizers: Vec<i64> = get_mapping(&maps, "soil-to-fertilizer", soils);
    // println!("fertilizers: {fertilizers:?}");
    let waters: Vec<i64> = get_mapping(&maps, "fertilizer-to-water", fertilizers);
    // println!("waters: {waters:?}");
    let lights: Vec<i64> = get_mapping(&maps, "water-to-light", waters);
    // println!("lights: {lights:?}");
    let temperatures: Vec<i64> = get_mapping(&maps, "light-to-temperature", lights);
    // println!("temperatures: {temperatures:?}");
    let humidities: Vec<i64> = get_mapping(&maps, "temperature-to-humidity", temperatures);
    // println!("humidities: {humidities:?}");
    let locations: Vec<i64> = get_mapping(&maps, "humidity-to-location", humidities);
    // println!("locations: {locations:?}");
    *locations.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process("seeds: 79 14 55 13

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
56 93 4");
        assert_eq!(result, 46)
    }
}