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

fn process(input: &str) -> usize {
    input
        .trim()
        .split(",")
        .map(|step| hash(&step.chars().collect(), 0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
        assert_eq!(result, 1320)
    }
}
