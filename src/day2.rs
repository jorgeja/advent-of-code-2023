use std::{cmp::Ordering, error::Error, str::FromStr};

const EXAMPLE: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct CubeSet {
    red: i32,
    green: i32,
    blue: i32,
}

impl CubeSet {
    fn power(&self) -> i32 {
        self.red * self.green * self.blue
    }

    fn max(&self, other: &Self) -> Self {
        CubeSet {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }
}

impl PartialOrd for CubeSet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let r_cmp = self.red.cmp(&other.red);
        let g_cmp = self.green.cmp(&other.green);
        let b_cmp = self.blue.cmp(&other.blue);
        match &[r_cmp, g_cmp, b_cmp] {
            [Ordering::Equal, Ordering::Equal, Ordering::Equal] => Some(Ordering::Equal),
            ord if ord.iter().any(|ord| *ord == Ordering::Greater) => Some(Ordering::Greater),
            ord if ord
                .iter()
                .all(|ord| *ord == Ordering::Less || *ord == Ordering::Equal) =>
            {
                Some(Ordering::Less)
            }
            _ => None,
        }
    }
}

impl FromStr for CubeSet {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cubes_set = CubeSet {
            red: 0,
            green: 0,
            blue: 0,
        };
        for cubes in s.split(", ") {
            let mut split = cubes.split(" ");
            let num = split
                .next()
                .ok_or("missing number of cubes")?
                .parse::<i32>()?;

            let color = split.next().ok_or("missing cube color")?;
            match color {
                "red" => cubes_set.red = num,
                "green" => cubes_set.green = num,
                "blue" => cubes_set.blue = num,
                _ => {}
            }
        }
        Ok(cubes_set)
    }
}

struct Game {
    num: i32,
    cube_sets: Vec<CubeSet>,
}

impl Game {
    fn is_valid(&self, rules: CubeSet) -> bool {
        self.cube_sets.iter().all(|cs| *cs <= rules)
    }

    fn min_cube_set(&self) -> CubeSet {
        self.cube_sets
            .iter()
            .fold(CubeSet::default(), |m, cs| cs.max(&m))
    }
}

impl FromStr for Game {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut game = Game {
            num: 0,
            cube_sets: Vec::default(),
        };
        let mut colon_split = s.split(": ");
        let game_info = colon_split.next().ok_or("missing game info")?;
        let mut game_split = game_info.split(" ");
        _ = game_split.next().ok_or("il formed game info")?;
        game.num = game_split
            .next()
            .ok_or("missing game number")?
            .parse::<i32>()?;

        let cube_sets = colon_split.next().ok_or("missing cube sets after ':'")?;
        for cube_set_str in cube_sets.split("; ") {
            match CubeSet::from_str(cube_set_str) {
                Ok(cs) => game.cube_sets.push(cs),
                Err(e) => println!("{:?}", e),
            };
        }

        Ok(game)
    }
}

fn solve_part1(input: &str, rules: CubeSet) -> i32 {
    input
        .lines()
        .filter_map(|line| Game::from_str(line).ok())
        .filter_map(|game| {
            if game.is_valid(rules) {
                Some(game.num)
            } else {
                None
            }
        })
        .sum()
}

fn solve_part2(input: &str) -> i32 {
    input
        .lines()
        .filter_map(|line| Game::from_str(line).ok())
        .map(|game| game.min_cube_set())
        .map(|cube_set| cube_set.power())
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::get_input;

    use super::*;
    #[test]
    fn day2_part1_test() {
        let rules = CubeSet {
            red: 12,
            green: 13,
            blue: 14,
        };

        let res = solve_part1(EXAMPLE, rules);
        assert_eq!(res, 8);
    }

    #[test]
    fn day2_part1() -> Result<(), Box<dyn Error>> {
        let rules = CubeSet {
            red: 12,
            green: 13,
            blue: 14,
        };
        let input = get_input(2023, 2)?;
        let res = solve_part1(&input, rules);
        println!("Day2 Part1 Result: {res}");
        Ok(())
    }
    #[test]
    fn day2_part2_test() {
        let res = solve_part2(EXAMPLE);
        assert_eq!(res, 2286);
    }

    #[test]
    fn day2_part2() -> Result<(), Box<dyn Error>> {
        let input = get_input(2023, 2)?;
        let res = solve_part2(&input);
        println!("Day2 Part2 Result: {res}");
        Ok(())
    }
}
