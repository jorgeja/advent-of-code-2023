use std::{cmp::Ordering, collections::HashMap, error::Error, num, str::FromStr};
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

const EXAMPLE_PART2: &str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
"#;

fn parse(input: &str) -> (String, HashMap<String, (String, String)>) {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().to_owned();

    lines.next();

    let nodes = lines
        .filter_map(|l| {
            let mut parts = l.split(" = ");
            let key = parts.next().unwrap().to_owned();

            let mut lr = parts.next().unwrap().split(", ");
            let left = lr.next().unwrap().strip_prefix('(').unwrap().to_owned();
            let right = lr.next().unwrap().strip_suffix(')').unwrap().to_owned();
            Some((key, (left, right)))
        })
        .collect();

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

fn solve_part2(input: &str) -> u64 {
    let (_, nodes) = parse(input);

    let mut next_nodes = nodes
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|s| (s.as_str(), 0))
        .collect::<Vec<_>>();
    next_nodes.sort();
    //let end_nodes = nodes.keys().filter(|k| k.ends_with('Z') ).map(|s| s.as_str()).collect::<Vec<_>>();
    println!("Num start nodes: {}", next_nodes.len());
    for (n, s) in &next_nodes {
        println!("{}, {}", n, s);
    }

    // println!("Num end nodes: {}", end_nodes.len());
    // for s in &end_nodes {
    //     println!("{s}");
    // }

    let mut previously_visited = HashMap::new();
    let mut steps = 0u64;
    while next_nodes.iter().any(|(_, step)| *step == 0) {
        steps += 1;
        let mod_step: u8 = (steps % 2) as u8;

        for (i, (node, step)) in next_nodes.iter_mut().enumerate() {
            let (left, right) = &nodes[*node];

            if let Some(last_steps) = previously_visited.get(&(i as u8, mod_step, *node)) {
                if i == 2 {
                    println!("{steps}: Cycle found for {node}:{step} on path {i} last visited at {last_steps}: {left} {right}");
                }
            } else {
                previously_visited.insert((i as u8, mod_step, *node), steps);
            }
            match mod_step {
                0 => *node = right.as_ref(),
                1 => *node = left.as_ref(),
                _ => unreachable!(),
            }

            if node.ends_with("Z") {
                *step = steps;
                break;
            }

            if node.contains("Z") {
                println!("Z-node: {node}");
            }
        }

        if steps > 200 {
            break;
        }
    }

    println!("Current state:");
    for (n, s) in next_nodes {
        println!("{}, {}", n, s);
    }

    steps
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
    //     let res = solve_part2(EXAMPLE_PART2);
    //     assert_eq!(res, 6);
    // }

    #[test]
    fn day8_part2() -> Result<(), Box<dyn Error>> {
        let input = get_input(2023, 8)?;
        let res = solve_part2(&input);
        println!("day8 Part2 Result: {res}");
        Ok(())
    }
}
