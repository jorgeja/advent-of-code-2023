use std::{error::Error, num, str::FromStr, collections::HashSet};

const EXAMPLE: &str = r#"seeds: 79 14 55 13

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

#[derive(Debug, Default, Clone, Copy)]
struct Range {
    input_start: u64,
    output_start: u64,
    length: u64,
}

impl Range {
    fn mapped(&self, input: u64) -> Option<u64> {
        if self.input_start <= input && input < self.input_start + self.length {
            let diff = input - self.input_start;
            Some(self.output_start + diff)
        } else {
            None
        }
    }
}

impl FromStr for Range {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums = s
            .split(" ")
            .filter_map(|n| n.parse::<u64>().ok())
            .collect::<Vec<_>>();

        if nums.len() != 3 {
            //println!("Range::from_str: '{s}' parsed to {:?}", nums);
            return Err(())
        }
        
        Ok(Range { input_start: nums[1], output_start: nums[0], length: nums[2]})
    }
}

#[derive(Debug, Default)]
struct Map {
    name: String,
    ranges: Vec<Range>
}

impl Map {
    fn mapped(&self, input: u64) -> u64 {
        let last = self.ranges.last().unwrap();
        if input < self.ranges[0].input_start || last.input_start + last.length <= input {
            // println!("{}: {input} outside bounds ", self.name);
            return input;
        } else {
            for r in &self.ranges {
                if let Some(mapped_input) = r.mapped(input) {
                    // println!("{}: Found {input} -> {mapped_input} in {:?}", self.name, r);
                    return mapped_input;
                }
            }
        }
        input
    }
}

fn parse(input: &str) -> (Vec<u64>, Vec<Map>){
    let mut lines = input.lines();

    let seeds = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .filter_map(|n| n.parse::<u64>().ok())
        .collect::<Vec<_>>();

    lines.next().unwrap(); // skip first empty line..

    let mut current_map: Option<Map> = None;
    let mut all_maps = Vec::default();

    for line in lines {
        if let Some(mut map) = current_map {
            if line.is_empty() {
                map.ranges.sort_by(|e1, e2| e1.input_start.cmp(&e2.input_start));
                all_maps.push(map);
                current_map = None;
            } else {
                map.ranges.push(Range::from_str(line).unwrap());
                current_map = Some(map);
            }
        } else {
            let mut new_map = Map::default();
            new_map.name = line.split(" map:").next().unwrap().to_owned();
            current_map = Some(new_map);
        }
    }

    if let Some(mut map) = current_map {
        map.ranges.sort_by(|e1, e2| e1.input_start.cmp(&e2.input_start));
        all_maps.push(map);
    }

    // for m in &all_maps {
    //     println!("{:?}", m);
    // }

    (seeds, all_maps)
}

fn solve_part1(input: &str) -> u64 {
    let (seeds, all_maps) = parse(input);

    seeds.iter().map(|seed|{
        let mut v = *seed;
        for map in &all_maps {
        //    print!("[{seed}] ");
           v = map.mapped(v)
        }
        v
    }).min().unwrap()
}

fn solve_part2(input: &str) -> u64 {
    let (seeds, all_maps) = parse(input);

    let mut all_seeds = HashSet::new();
    for v in seeds.chunks(2) {
        all_seeds.extend(v[0]..v[0]+v[1]);
    }

    all_seeds.iter().map(|seed|{
        let mut v = *seed;
        for map in &all_maps {
           //    print!("[{seed}] ");
           v = map.mapped(v)
        }
        v
    }).min().unwrap()
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::get_input;

    use super::*;

    #[test]
    fn day5_part1_test() {
        let res = solve_part1(EXAMPLE);
        assert_eq!(res, 35);
    }

    #[test]
    fn day5_part1() -> Result<(), Box<dyn Error>> {
        let input = get_input(2023, 5)?;
        let res = solve_part1(&input);
        println!("day5 Part1 Result: {res}");
        Ok(())
    }

    #[test]
    fn day5_part2_test() {
        let res = solve_part2(EXAMPLE);
        assert_eq!(res, 46);
    }

    #[test]
    fn day5_part2() -> Result<(), Box<dyn Error>> {
        let input = get_input(2023, 5)?;
        let res = solve_part2(&input);
        println!("day5 Part2 Result: {res}");
        Ok(())
    }
}
