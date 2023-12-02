pub fn process_part1(input: &str) -> String {
    let result: i32 = input
        .lines()
        .map(|line| {
            let v: Vec<_> = line.chars().filter(|chr| chr.is_numeric()).collect();
            format!("{}{}", v.first().unwrap(), v.last().unwrap())
                .parse::<i32>()
                .unwrap()
        })
        .sum();
    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let result: i32 = input
        .lines()
        .map(|line| {
            let v: Vec<_> = line.chars().filter(|chr| chr.is_numeric()).collect();
            format!("{}{}", v.first().unwrap(), v.last().unwrap())
                .parse::<i32>()
                .unwrap()
        })
        .sum();
    result.to_string()
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
        assert_eq!(result, "142");
    }

    #[test]
    fn part2() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "281");
    }
}
