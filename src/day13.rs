use std::fmt::Write;

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

impl std::fmt::Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("W: {} H: {}\n", self.width, self.height))?;
        for r in 0..self.height {
            for c in 0..self.width {
                let chr = self.index( r, c);
                f.write_char(*chr as char)?;
            }

            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Pattern {
    fn index(&self, row: usize, col: usize) -> &u8 {
        let i = row * self.width + col;
        self.data.get(i).unwrap_or(&0)
    }

    fn find_vertical_mirror(&self, use_smudge: bool) -> usize {
        let mut the_smudge = None;
        let mut mirror_cols = Vec::new();

        for c in 0..(self.width-1) {
            let mut unequal = 0;
            let mut smudges = 0;
            let mut smudge = (0, 0, 0, 0);

            for c2 in c+1..self.width {
                let c1 = c - (c2 - c - 1);
                //print!("{c}: Checking col {c1} vs {c2}");
                
                let count_unequal = (0..self.height)
                    .map(|row| (row, self.index(row, c1), self.index(row, c2)))
                    .filter(|(row, e1, e2)| if e1 != e2 {
                        smudge = (*row, c, c1, c2);
                        true
                    } else { false })
                    .count();
                
                if count_unequal == 1 {
                    smudges += 1;
                    //println!(" > Smudge: {:?}", smudge);
                } else {
                    //println!("");
                }

                if count_unequal > 0 {
                    unequal += 1;
                }

                if c1 == 0 {
                    break;
                }
            }

            if smudges == 1 && unequal == 1{
                the_smudge = Some(smudge);
            }

            if unequal == 0 {
                mirror_cols.push(c+1);
            }
        }
        if let Some(smdg) = the_smudge {
            //println!("Col Smudge: [{}, {}] {} [{}, {}]", smdg.0, smdg.2, smdg.1, smdg.0, smdg.3);
            if use_smudge {
                //println!("Smudge Col Mirrors: {:?}", smdg.1 + 1);
                return smdg.1 + 1;
            }
        }
        if use_smudge {
            return 0;
        }

        //println!("Col Mirrors: {:?}", &mirror_cols);
        *mirror_cols.last().unwrap_or(&0)
    }

    fn find_horizontal_mirror(&self, use_smudge: bool) -> usize {
        let mut the_smudge = None;
        let mut mirror_rows = Vec::new();

        for r in 0..(self.height-1) {
            let mut unequal = 0;
            let mut smudges = 0;
            let mut smudge = (0, 0, 0, 0);
            for r2 in r+1..self.height {
                let r1 = r - (r2 - r - 1);
                //print!("{r}: Checking row {r1} vs {r2}");
                let count_unequal = (0..self.width)
                    .map(|col: usize| (col, self.index(r1, col), self.index(r2, col)))
                    .filter(|(col, e1, e2)| if e1 != e2 {
                        smudge = (*col, r, r1, r2);
                        true
                    } else { false })
                    .count();
                
                if count_unequal == 1 {
                    smudges += 1;
                    //println!(" > {smudges} Smudge: {:?}", smudge);
                } else {
                    //println!("");
                }

                if count_unequal > 0 {
                    unequal += 1;
                }

                if r1 == 0 {
                    break;
                }
            }

            if smudges == 1 && unequal == 1 {
                the_smudge = Some(smudge);
            }

            if unequal == 0 {
                mirror_rows.push(r+1);
            }
        }

        if let Some(smdg) = the_smudge {
            //println!("Row Smudge: [{}, {}] {} [{}, {}]", smdg.2, smdg.0, smdg.1, smdg.3, smdg.0);
            if use_smudge {
                //println!("Smudge Row Mirrors: {:?}", smdg.1 + 1);
                return smdg.1 + 1;
            }
        }

        if use_smudge {
            return 0;
        }
        
        //println!("Row Mirrors: {:?}", &mirror_rows);
        *mirror_rows.last().unwrap_or(&0)
    }

    fn mirror_value(&self, use_smudge: bool) -> usize {
        let col_val = self.find_vertical_mirror(use_smudge);
        //println!("Column Mirror: {}", col_val);
        let row_val = self.find_horizontal_mirror(use_smudge);
        //println!("Row Mirror: {}", row_val);
        let v = col_val + 100 * row_val;
        //println!("Value: {v}");
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
    patterns.iter().map(|pat| pat.mirror_value(false) as u32).sum()
}

fn solve_part2(input: &str) -> u32 {
    let patterns = parse(input);
    patterns.iter().map(|pat| {println!("{}", pat); pat.mirror_value(true) as u32}).sum()
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

    #[test]
    fn day13_part2_test() {
        let res = solve_part2(EXAMPLE);
        assert_eq!(res, 400);
    }

    #[test]
    fn day13_part2() -> Result<(), Box<dyn Error>> {
        let input = get_input(2023, 13)?;
        let res = solve_part2(&input);
        println!("day13 Part2 Result: {res}");
        Ok(())
    }
}
