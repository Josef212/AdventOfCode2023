use std::{collections::HashMap, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq, Eq)]
pub struct GameInfo {
    pub id: u32,
    pub sets: Vec<CubeSets>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct CubeSets(Vec<(u32, String)>);

impl GameInfo {
    pub fn get_power(&self) -> u32 {
        let mut min_needed = HashMap::new();
        for s in &self.sets {
            let flat = s.flatten();
            for s in &flat {
                let color = *s.0;
                min_needed.entry(color).or_insert(0 as u32);
                min_needed.insert(color, std::cmp::max(*s.1, min_needed[color]));
            }

        }

        let mut power = 1;
        for s in &min_needed {
            if *s.1 > 0 {
                power *= *s.1;
            }
        }

        power
    }
}

impl CubeSets {
    pub fn flatten(&self) -> HashMap<&str, u32> {
        let mut blocks = HashMap::new();
        for s in &self.0 {
            let color = s.1.as_str();
            blocks.entry(color).or_insert(0);
            (*blocks.get_mut(color).unwrap()) += s.0;
        }

        blocks
    }
}

impl FromStr for GameInfo {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(':').collect::<Vec<_>>();
        let game_id = split[0].split(" ").collect::<Vec<_>>()[1]
            .trim()
            .parse::<u32>()
            .unwrap();
        let data = split[1]
            .split(";")
            .map(|s| {
                CubeSets(
                    s.split(",")
                        .map(|ss| {
                            let sp = ss.trim().split(" ").collect::<Vec<_>>();
                            let n = sp[0].trim().parse::<u32>().unwrap();
                            let c = sp[1].trim().to_string();
                            (n, c)
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>();

        Ok(GameInfo {
            id: game_id,
            sets: data,
        })
    }
}

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<GameInfo> {
    input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| GameInfo::from_str(l).unwrap())
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[GameInfo]) -> u32 {
    let mut id_sum = 0;
    let mut blocks = HashMap::new();
    let max_blocks = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    for game_info in input {
        blocks.clear();

        let mut check = true;
        for set in &game_info.sets {
            blocks.clear();

            for s in &set.0 {
                let color = (&s.1).as_str();
                blocks.entry(color).or_insert(0);
                (*blocks.get_mut(color).unwrap()) += s.0;
            }

            for mb in &max_blocks {
                let max = *mb.1;
                let color = *mb.0;
                if let Some(set_count) = blocks.get(color) {
                    if *set_count > max {
                        check = false;
                        break;
                    }
                }
            }

            if !check {
                break;
            }
        }

        if check {
            id_sum += game_info.id;
        }
    }

    id_sum
}

#[aoc(day2, part2)]
pub fn part2(input: &[GameInfo]) -> u32 {
    input.iter().map(|gi| gi.get_power()).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_DATA: &str = r#"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#;

    const PART1_RESULT: u32 = 8;
    const PART2_RESULT: u32 = 2286;

    #[test]
    fn test_parser() {
        const TEST_PARSE_DATA: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let parsed = input_generator(TEST_PARSE_DATA);
        let ret = vec![GameInfo {
            id: 1,
            sets: vec![
                CubeSets(vec![(3, "blue".to_string()), (4, "red".to_string())]),
                CubeSets(vec![
                    (1, "red".to_string()),
                    (2, "green".to_string()),
                    (6, "blue".to_string()),
                ]),
                CubeSets(vec![(2, "green".to_string())]),
            ],
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
