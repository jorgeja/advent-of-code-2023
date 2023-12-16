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
    let (instructions, nodes) = parse(input);

    let mut start_nodes = nodes
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|s| s.as_str())
        .collect::<Vec<_>>();
    
    start_nodes.sort();

    let mut cycle_steps = Vec::new();
    for start_node in start_nodes {
        let mut last_seen = HashMap::new();
        let mut next_node = start_node;
        let mut steps = 1usize;
        let mut last_seen_steps = 0usize;
        let mut z_has_been_seen = false;
        'cycle: loop {
            for inst in instructions.chars() {
                if next_node.contains('Z') {
                    z_has_been_seen = true;
                    println!("{} : {}", next_node, steps);
                }

                let node = &nodes[next_node];
                if let Some(last_steps) = last_seen.get(&(next_node, inst)) {
                    if z_has_been_seen {
                        last_seen_steps = *last_steps;
                        break 'cycle;
                    }
                } else {
                    last_seen.insert((next_node, inst), steps);
                }

                match inst {
                    'L' => next_node = &node.0,
                    'R' => next_node = &node.1,
                    _ => {}
                }

                steps += 1;
            }
        }

        cycle_steps.push((next_node.to_owned(), last_seen_steps, steps));
        println!("{:?}", cycle_steps.last());
    }

    for (_, ls, s) in cycle_steps {
        print!(" {},", s-ls);
    }

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
    //     let res = solve_part2(EXAMPLE_PART2);
    //     assert_eq!(res, 6);
    // }

    #[test]
    fn day8_part2() -> Result<(), Box<dyn Error>> {
        let input = get_input(2023, 8)?;
        let res = solve_part2(&input);
        println!("day8 Part2 Result: {res}");
        
        // let factors = vec![2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 3 , 19 , 1879049 , 69751037u64];
        // let res = factors.iter().fold(1, |acc, v| acc * *v);
        Ok(())
    }


}
