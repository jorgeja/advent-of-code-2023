use std::{collections::HashMap, error::Error, num, str::FromStr, cmp::Ordering};
const EXAMPLE: &str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;

const EXAMPLE_2: &str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;

fn parse(input: &str) -> (String, HashMap<String, (String, String)>) {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().to_owned();
    
    lines.next();

    let nodes = lines.filter_map(|l| {
        let mut parts = l.split(" = ");
        let key = parts.next().unwrap().to_owned();

        let mut lr = parts.next().unwrap().split(", ");
        let left = lr.next().unwrap().strip_prefix('(').unwrap().to_owned();
        let right = lr.next().unwrap().strip_suffix(')').unwrap().to_owned();
        Some((key, (left, right)))
    }).collect();

    (instructions, nodes)
}

fn solve_part1(input: &str) -> u32 {
    let (instructions, nodes) = parse(input);

    let mut next_node = "AAA";
    let mut steps = 0;
    while next_node != "ZZZ" {
        for inst in instructions.chars() {
            let node = &nodes[next_node];
            match inst {
                'L' => next_node = &node.0,
                'R' => next_node = &node.1,
                _ => {}
            }
            steps += 1;
            if next_node == "ZZZ" {
                break;
            }
        }
    }

    steps
}

fn solve_part2(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::get_input;

    use super::*;

    #[test]
    fn day8_part1_test() {
        let res = solve_part1(EXAMPLE);
        assert_eq!(res, 2);

        let res = solve_part1(EXAMPLE_2);
        assert_eq!(res, 6);
    }

    #[test]
    fn day8_part1() -> Result<(), Box<dyn Error>> {
        let input = get_input(2023, 8)?;
        let res = solve_part1(&input);
        println!("day8 Part1 Result: {res}");
        Ok(())
    }

    // #[test]
    // fn day8_part2_test() {
    //     let res = solve_part2(EXAMPLE_2);
    //     assert_eq!(res, 5905);
    // }

    // #[test]
    // fn day8_part2() -> Result<(), Box<dyn Error>> {
    //     let input = get_input(2023, 8)?;
    //     let res = solve_part2(&input);
    //     println!("day8 Part2 Result: {res}");
    //     Ok(())
    // }
}
