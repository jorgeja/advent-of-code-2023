use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Write,
    str::FromStr,
};

const EXAMPLE: &str = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd)]
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

    fn map(&mut self, f: impl Fn(&mut u8)) {
        self.data.iter_mut().for_each(f);
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
                f.write_char((data + b'0') as char)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

type Pos = (usize, usize);
type Dir = (isize, isize);

const UP: Dir = (0, -1);
const DOWN: Dir = (0, 1);
const RIGHT: Dir = (1, 0);
const LEFT: Dir = (-1, 0);
const NONE: Dir = (0, 0);

fn format_dir(dir: Dir) -> u8 {
    match dir {
        UP => b'^' - b'0',
        DOWN => b'v' - b'0',
        RIGHT => b'>' - b'0',
        LEFT => b'<' - b'0',
        _ => 0,
    }
}

fn move_in_dir(pos: Pos, dir: Dir) -> Pos {
    (
        (pos.0 as isize + dir.0) as usize,
        (pos.1 as isize + dir.1) as usize,
    )
}

fn possible(dir: Dir, consecutive: usize) -> [(Dir, usize); 3] {
    match dir {
        UP => [
            (LEFT, 1),
            if consecutive < 3 {
                (UP, consecutive + 1)
            } else {
                (NONE, 0)
            },
            (RIGHT, 1),
        ],
        DOWN => [
            (RIGHT, 1),
            if consecutive < 3 {
                (DOWN, consecutive + 1)
            } else {
                (NONE, 0)
            },
            (LEFT, 1),
        ],
        RIGHT => [
            (UP, 1),
            if consecutive < 3 {
                (RIGHT, consecutive + 1)
            } else {
                (NONE, 0)
            },
            (DOWN, 1),
        ],
        LEFT => [
            (DOWN, 1),
            if consecutive < 3 {
                (LEFT, consecutive + 1)
            } else {
                (NONE, 0)
            },
            (UP, 1),
        ],
        _ => unreachable!(),
    }
}

#[derive(Debug, Clone, PartialEq)]
struct State {
    pos: Pos,
    dir: Dir,
    consecutive: usize,
    heat_loss: usize,
    grid: Grid,
}

impl Eq for State {}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (usize::MAX - self.heat_loss).cmp(&(usize::MAX - other.heat_loss))
    }
}

fn traverse(orig_grid: &Grid, start: Pos, end: Pos) -> u32 {
    let mut min_heat_loss = usize::MAX;
    let mut stack = Vec::new();
    let mut visited = HashMap::new();
    let mut visits = 0;
    let mut cache_hits = 0;
    let mut next_grid = orig_grid.clone();
    next_grid.set(start.0, start.1, b'@' - b'0');
    stack.push(State {
        pos: start,
        dir: RIGHT,
        consecutive: 1,
        heat_loss: 0,
        grid: next_grid.clone(),
    });
    stack.push(State {
        pos: start,
        dir: DOWN,
        consecutive: 1,
        heat_loss: 0,
        grid: next_grid,
    });

    let mut min_grid = orig_grid.clone();
    let mut working_stack = Vec::new();
    while let Some(State {
        pos,
        dir,
        consecutive,
        heat_loss,
        mut grid,
    }) = stack.pop()
    {
        visits += 1;
        if let Some(former_heatloss) = visited.get_mut(&(pos, dir, consecutive)) {
            if *former_heatloss >= heat_loss {
                *former_heatloss = heat_loss;
            } else {
                cache_hits += 1;
                continue;
            }
        } else {
            visited.insert((pos, dir, consecutive), heat_loss);
        }

        if heat_loss > min_heat_loss {
            continue;
        }
        //grid.set(pos.0, pos.1, b'@' - b'0');
        grid.set(pos.0, pos.1, format_dir(dir));

        if pos == end {
            if heat_loss < min_heat_loss {
                min_heat_loss = heat_loss;
                min_grid = grid.clone();
            }
            break;
        }

        working_stack.clear();
        for (next_dir, next_consecutive) in possible(dir, consecutive) {
            let next_pos = move_in_dir(pos, next_dir);
            let current_heat_loss = orig_grid.get(next_pos.0, next_pos.1);
            if next_dir == NONE || current_heat_loss == 0 {
                continue;
            }
            let next_heat_loss = current_heat_loss as usize + heat_loss;
            //print!("{}={} ", (b'0' + format_dir(next_dir)) as char, next_heat_loss);
            working_stack.push(State {
                pos: next_pos,
                dir: next_dir,
                consecutive: next_consecutive,
                heat_loss: next_heat_loss,
                grid: grid.clone(),
            });
        }
        //println!("");
        working_stack.sort();
        stack.extend(working_stack.iter().cloned());
    }

    println!("{min_grid}");
    println!("Cache Hits: {cache_hits}/{visits}");
    min_heat_loss as u32
}

fn solve_part1(input: &str) -> u32 {
    let mut grid = Grid::from_str(input).unwrap();
    grid.map(|v| *v -= b'0');
    println!("{grid}");
    traverse(&grid, (0, 0), (grid.width - 1, grid.height - 1))
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
    fn day17_part1_test() {
        let res = solve_part1(EXAMPLE);
        println!("{res}");
        assert_eq!(res, 102)
    }

    #[test]
    fn day17_part1() -> Result<(), Box<dyn Error>> {
        let input = get_input(2023, 17)?;
        let res = solve_part1(&input);
        println!("day17 Part1 Result: {res}");
        Ok(())
    }

    // #[test]
    // fn day17_part1_recursive() {
    //     let res = solve_part1(&input);
    //     assert_eq!(res, 102);
    // }

    #[test]
    fn day17_part2_test() {
        let res = solve_part2(EXAMPLE);
        assert_eq!(res, 51);
    }

    #[test]
    fn day17_part2() -> Result<(), Box<dyn Error>> {
        let input = get_input(2023, 17)?;
        let res = solve_part2(&input);
        println!("day17 Part2 Result: {res}");
        Ok(())
    }
}
