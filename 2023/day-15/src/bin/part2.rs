use std::collections::BTreeMap;

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal_length: usize,
}

impl PartialEq for Lens {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn hash(s: &Vec<char>, value: usize) -> usize {
    match s.len() {
        0 => return value,
        _ => {
            let value = (17 * (value + (s[0] as u32) as usize)) % 256;
            return hash(&s[1..].to_vec(), value);
        }
    }
}

fn get_label(s: &Vec<char>, label: &mut Vec<char>) -> (usize, String, Vec<char>) {
    match s[0] {
        '=' | '-' => return (hash(label, 0), label.iter().collect(), s.clone()),
        _ => {
            label.push(s[0]);
            return get_label(&mut s[1..].to_vec(), label);
        }
    }
}

fn power(boxes: &BTreeMap<usize, Vec<Lens>>) -> usize {
    boxes
        .into_iter()
        .map(|(b, lenses)| {
            (1 + b)
                * lenses
                    .iter()
                    .enumerate()
                    .map(|(i, lens)| (i + 1) * lens.focal_length)
                    .sum::<usize>()
        })
        .sum()
}

fn do_step(boxes: &mut BTreeMap<usize, Vec<Lens>>, s: &mut Vec<char>) -> () {
    let (box_nr, label, remainder) = get_label(s, &mut Vec::new());

    match remainder[0] {
        '-' => boxes
            .entry(box_nr)
            .or_insert(Vec::new())
            .retain(|lens| lens.label != label),
        '=' => {
            let contents = boxes.entry(box_nr).or_insert(Vec::new());
            let lens: Lens = Lens {
                label: label.clone(),
                focal_length: remainder[1].to_digit(10).unwrap() as usize,
            };

            match contents.contains(&lens) {
                false => contents.push(lens),
                true => {
                    let contents: Vec<Lens> = contents
                        .iter()
                        .map(|l| match l {
                            _ if l == &lens => lens.clone(),
                            _ => l.clone(),
                        })
                        .collect();
                    boxes.insert(box_nr, contents);
                }
            }
        }
        _ => panic!("Should be unreachable! Got `{}`", remainder[0]),
    }
}

fn process(input: &str) -> usize {
    let mut boxes: BTreeMap<usize, Vec<Lens>> = BTreeMap::new();

    let _ = input
        .trim()
        .split(",")
        .map(|step| do_step(&mut boxes, &mut step.chars().collect()))
        .collect::<()>();

    power(&boxes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
        assert_eq!(result, 145)
    }
}
