use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}
impl Part {
    fn from_str(input: &str) -> Part {
        let re = Regex::new(r"x=([0-9]+),m=([0-9]+),a=([0-9]+),s=([0-9]+)").unwrap();

        let caps = re.captures(input).unwrap();

        Part {
            x: caps.get(1).unwrap().as_str().parse().unwrap(),
            m: caps.get(2).unwrap().as_str().parse().unwrap(),
            a: caps.get(3).unwrap().as_str().parse().unwrap(),
            s: caps.get(4).unwrap().as_str().parse().unwrap(),
        }
    }
    fn rating(&self) -> usize {
        self.x + self.m + self.a + self.s
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

    fn apply(&self, part: &Part) -> Option<String> {
        let rel_value = match self.category {
            'x' => part.x,
            'm' => part.m,
            'a' => part.a,
            's' => part.s,
            _ => panic!("Should be unreachable!"),
        };

        let res = match self.operator {
            '>' => rel_value > self.value,
            '<' => rel_value < self.value,
            _ => panic!("Should be unreachable!"),
        };

        match res {
            true => Some(self.target.clone()),
            false => None,
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

    fn process(&self, part: &Part) -> String {
        let res: Vec<String> = self
            .rules
            .iter()
            .filter_map(|rule| rule.apply(&part))
            .collect();

        if res.len() == 0 {
            return self.default_target.clone();
        } else {
            return res[0].clone();
        }
    }
}

fn process(input: &str) -> usize {
    let (workflows, parts) = input.split("\n\n").collect_tuple().unwrap();

    let parts: Vec<Part> = parts.lines().map(|p| Part::from_str(p)).collect();
    let workflows: Vec<Workflow> = workflows.lines().map(|l| Workflow::from_str(l)).collect();

    let workflows: HashMap<String, Workflow> =
        workflows.into_iter().map(|w| (w.name.clone(), w)).collect();

    let rating = parts
        .into_iter()
        .filter_map(|part| {
            let mut next_workflow = "in".to_string();
            loop {
                let workflow = workflows.get(&next_workflow).unwrap();

                next_workflow = workflow.process(&part);

                match next_workflow.as_str() {
                    "R" => break None,
                    "A" => break Some(part.rating()),
                    _ => continue,
                }
            }
        })
        .sum::<usize>();

    rating
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(result, 19114)
    }
}
