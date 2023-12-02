fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> i32 {
    let sum_of_powers: i32 = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(":").collect();

            let grabs: Vec<Vec<&str>> = parts[1]
                .split(&[',', ';'][..])
                .map(|x| x.trim().split_whitespace().collect())
                .collect();

            let reds: i32 = grabs
                .iter()
                .filter_map(|x| {
                    if x[1] == "red" {
                        Some(x[0].parse::<i32>().unwrap())
                    } else {
                        None
                    }
                })
                .max()
                .unwrap();

            let greens: i32 = grabs
                .iter()
                .filter_map(|x| {
                    if x[1] == "green" {
                        Some(x[0].parse::<i32>().unwrap())
                    } else {
                        None
                    }
                })
                .max()
                .unwrap();

            let blues: i32 = grabs
                .iter()
                .filter_map(|x| {
                    if x[1] == "blue" {
                        Some(x[0].parse::<i32>().unwrap())
                    } else {
                        None
                    }
                })
                .max()
                .unwrap();

            let power: i32 = reds * greens * blues;

            power
        })
        .sum();

    sum_of_powers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, 2286)
    }
}
