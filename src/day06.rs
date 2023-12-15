use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

type ChallangeData = Data;
type OutputData = u32;

#[aoc_generator(day6)]
fn input_generator(input: &str) -> ChallangeData {
    ChallangeData::from_str(input).expect("Could not parse input")
}

#[aoc(day6, part1)]
pub fn part1(input: &ChallangeData) -> OutputData {
    input
        .0
        .iter()
        .map(|rd| rd.get_hold_time_win_possibilities().len() as u32)
        .product()
}

#[aoc(day6, part2)]
pub fn part2(input: &ChallangeData) -> OutputData {
    input.merge_samples().get_hold_time_win_possibilities().len() as u32
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_DATA: &str = r#"
Time:      7  15   30
Distance:  9  40  200
"#;

    const PART1_RESULT: OutputData = 288;
    const PART2_RESULT: OutputData = 71503;

    #[test]
    fn test_parser() {
        let parsed = input_generator(TEST_DATA);
        let ret = ChallangeData::default();

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

#[derive(Debug, PartialEq, Eq)]
pub struct Data(Vec<RaceData>);

#[derive(Debug, PartialEq, Eq, Default)]
pub struct RaceData {
    pub time: u64,
    pub dst: u64,
}

impl Default for Data {
    fn default() -> Self {
        Self(vec![
            RaceData { time: 7, dst: 9 },
            RaceData { time: 15, dst: 40 },
            RaceData { time: 30, dst: 200 },
        ])
    }
}

impl FromStr for Data {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s
            .trim()
            .split("\n")
            .filter(|l| !l.is_empty())
            .collect::<Vec<_>>();
        let times = split[0]
            .trim()
            .split(":")
            .last()
            .unwrap()
            .trim()
            .split(" ")
            .filter(|n| !n.is_empty())
            .map(|n| {
                n.trim()
                    .parse::<u64>()
                    .expect("Could not parse time number")
            })
            .collect::<Vec<_>>();
        let distances = split[1]
            .trim()
            .split(":")
            .last()
            .unwrap()
            .trim()
            .split(" ")
            .filter(|n| !n.is_empty())
            .map(|n| {
                n.trim()
                    .parse::<u64>()
                    .expect("Could not parse distance number")
            })
            .collect::<Vec<_>>();

        let count = times.len();
        let samples = (0..count)
            .map(|i| RaceData {
                time: times[i],
                dst: distances[i],
            })
            .collect::<Vec<_>>();

        Ok(Self(samples))
    }
}

impl RaceData {
    pub fn get_hold_time_win_possibilities(&self) -> Vec<u64> {
        (0..=self.time)
            .filter_map(|i| {
                let hold = i;
                let run = self.time - hold;
                let distance = hold * run;
                match distance > self.dst {
                    true => Some(hold),
                    false => None,
                }
            })
            .collect::<Vec<_>>()
    }
}

impl Data {
    pub fn merge_samples(&self) -> RaceData {
        let mut time = String::new();
        let mut dst = String::new();
        self.0.iter().for_each(|s| {
            time += &s.time.to_string();
            dst += &s.dst.to_string();
        });

        let time = time.parse::<u64>().expect("Could not parse merged time");
        let dst = dst.parse::<u64>().expect("Could not parse merged dst");

        RaceData { time, dst }
    }
}
