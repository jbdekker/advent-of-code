use std::{collections::BTreeMap, ops::RangeInclusive};

use itertools::Itertools;
use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug, Clone)]
struct Part {
    x: Option<RangeInclusive<usize>>,
    m: Option<RangeInclusive<usize>>,
    a: Option<RangeInclusive<usize>>,
    s: Option<RangeInclusive<usize>>,
}

impl Part {
    fn prod(&self) -> usize {
        (self.x.clone().unwrap().end() - self.x.clone().unwrap().start() + 1)
            * (self.m.clone().unwrap().end() - self.m.clone().unwrap().start() + 1)
            * (self.a.clone().unwrap().end() - self.a.clone().unwrap().start() + 1)
            * (self.s.clone().unwrap().end() - self.s.clone().unwrap().start() + 1)
    }
}

#[derive(Debug)]
struct Rule {
    category: char,
    operator: char,
    value: usize,
    target: String,
}

impl Rule {
    fn from_str(input: &str) -> Rule {
        let re = Regex::new(r"([a-z])([<>])([0-9]+):([a-zA-Z]+)").unwrap();

        let caps = re.captures(input).unwrap();

        Rule {
            category: caps.get(1).unwrap().as_str().chars().next().unwrap(),
            operator: caps.get(2).unwrap().as_str().chars().next().unwrap(),
            value: caps.get(3).unwrap().as_str().parse().unwrap(),
            target: caps.get(4).unwrap().as_str().to_string(),
        }
    }

    fn apply(&self, part: &Part) -> ((String, Part), Part) {
        let mut part_0 = part.clone();
        let mut part_1 = part.clone();

        fn split_range(
            range: &Option<RangeInclusive<usize>>,
            value: usize,
            operator: char,
        ) -> (Option<RangeInclusive<usize>>, Option<RangeInclusive<usize>>) {
            // below, above value

            let (a, b) = match operator {
                '>' => (0, 1),
                '<' => (1, 0),
                _ => panic!(),
            };

            match range {
                Some(range) => {
                    if range.contains(&value) {
                        (
                            Some(*range.start()..=(value - a)),
                            Some((value + b)..=*range.end()),
                        )
                    } else {
                        if value > *range.start() {
                            (Some(range.clone()), None)
                        } else {
                            (None, Some(range.clone()))
                        }
                    }
                }
                None => (None, None),
            }
        }

        match self.category {
            'x' => {
                (part_0.x, part_1.x) = split_range(&part.x, self.value, self.operator);
            }
            'm' => {
                (part_0.m, part_1.m) = split_range(&part.m, self.value, self.operator);
            }
            'a' => {
                (part_0.a, part_1.a) = split_range(&part.a, self.value, self.operator);
            }
            's' => {
                (part_0.s, part_1.s) = split_range(&part.s, self.value, self.operator);
            }
            _ => panic!(),
        }

        match self.operator {
            '>' => ((self.target.clone(), part_1), part_0),
            '<' => ((self.target.clone(), part_0), part_1),
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
    default_target: String,
}

impl Workflow {
    fn from_str(input: &str) -> Workflow {
        let re = Regex::new(r"([a-z]+)\{(([a-z][<>][0-9]+:[a-zA-Z]+[,]*)+),([a-zA-Z]+)\}").unwrap();

        let caps = re.captures(input).unwrap();

        Workflow {
            name: caps.get(1).unwrap().as_str().to_string(),
            rules: caps
                .get(2)
                .unwrap()
                .as_str()
                .split(",")
                .map(|x| Rule::from_str(x))
                .collect(),
            default_target: caps.get(4).unwrap().as_str().to_string(),
        }
    }

    fn process(&self, part: &Part) -> Vec<(String, Part)> {
        let mut res: Vec<(String, Part)> = Vec::new();

        let mut part = part.clone();
        for rule in self.rules.iter() {
            let ((target, redirected), passed) = rule.apply(&part);
            part = passed;
            res.push((target, redirected));
        }

        res.push((self.default_target.clone(), part));

        res
    }
}

fn process(input: &str) -> usize {
    let (workflows, _) = input.split("\n\n").collect_tuple().unwrap();

    let workflows: Vec<Workflow> = workflows.lines().map(|l| Workflow::from_str(l)).collect();

    let workflows: BTreeMap<String, Workflow> =
        workflows.into_iter().map(|w| (w.name.clone(), w)).collect();

    let part = Part {
        x: Some(1..=4000),
        m: Some(1..=4000),
        a: Some(1..=4000),
        s: Some(1..=4000),
    };

    let mut accepted_parts: Vec<Part> = Vec::new();

    fn run(
        workflows: &BTreeMap<String, Workflow>,
        workflow_name: String,
        part: Part,
        accepted_parts: &mut Vec<Part>,
    ) -> () {
        let workflow = workflows.get(&workflow_name).unwrap();

        let res = workflow.process(&part);

        for (k, v) in res.iter() {
            match k.as_str() {
                "A" => {
                    accepted_parts.push(v.clone());
                }
                "R" => (),
                _ => {
                    run(workflows, k.to_string(), v.clone(), accepted_parts);
                }
            }
        }
    }

    run(&workflows, "in".to_string(), part, &mut accepted_parts);

    // dbg!(&accepted_parts);

    accepted_parts.iter().fold(0, |acc, x| acc + x.prod())
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    in
        px
            qkq
                A
                crn
                    A
                    R
            A
            rfg
                gd
                    R
                    R
                R
                A
        qqz
            qs
                A
                lnx
                    A
                    A
            hdj
                A
                pv
                    R
                    A
            R


    */

    #[test]
    fn it_works() {
        let result = process(
            "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}",
        );
        assert_eq!(result, 167409079868000)
    }
}

// 73619427776979 => to low
