pub fn process_part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let (id, game) = line.split_once(": ").unwrap();
            let mut val: i32 = id[5..].parse().unwrap();
            let game = game.replace(';', ",");
            let cubes: Vec<&str> = game.split(", ").collect();
            for cube in cubes {
                let (amount, color) = cube.split_once(' ').unwrap();
                let amount: i32 = amount.parse().unwrap();
                match color {
                    "red" => {
                        if amount > 12 {
                            val = 0
                        }
                    }
                    "green" => {
                        if amount > 13 {
                            val = 0
                        }
                    }
                    "blue" => {
                        if amount > 13 {
                            val = 0
                        }
                    }
                    _ => panic!("invalid input"),
                }
            }
            val
        })
        .sum()
}

pub fn process_part2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            let (_, sets) = line.split_once(": ").unwrap();
            for set in sets.split("; ") {
                for pair in set.split(", ") {
                    let (num, color) = pair.split_once(' ').unwrap();
                    let num: i32 = num.parse().unwrap();
                    match color {
                        "red" => red = red.max(num),
                        "green" => green = green.max(num),
                        "blue" => blue = blue.max(num),
                        _ => panic!("invalid input"),
                    }
                }
            }
            red * green * blue
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
    #[test]
    fn part1() {
        let result = process_part1(INPUT);
        assert_eq!(result, 8);
    }

    #[test]
    fn part2() {
        let result = process_part2(INPUT);
        assert_eq!(result, 2286);
    }
}
