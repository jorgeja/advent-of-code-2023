use std::{collections::HashMap, fmt::Write, path::Display};

const EXAMPLE: &str = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

#[derive(Debug, Default)]
struct Pattern {
    height: usize,
    width: usize,
    data: Vec<u8>,
    buffer: Vec<u8>,
}

fn get_index(data: &[u8], width: usize, row: usize, col: usize) -> u8 {
    let i = row * width + col;
    *data.get(i).unwrap_or(&0)
}

impl std::fmt::Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("W: {} H: {}\n", self.width, self.height))?;
        for r in 0..self.height {
            for c in 0..self.width {
                let rock = get_index(&self.data, self.width, r, c);
                f.write_char(rock as char)?;
            }

            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Pattern {
    fn rotate(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                let cur_rock = get_index(&self.data, self.width, row, col);
                let new_row = col;
                let new_col = self.width - 1 - row;
                self.buffer[new_row * self.width + new_col] = cur_rock;
            }
        }
        self.data.clear();
        self.data.extend(self.buffer.iter().copied());
    }

    fn tilt_north(&mut self) {
        self.buffer = self.data.clone();

        for row in 1..self.height {
            for col in 0..self.width {
                let cur_rock = get_index(&self.data, self.width, row, col);
                if !(cur_rock == b'O') {
                    continue;
                }

                let mut rock_row = row;
                let orig_idx = row * self.width + col;

                for past_row in (0..row).rev() {
                    let rock = get_index(&self.buffer, self.width, past_row, col);
                    if rock != b'.' {
                        break;
                    } else {
                        rock_row = past_row;
                    }
                }

                self.buffer[orig_idx] = b'.';
                self.buffer[rock_row * self.width + col] = b'O';
            }
        }

        self.data.clear();
        self.data.extend(self.buffer.iter().copied());
    }

    fn weight(&mut self) -> usize {
        let mut weight = 0;
        for r in 0..self.height {
            for c in 0..self.width {
                let rock = get_index(&self.data, self.width, r, c);
                if rock == b'O' {
                    weight += self.height - r;
                }
            }
        }
        weight
    }

    fn cycle(&mut self) {
        for _ in 0..4 {
            self.tilt_north();
            self.rotate();
        }
    }

    fn cycle_amount(&mut self, cycles: usize) -> usize {
        let mut cache: HashMap<Vec<u8>, usize> = HashMap::new();

        let mut found_cycle = 0;
        for i in 0..cycles {
            if let Some(cycle) = cache.get(&self.data) {
                let div = (cycles - i) / (i - cycle);
                found_cycle = div * (i - cycle) + i;
                break;
            } else {
                let start_state = self.data.clone();
                self.cycle();
                cache.insert(start_state, i);
            }
        }

        for _ in found_cycle..cycles {
            self.cycle();
        }
        self.weight()
    }
}

fn parse(input: &str) -> Pattern {
    let mut pattern = Pattern::default();
    let mut width = 0;
    for line in input.lines() {
        width = width.max(line.len());
        pattern.data.extend(line.as_bytes().iter().copied());
        pattern.height += 1;
    }
    pattern.width = width;
    pattern.buffer = pattern.data.clone();
    pattern
}

fn solve_part1(input: &str) -> usize {
    let mut pattern = parse(input);
    pattern.tilt_north();
    pattern.weight()
}

fn solve_part2(input: &str) -> usize {
    let mut pattern = parse(input);
    pattern.cycle_amount(1_000_000_000)
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::get_input;

    use super::*;

    #[test]
    fn test_cycle() {
        let mut pattern = parse(EXAMPLE);
        pattern.cycle();
        println!("{}", pattern);
        pattern.cycle();
        println!("{}", pattern);
        pattern.cycle();
        println!("{}", pattern);
    }

    #[test]
    fn day14_part1_test() {
        let res = solve_part1(EXAMPLE);
        println!("{res}");
        assert_eq!(res, 136)
    }

    #[test]
    fn day14_part1() -> Result<(), Box<dyn Error>> {
        let input = get_input(2023, 14)?;
        let res = solve_part1(&input);
        println!("day14 Part1 Result: {res}");
        Ok(())
    }

    #[test]
    fn day14_part2_test() {
        let res = solve_part2(EXAMPLE);
        assert_eq!(res, 64);
    }

    #[test]
    fn day14_part2() -> Result<(), Box<dyn Error>> {
        let input = get_input(2023, 14)?;
        let res = solve_part2(&input);
        println!("day14 Part2 Result: {res}");
        Ok(())
    }
}
