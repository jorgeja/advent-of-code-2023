const EXAMPLE: &str = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;

#[derive(Debug, Default)]
struct Pattern {
    height: usize,
    width: usize,
    data: Vec<u8>,
}

impl Pattern {
    fn index(&self, row: usize, col: usize) -> &u8 {
        let i = row * self.width + col;
        self.data.get(i).unwrap_or(&0)
    }

    fn find_vertical_mirror(&self) -> usize {
        'mirror_line: for c in 0..(self.width-1) {
            for c2 in c+1..self.width {
                let c1 = c - (c2 - c - 1);
                //println!("{c}: Checking col {c1} vs {c2}");
                if ! (0..self.height)
                .map(|row| (self.index(row, c1), self.index(row, c2)))
                .all(|(e1, e2)| e1 == e2) {
                    continue 'mirror_line;
                }
                if c1 == 0 {
                    break;
                }
            }
            return c + 1;
        }
        0
    }
    fn find_horizontal_mirror(&self) -> usize {
        'mirror_line: for r in 0..(self.height-1) {
            for r2 in r+1..self.height {
                let r1 = r - (r2 - r - 1);
                //println!("Checking row {r1} vs {r2}");
                if ! (0..self.width)
                .map(|col| (self.index(r1, col), self.index(r2, col)))
                .all(|(e1, e2)| e1 == e2) {
                    continue 'mirror_line;
                }
                if r1 == 0 {
                    break;
                }
            }
            return r+1;
        }
        0
    }

    fn mirror_value(&self) -> usize {
        let col_val = self.find_vertical_mirror();
        println!("Column Mirror: {}", col_val);
        let row_val = self.find_horizontal_mirror();
        println!("Row Mirror: {}", row_val);
        if col_val == 0 && row_val == 0 {
            for r in 0..self.height {
                for c in 0..self.width {
                    print!("{}", *self.index(r, c) as char)
                }
                println!("");
            }
        }
        let v = col_val + 100 * row_val;
        println!("Value: {v}");
        v
    }
}

fn parse(input: &str) -> Vec<Pattern> {
    let mut patterns = Vec::new();
    let mut current_pattern = Pattern::default();
    for line in input.lines() {
        if line.is_empty() {
            patterns.push(current_pattern);
            current_pattern = Pattern::default();
        } else {
            let last_width = current_pattern.data.len();
            current_pattern.data.extend(line.as_bytes().iter().copied());
            current_pattern.width = current_pattern
                .width
                .max(current_pattern.data.len() - last_width);
            current_pattern.height += 1;
        }
    }
    patterns.push(current_pattern);
    patterns
}

fn solve_part1(input: &str) -> u32 {
    let patterns = parse(input);
    patterns.iter().map(|pat| pat.mirror_value() as u32).sum()
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::get_input;

    use super::*;

    #[test]
    fn day13_part1_test() {
        let res = solve_part1(EXAMPLE);
        println!("{res}");
        assert_eq!(res, 405);
    }

    #[test]
    fn day13_part1() -> Result<(), Box<dyn Error>> {
        let input = get_input(2023, 13)?;
        let res = solve_part1(&input);
        println!("day13 Part1 Result: {res}");
        Ok(())
    }

    // #[test]
    // fn day13_part2_test() {
    //     let res = solve_part2(10, EXAMPLE);
    //     assert_eq!(res, 1030);
    //     let res = solve_part2(100, EXAMPLE);
    //     assert_eq!(res, 8410);
    // }

    // #[test]
    // fn day13_part2() -> Result<(), Box<dyn Error>> {
    //     let input = get_input(2023, 11)?;
    //     let res = solve_part2(1_000_000, &input);
    //     println!("day13 Part2 Result: {res}");
    //     Ok(())
    // }
}
