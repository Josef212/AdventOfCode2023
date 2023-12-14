use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CardInfo {
    pub card_num: u32,
    pub winning_nums: Vec<u32>,
    pub user_nums: Vec<u32>,
}

impl CardInfo {
    pub fn get_matches(&self) -> Vec<u32> {
        self.user_nums
            .iter()
            .filter(|&n| self.is_number_winner(*n))
            .map(|n| *n)
            .collect()
    }

    fn is_number_winner(&self, num: u32) -> bool {
        for n in &self.winning_nums {
            if *n == num {
                return true;
            }
        }
        false
    }
}

impl FromStr for CardInfo {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.trim().split(":").collect::<Vec<_>>();
        let card = split[0]
            .trim()
            .split(" ")
            .last()
            .unwrap()
            .trim()
            .parse::<u32>()
            .expect("Could not parse card number");
        let nums = split[1].trim().split("|").collect::<Vec<_>>();
        let winning = nums[0]
            .trim()
            .split(" ")
            .filter(|n| !n.is_empty())
            .map(|n| {
                n.trim()
                    .parse::<u32>()
                    .expect("Could not parse winning number")
            })
            .collect::<Vec<_>>();
        let user = nums[1]
            .trim()
            .split(" ")
            .filter(|n| !n.is_empty())
            .map(|n| {
                n.trim()
                    .parse::<u32>()
                    .expect("Could not parse user's number")
            })
            .collect::<Vec<_>>();

        Ok(CardInfo {
            card_num: card,
            winning_nums: winning,
            user_nums: user,
        })
    }
}

#[aoc_generator(day4)]
fn input_generator(input: &str) -> Vec<CardInfo> {
    input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| CardInfo::from_str(l).expect("Could not parse line"))
        .collect()
}

#[aoc(day4, part1)]
pub fn part1(input: &[CardInfo]) -> u32 {
    input
        .iter()
        .map(|ci| {
            let mat = ci.get_matches();
            let matches = mat.len();
            if matches > 0 {
                2u32.pow((matches as u32) - 1)
            } else {
                0
            }
        })
        .sum()
}

#[aoc(day4, part2)]
pub fn part2(input: &[CardInfo]) -> u32 {
    let mut groups = vec![0; input.len()];
    for i in 0..input.len() {
        groups[i] += 1;
        for j in 0..input[i].get_matches().len() {
            let t = i + j + 1;
            if t < groups.len() {
                groups[t] += groups[i];
            }
        }
    }

    groups.iter().map(|a| *a as u32).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_DATA: &str = r#"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#;

    const PART1_RESULT: u32 = 13;
    const PART2_RESULT: u32 = 30;

    #[test]
    fn test_parser() {
        let parsed = input_generator("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        let ret = vec![CardInfo {
            card_num: 1,
            winning_nums: vec![41, 48, 83, 86, 17],
            user_nums: vec![83, 86, 6, 31, 17, 9, 48, 53],
        }];

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
