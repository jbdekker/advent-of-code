fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> usize {
    let sum_of_game_numbers: usize = input
        .lines()
        .enumerate()
        .filter_map(|(game_id, line)| {
            let mut rgb = vec![0, 0, 0];

            line.splitn(2, ":")
                .nth(1)
                .unwrap()
                .split(&[',', ';'][..])
                .all(|cubes| {
                    let parts: Vec<&str> = cubes.trim().split_whitespace().collect();
                    let i = match parts[1] {
                        "red" => 0,
                        "green" => 1,
                        "blue" => 2,
                        _ => panic!(),
                    };

                    rgb[i] = rgb[i].max(parts[0].parse::<usize>().unwrap());
                    rgb[i] <= 12 + i
                })
                .then_some(game_id + 1)
        })
        .sum();

    sum_of_game_numbers
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
        assert_eq!(result, 8)
    }
}
