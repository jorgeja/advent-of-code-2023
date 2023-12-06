use std::{collections::HashSet, error::Error, num, str::FromStr};
const EXAMPLE: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;

#[derive(Debug, Clone, Copy)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn solve(&self) -> u64 {
        let mut winning_solutions = 0;
        for t in 1..self.time {
            let d = distance(t, self.time);
            if d > self.distance {
                winning_solutions += 1;
            }
        }

        winning_solutions
    }
}

fn distance(hold_time: u64, max_time: u64) -> u64 {
    (max_time - hold_time) * hold_time
}

fn parse(input: &str) -> Vec<Race> {
    let mut lines = input.lines();
    let times = lines
        .next()
        .unwrap()
        .split(" ")
        .filter_map(|v| v.parse::<u64>().ok())
        .collect::<Vec<_>>();
    let distances = lines
        .next()
        .unwrap()
        .split(" ")
        .filter_map(|v| v.parse::<u64>().ok())
        .collect::<Vec<_>>();

    if times.len() != distances.len() {
        println!(
            "Bad parsing of input: {} != {}",
            times.len(),
            distances.len()
        );
        return Vec::default();
    }

    let mut races = Vec::default();
    for i in 0..times.len() {
        races.push(Race {
            time: times[i],
            distance: distances[i],
        })
    }

    races
}

fn parse2(input: &str) -> Race {
    let mut lines = input.lines();
    let times = lines
        .next()
        .unwrap()
        .split("Time:")
        .nth(1)
        .unwrap()
        .split(" ")
        .collect::<String>();
    let distances = lines
        .next()
        .unwrap()
        .split("Distance:")
        .nth(1)
        .unwrap()
        .split(" ")
        .collect::<String>();

    Race {
        time: times.parse().unwrap(),
        distance: distances.parse().unwrap(),
    }
}

fn solve_part1(input: &str) -> u64 {
    let races = parse(input);
    races.iter().fold(1, |acc, r| acc * r.solve())
}

fn solve_part2(input: &str) -> u64 {
    parse2(input).solve()
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::get_input;

    use super::*;

    #[test]
    fn day6_part1_test() {
        let res = solve_part1(EXAMPLE);
        assert_eq!(res, 288)
    }

    #[test]
    fn day6_part1() -> Result<(), Box<dyn Error>> {
        let input = get_input(2023, 6)?;
        let res = solve_part1(&input);
        println!("day6 Part1 Result: {res}");
        Ok(())
    }

    #[test]
    fn day6_part2_test() {
        let res = solve_part2(EXAMPLE);
        assert_eq!(res, 71503);
    }

    #[test]
    fn day6_part2() -> Result<(), Box<dyn Error>> {
        let input = get_input(2023, 6)?;
        let res = solve_part2(&input);
        println!("day6 Part2 Result: {res}");
        Ok(())
    }
}
