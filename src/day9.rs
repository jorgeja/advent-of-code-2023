use std::{cmp::Ordering, collections::HashMap, error::Error, num, str::FromStr};
const EXAMPLE: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;

fn solve(input: &str) -> (i32, i32) {
    let mut last_vals = Vec::new();
    let mut first_vals = Vec::new();
    let mut current_sequence = Vec::new();
    let mut next_sequence = Vec::new();
    let mut sum = (0, 0);
    input.lines().for_each(|line| {
        current_sequence.clear();
        current_sequence.extend(
            line.split(' ')
                .map(str::parse::<i32>)
                .filter_map(|res| res.ok()),
        );

        last_vals.push(current_sequence.last().copied().unwrap());
        first_vals.push(current_sequence.first().copied().unwrap());

        while current_sequence.len() > 1
            && current_sequence
                .windows(2)
                .any(|elems| elems[0] != elems[1])
        {
            next_sequence.clear();
            for elems in current_sequence.windows(2) {
                next_sequence.push(elems[1] - elems[0]);
            }
            last_vals.push(next_sequence.last().copied().unwrap());
            first_vals.push(next_sequence.first().copied().unwrap());
            current_sequence.clear();
            current_sequence.extend(next_sequence.iter().copied());
        }

        let mut last_val = 0;
        while let Some(val) = last_vals.pop() {
            last_val = val + last_val;
        }
        let mut first_val = 0;
        while let Some(val) = first_vals.pop() {
            first_val = val - first_val;
        }
        sum = (sum.0 + first_val, sum.1 + last_val);
    });
    sum
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
    fn day9_test() {
        let res = solve(EXAMPLE);
        println!("day9 example {res:?}");
        assert_eq!(res, (2, 114));
    }

    #[test]
    fn day9() -> Result<(), Box<dyn Error>> {
        let input = get_input(2023, 9)?;
        let res = solve(&input);
        println!("day9 result: {res:?}");
        Ok(())
    }

    // #[test]
    // fn day9_part2_test() {
    //     let res = solve_part2(EXAMPLE_2);
    //     assert_eq!(res, 5905);
    // }

    // #[test]
    // fn day9_part2() -> Result<(), Box<dyn Error>> {
    //     let input = get_input(2023, 9)?;
    //     let res = solve_part2(&input);
    //     println!("day9 Part2 Result: {res}");
    //     Ok(())
    // }
}
