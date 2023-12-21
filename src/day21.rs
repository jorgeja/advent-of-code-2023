use std::{
    collections::{HashMap, HashSet, BinaryHeap},
    fmt::Write,
    str::FromStr,
};

const EXAMPLE: &str = r#"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."#;

#[derive(Debug, Default, Clone)]
struct Grid {
    height: usize,
    width: usize,
    data: Vec<u8>,
}

impl Grid {
    fn get(&self, x: usize, y: usize) -> u8 {
        if x > (self.width - 1) || y > (self.height - 1) {
            return 0;
        }

        let i = y * self.width + x;
        self.data[i]
    }

    fn set(&mut self, x: usize, y: usize, val: u8) {
        let i = y * self.width + x;
        if let Some(data) = self.data.get_mut(i) {
            *data = val;
        }
    }

    fn find_start(&self) -> Pos {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get(x, y) == b'S' {
                    return (x, y);
                }
            }
        }
        (0, 0)
    } 
}

impl FromStr for Grid {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut grid = Grid::default();
        let mut width = 0;
        for line in input.lines() {
            width = width.max(line.len());
            grid.data.extend(line.as_bytes().iter().copied());
            grid.height += 1;
        }
        grid.width = width;
        Ok(grid)
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("W: {} H: {}\n", self.width, self.height))?;
        for y in 0..self.height {
            for x in 0..self.width {
                let data = self.get(x, y);
                f.write_char(data as char)?;
            }

            f.write_char('\n')?;
        }
        Ok(())
    }
}

type Pos = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Plot {
    step: usize,
    pos: Pos
}

impl<'a> PartialOrd for Plot {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.step.cmp(&other.step))
    }
}

impl<'a> Ord for Plot {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

fn solve_part1(steps: usize, input: &str) -> u32 {
    let mut grid = Grid::from_str(input).unwrap();
    println!("{grid}");
    let start = grid.find_start();
    let mut stack = BinaryHeap::new();
    
    for (x, y) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
        let next_pos = ((start.0 as i32 + x) as usize, (start.1 as i32 + y) as usize);
        stack.push(Plot { step: steps - 1, pos: next_pos });
    }
    let mut current_step = steps - 1;

    let mut odd_plots = 0;
    let mut even_plots = 1;
    while let Some(Plot { step, pos }) = stack.pop() {
        if step < current_step {
            //println!("{step}: {grid}");
            current_step = step;
        }

        let tile = grid.get(pos.0, pos.1);
        
        //println!("[{step}] {pos:?} : {}", tile as char);

        if tile != b'.' { continue }
        
        if step % 2 == 0 {
            even_plots += 1;
        } else {
            odd_plots += 1;
        }

        grid.set(pos.0, pos.1, b'O');
        if step == 0 { continue }

        for (x, y) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            let next_pos = ((pos.0 as i32 + x) as usize, (pos.1 as i32 + y) as usize);
            stack.push(Plot { step: step - 1, pos: next_pos });
        }
    }

    println!("0: {grid}");
    println!("{odd_plots}, {even_plots}");
    even_plots
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
    fn day21_part1_test() {
        let res = solve_part1(6, EXAMPLE);
        println!("{res}");
        assert_eq!(res, 16)
    }

    #[test]
    fn day21_part1() -> Result<(), Box<dyn Error>> {
        let input = get_input(2023, 21)?;
        let res = solve_part1(64, &input);
        println!("day21 Part1 Result: {res}");
        Ok(())
    }

    // #[test]
    // fn day21_part2_test() {
    //     let res = solve_part2(EXAMPLE);
    //     assert_eq!(res, 51);
    // }

    // #[test]
    // fn day21_part2() -> Result<(), Box<dyn Error>> {
    //     let input = get_input(2023, 21)?;
    //     let res = solve_part2(&input);
    //     println!("day21 Part2 Result: {res}");
    //     Ok(())
    // }
}
