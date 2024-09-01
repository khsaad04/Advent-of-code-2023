pub fn process_part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let v: Vec<_> = line.chars().filter(|chr| chr.is_numeric()).collect();
            format!("{}{}", v.first().unwrap(), v.last().unwrap())
                .parse::<i32>()
                .unwrap()
        })
        .sum()
}

pub fn process_part2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let line = parse_part2(line);
            let v: Vec<_> = line.chars().filter(|chr| chr.is_numeric()).collect();
            format!("{}{}", v.first().unwrap(), v.last().unwrap())
                .parse::<i32>()
                .unwrap()
        })
        .sum()
}

fn parse_part2(input: &str) -> String {
    let possible_values: &[(&str, char); 18] = &[
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
        ("1", '1'),
        ("2", '2'),
        ("3", '3'),
        ("4", '4'),
        ("5", '5'),
        ("6", '6'),
        ("7", '7'),
        ("8", '8'),
        ("9", '9'),
    ];

    let mut numbers = String::new();

    for i in 0..input.len() {
        let text = &input[i..];

        for (ident, number) in possible_values.iter() {
            if text.starts_with(ident) {
                numbers.push(*number);
            }
        }
    }

    numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";
    const INPUT2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";

    #[test]
    fn part1() {
        let result = process_part1(INPUT);
        assert_eq!(result, 142);
    }

    #[test]
    fn part2() {
        let result = process_part2(INPUT2);
        assert_eq!(result, 281);
    }
}
