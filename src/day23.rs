use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Write,
    str::FromStr,
};

const EXAMPLE: &str = r#"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"#;

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

type Dir = (isize, isize);
const UP: Dir = (0, -1);
const DOWN: Dir = (0, 1);
const RIGHT: Dir = (1, 0);
const LEFT: Dir = (-1, 0);
const NONE: Dir = (0, 0);

fn solve_part1(input: &str) -> u32 {
    let grid = Grid::from_str(input).unwrap();
    println!("{grid}");
    let mut stack = Vec::new();
    stack.push(((1, 0), DOWN, 0, ));
    let mut longest_path = 0;
    let end = (grid.width - 2, grid.height -1);
    while let Some((pos, dir, step)) = stack.pop() {
        if pos == end {
            longest_path = longest_path.max(step);
        }

        for next_dir in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            let next_pos = ((pos.0 as isize + next_dir.0) as usize, (pos.1 as isize + next_dir.1) as usize);
            match (dir, next_dir) {
                (UP, DOWN) | (DOWN, UP) | (RIGHT, LEFT) | (LEFT, RIGHT) => continue,
                _ => {}
            }

            let tile = grid.get(next_pos.0, next_pos.1);
            match (tile, next_dir) {
                (b'>', LEFT) | (b'<', RIGHT) | (b'^', DOWN) | (b'v', UP) | (b'#', _) | (0, _) => {
                    continue
                }
                _ => {}
            }

            stack.push((next_pos, next_dir, step + 1));
        }
    }

    longest_path
}

fn solve_part2(input: &str) -> u32 {
    let grid = Grid::from_str(input).unwrap();
    println!("{grid}");
    let mut stack = Vec::new();
    let mut next_poses = Vec::new();
    stack.push(((1, 0), DOWN, 0, HashSet::new()));
    let mut longest_path = 0;
    let end = (grid.width - 2, grid.height -1);
    while let Some((pos, dir, step, mut visited)) = stack.pop() {
        if pos == end {
            longest_path = longest_path.max(step);
            continue
        }

        
        visited.insert(pos);
        next_poses.clear();
        for next_dir in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            let next_pos = ((pos.0 as isize + next_dir.0) as usize, (pos.1 as isize + next_dir.1) as usize);
            match (dir, next_dir) {
                (UP, DOWN) | (DOWN, UP) | (RIGHT, LEFT) | (LEFT, RIGHT) => continue,
                _ => {}
            }

            if visited.contains(&next_pos) {
                continue;
            }

            let tile = grid.get(next_pos.0, next_pos.1);
            match (tile, next_dir) {
                (b'#', _) | (0, _) => {
                    continue
                }
                _ => {}
            }
            next_poses.push((next_pos, next_dir));
        }

        if next_poses.len() == 1 {
            let (next_pos, next_dir) = next_poses.last().unwrap();
            stack.push((*next_pos, *next_dir, step + 1, visited));
        } else {
            for (i, (next_pos, next_dir)) in next_poses.iter().enumerate() {
                if i == next_poses.len() - 1  {
                    stack.push((*next_pos, *next_dir, step + 1, visited));
                    break;
                } else {
                    stack.push((*next_pos, *next_dir, step + 1, visited.clone()));
                } 
            }
        }
    }
    longest_path
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::get_input;

    use super::*;

    #[test]
    fn day23_part1_test() {
        let res = solve_part1(EXAMPLE);
        println!("{res}");
        assert_eq!(res, 94)
    }

    #[test]
    fn day23_part1() -> Result<(), Box<dyn Error>> {
        let input = get_input(2023, 23)?;
        let res = solve_part1(&input);
        println!("day23 Part1 Result: {res}");
        Ok(())
    }

    #[test]
    fn day23_part2_test() {
        let res = solve_part2(EXAMPLE);
        assert_eq!(res, 154);
    }

    #[test]
    fn day23_part2() -> Result<(), Box<dyn Error>> {
        let input = get_input(2023, 23)?;
        let res = solve_part2(&input);
        println!("day23 Part2 Result: {res}");
        Ok(())
    }
}
