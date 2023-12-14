use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn input_generator(input: &str) -> Vec<String> {
    input.split('\n').map(|l| l.trim().to_string()).filter(|l| !l.is_empty()).collect::<Vec<_>>()
}

#[aoc(day1, part1)]
pub fn part1(input: &[String]) -> u32 {
    input.iter().map(|line| {
        let a = line.chars().filter_map(|c| {
            match c.is_digit(10) {
                false => None,
                true => Some(c),
            }
        }).collect::<Vec<_>>();
        let first = a.first().unwrap();
        let last = a.last().unwrap();
        format!("{}{}", first, last).parse::<u32>().unwrap()
    }).sum::<u32>()
}

#[aoc(day1, part2)]
pub fn part2(_input: &[String]) -> u32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_DATA: &str = r#"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"#;

    const PART1_RESULT: u32 = 142;
    const PART2_RESULT: u32 = 0;

    #[test]
    fn test_parser() {
        let parsed = input_generator(TEST_DATA);
        let ret = vec![
            "1abc2".to_string(),
            "pqr3stu8vwx".to_string(),
            "a1b2c3d4e5f".to_string(),
            "treb7uchet".to_string(),
        ];

        assert_eq!(parsed, ret);
    }

    #[test]
    fn test_part1() {
        let parsed = input_generator(TEST_DATA);
        let result = part1(&parsed);

        assert_eq!(result, PART1_RESULT);
    }

    #[test]
    fn test_part2() {
        let parsed = input_generator(TEST_DATA);
        let result = part2(&parsed);

        assert_eq!(result, PART2_RESULT);
    }
}
