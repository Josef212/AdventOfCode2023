use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
fn input_generator(input: &str) -> Data {
    Data::from_str(input).expect("Could not parse input")
}

#[aoc(day5, part1)]
pub fn part1(input: &Data) -> u32 {
    input
        .seeds
        .iter()
        .map(|s| input.map_seed_to_location(*s))
        .min()
        .expect("Could not get min location")
}

#[aoc(day5, part2)]
pub fn part2(input: &Data) -> u32 {
    input
        .seeds
        .chunks(2)
        .map(|p| {
            let start = &p[0];
            let len = p[1];
            (0..len).map(|i| {
                let seed = i + *start;
                input.map_seed_to_location(seed)
            })
        })
        .flatten()
        .min()
        .expect("Could not get min location")
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_DATA: &str = r#"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"#;

    const PART1_RESULT: u32 = 35;
    const PART2_RESULT: u32 = 46;

    #[test]
    fn test_parser() {
        let parsed = input_generator(TEST_DATA);
        let ret = Data::default();

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
pub struct Data {
    pub seeds: Vec<u32>,
    pub soil_to_fertilizer: DataMap,
    pub seed_to_soil: DataMap,
    pub fertilizer_to_water: DataMap,
    pub water_to_light: DataMap,
    pub light_to_temperature: DataMap,
    pub temperature_to_humidity: DataMap,
    pub humidity_to_location: DataMap,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DataMap(Vec<Mapping>);

#[derive(Debug, PartialEq, Eq)]
pub struct Mapping {
    pub destination_range_start: u32,
    pub source_range_start: u32,
    pub range_len: u32,
}

impl FromStr for Data {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.trim().split("\n\n").collect::<Vec<_>>();

        let seeds = split[0]
            .split(":")
            .last()
            .unwrap()
            .trim()
            .split(" ")
            .map(|a| {
                a.trim()
                    .parse::<u32>()
                    .expect("Could not parse seed number")
            })
            .collect();

        let seed_to_soil =
            DataMap::from_str(split[1]).expect("Could not parse DataMap for seed_to_soil");
        let soil_to_fertilizer =
            DataMap::from_str(split[2]).expect("Could not parse DataMap for soil_to_fertilizer");
        let fertilizer_to_water =
            DataMap::from_str(split[3]).expect("Could not parse DataMap for fertilizer_to_water");
        let water_to_light =
            DataMap::from_str(split[4]).expect("Could not parse DataMap for water_to_light");
        let light_to_temperature =
            DataMap::from_str(split[5]).expect("Could not parse DataMap for light_to_temperature");
        let temperature_to_humidity = DataMap::from_str(split[6])
            .expect("Could not parse DataMap for temperature_to_humidity");
        let humidity_to_location =
            DataMap::from_str(split[7]).expect("Could not parse DataMap for humidity_to_location");

        Ok(Self {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        })
    }
}

impl FromStr for DataMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.trim().split("\n").collect::<Vec<_>>();
        let mappings = split
            .iter()
            .skip(1)
            .map(|line| Mapping::from_str(line).expect("Could not parse mapping"))
            .collect();

        Ok(Self(mappings))
    }
}

impl FromStr for Mapping {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.trim().split(" ").collect::<Vec<_>>();
        let a = split[0]
            .trim()
            .parse::<u32>()
            .expect("Could not parse destination_range_start");
        let b = split[1]
            .trim()
            .parse::<u32>()
            .expect("Could not parse source_range_start");
        let c = split[2]
            .trim()
            .parse::<u32>()
            .expect("Could not parse range_leng");

        Ok(Self {
            destination_range_start: a,
            source_range_start: b,
            range_len: c,
        })
    }
}

impl Default for Data {
    fn default() -> Self {
        let seeds = vec![79, 14, 55, 13];
        let seed_to_soil = DataMap(vec![Mapping::new(50, 98, 2), Mapping::new(52, 50, 48)]);
        let soil_to_fertilizer = DataMap(vec![
            Mapping::new(0, 15, 37),
            Mapping::new(37, 52, 2),
            Mapping::new(39, 0, 15),
        ]);
        let fertilizer_to_water = DataMap(vec![
            Mapping::new(49, 53, 8),
            Mapping::new(0, 11, 42),
            Mapping::new(42, 0, 7),
            Mapping::new(57, 7, 4),
        ]);
        let water_to_light = DataMap(vec![Mapping::new(88, 18, 7), Mapping::new(18, 25, 70)]);
        let light_to_temperature = DataMap(vec![
            Mapping::new(45, 77, 23),
            Mapping::new(81, 45, 19),
            Mapping::new(68, 64, 13),
        ]);
        let temperature_to_humidity = DataMap(vec![Mapping::new(0, 69, 1), Mapping::new(1, 0, 69)]);
        let humidity_to_location = DataMap(vec![Mapping::new(60, 56, 37), Mapping::new(56, 93, 4)]);

        Self {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        }
    }
}

impl Mapping {
    pub fn new(destination_range_start: u32, source_range_start: u32, range_len: u32) -> Self {
        Self {
            destination_range_start,
            source_range_start,
            range_len,
        }
    }

    pub fn map(&self, val: u32) -> u32 {
        let diff = val - self.source_range_start;
        let map = self.destination_range_start + diff;
        map
    }

    pub fn is_mapping_valid(&self, val: u32) -> bool {
        val >= self.source_range_start && val < self.source_range_start + self.range_len
    }
}

impl DataMap {
    pub fn map(&self, val: u32) -> u32 {
        match self.get_mapping(val) {
            None => val,
            Some(mapping) => mapping.map(val),
        }
    }

    fn get_mapping(&self, val: u32) -> Option<&Mapping> {
        self.0.iter().find(|m| m.is_mapping_valid(val))
    }
}

impl Data {
    pub fn map_seed_to_location(&self, seed: u32) -> u32 {
        let soil = self.seed_to_soil.map(seed);
        let fertilizer = self.soil_to_fertilizer.map(soil);
        let water = self.fertilizer_to_water.map(fertilizer);
        let light = self.water_to_light.map(water);
        let temperature = self.light_to_temperature.map(light);
        let humidity = self.temperature_to_humidity.map(temperature);
        let location = self.humidity_to_location.map(humidity);

        location
    }
}
