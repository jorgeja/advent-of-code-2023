use std::{
    cmp::Ordering,
    collections::HashMap,
    error::Error,
    fmt::{Debug, Write},
    num,
    slice::SliceIndex,
    str::FromStr,
};
const EXAMPLE: &str = r#"-L|F7
7S-7|
L|7||
-L-J|
L|-JF"#;

const EXAMPLE_2: &str = r#"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"#;

struct Field{
    field: Vec<Vec<u8>>,
    height: usize,
    width: usize,
}

impl Field {
    fn new(field: Vec<Vec<u8>>) -> Self {
        
        let height = field.len();
        let width = field[0].len();
        Self {
            field,
            height,
            width 
        }
    }
    fn index(&self, pos: Pos) -> Option<u8> {
        if pos.0 < 0 || pos.1 < 0 || pos.0 as usize >= self.width || pos.1 as usize >= self.height {
            return None;
        }
        Some(self.field[pos.1 as usize][pos.0 as usize])
    } 
}
impl Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for l in &self.field {
            for c in l {
                f.write_char(*c as char)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

type Pos = (i32, i32);

fn parse(input: &str) -> (Pos, Field) {
    let mut start_pos = None;
    let field = input
        .lines()
        .enumerate()
        .map(|(row, l)| {
            if start_pos.is_none() {
                if let Some(col) = l.find('S') {
                    start_pos = Some((col as i32, row as i32));
                }
            }
            l.as_bytes().iter().copied().collect::<Vec<u8>>()
        })
        .collect();
    (start_pos.unwrap(), Field::new(field))
}

#[derive(Debug,Clone, Copy, PartialEq, Eq)]
enum Dir {
    Left,
    Up,
    Right,
    Down,
}

fn next_pos(pos: Pos, dir: Dir, pipe: u8) -> Option<(Pos, Dir)> {
    let next = match pipe {
        b'|' => match dir {
            Dir::Up => ((pos.0, pos.1 - 1), dir),
            Dir::Down => ((pos.0, pos.1 + 1), dir),
            _ => return None,
        },
        b'-' => match dir {
            Dir::Left => ((pos.0 - 1, pos.1), dir),
            Dir::Right => ((pos.0 + 1, pos.1), dir),
            _ => return None,
        },
        b'L' => match dir {
            Dir::Down => ((pos.0 + 1, pos.1), Dir::Right),
            Dir::Left => ((pos.0, pos.1 - 1), Dir::Up),
            _ => return None,
        },
        b'J' => match dir {
            Dir::Down => ((pos.0 - 1, pos.1), Dir::Left),
            Dir::Right => ((pos.0, pos.1 - 1), Dir::Up),
            _ => return None,
        },
        b'7' => match dir {
            Dir::Up => ((pos.0 - 1, pos.1), Dir::Left),
            Dir::Right => ((pos.0, pos.1 + 1), Dir::Down),
            _ => return None,
        },
        b'F' => match dir {
            Dir::Up => ((pos.0 + 1, pos.1), Dir::Right),
            Dir::Left => ((pos.0, pos.1 + 1), Dir::Down),
            _ => return None,
        },
        _ => return None,
    };
    Some(next)
}

fn solve_part1(input: &str) -> u32 {
    let (start_pos, field) = parse(input);
    println!("Checking loop from {start_pos:?}");
    // try all direction our from start
    let mut valid_loop_length = 0;
    for (d_pos, start_dir) in [((-1, 0), Dir::Left),((1, 0), Dir::Right),((0, -1), Dir::Up),((0, 1), Dir::Down)] {
        let mut pos = (start_pos.0 + d_pos.0, start_pos.1 + d_pos.1);
        if let Some(mut pipe) = field.index(pos) {

            let mut dir = start_dir;
            let mut steps = 1;
            //println!("Checking loop in dir {start_dir:?} at {pos:?} {}", pipe as char);
            while let Some((next_pos, next_dir)) = next_pos(pos, dir, pipe) {
                //steps += 1;
                pos = next_pos;
                dir = next_dir;
                //println!("{} {dir:?} {pos:?}", pipe as char);

                if let Some(next_pipe) = field.index(pos) {
                    steps += 1;
                    
                    pipe = next_pipe;
                    if pipe == b'S' {
                        valid_loop_length = valid_loop_length.max(steps);

                        println!("Found a loop with length {steps} ");
                        break;
                    }
                } else {
                    break;
                }
            }
        }
    }

    valid_loop_length / 2
}

fn solve_part2(input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use std::{error::Error, f64::consts::E};

    use crate::get_input;

    use super::*;

    #[test]
    fn day10_part1_test() {
        let res = solve_part1(EXAMPLE);
        println!("{res}");
        assert_eq!(res, 4);

        let res = solve_part1(EXAMPLE_2);
        println!("{res}");
        assert_eq!(res, 8);
    }

    #[test]
    fn day10_part1() -> Result<(), Box<dyn Error>> {
        let input = get_input(2023, 10)?;
        let res = solve_part1(&input);
        println!("day10 Part1 Result: {res}");
        Ok(())
    }

    // #[test]
    // fn day10_part2_test() {
    //     let res = solve_part2(EXAMPLE_PART2);
    //     assert_eq!(res, 6);
    // }

    // #[test]
    // fn day10_part2() -> Result<(), Box<dyn Error>> {
    //     let input = get_input(2023, 10)?;
    //     let res = solve_part2(&input);
    //     println!("day10 Part2 Result: {res}");
    //     Ok(())
    // }
}
